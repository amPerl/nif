use std::collections::HashSet;

use eframe::egui;
use eframe::egui_wgpu::wgpu;
use nif::blocks::Block;
use nif::glam::{Mat4, Vec3, Vec4};
use nif::Nif;

use crate::scene::{cull_of, geometry_of, Viewpoint};

/// A shape the ray passed through.
pub struct Hit {
    pub block: usize,
    pub distance: f32,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

/// The ray under the cursor, in the file's own world space. The projection is DirectX style, so
/// near is depth 0.
///
/// `view_proj` draws the scene moved to sit near zero, so unprojecting through it gives a ray in
/// that moved space, while everything this module tests against is where the file puts it.
/// `origin` moves the ray back. Only its start needs it: a translation leaves the direction
/// alone.
pub fn ray_through(
    view_proj: Mat4,
    origin: Vec3,
    rect: egui::Rect,
    pointer: egui::Pos2,
) -> Option<Ray> {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return None;
    }
    let x = 2.0 * (pointer.x - rect.left()) / rect.width() - 1.0;
    let y = 1.0 - 2.0 * (pointer.y - rect.top()) / rect.height();

    let inverse = view_proj.inverse();
    let unproject = |depth: f32| {
        let clip = inverse * Vec4::new(x, y, depth, 1.0);
        (clip.w.abs() > f32::EPSILON).then(|| clip.truncate() / clip.w)
    };
    let unprojected = unproject(0.0)?;
    let far = unproject(1.0)?;
    let direction = far - unprojected;
    let start = origin + unprojected;
    (direction.length_squared() > 0.0).then(|| Ray {
        origin: start,
        direction: direction.normalize(),
    })
}

/// Every drawable shape the ray passes through, nearest first. `visible` is the set of shape
/// blocks currently drawn, so a hidden LOD level cannot be picked, and `viewpoint` is the one
/// the preview is drawing from, so an animated or billboarded shape is picked where it appears
/// rather than where the file stores it.
///
/// Culling matches the renderer, so a back face that is not drawn is not pickable. Shapes that
/// blend to nothing are skipped as well.
pub fn hits(
    nif: &Nif,
    ray: &Ray,
    visible: &HashSet<usize>,
    viewpoint: Viewpoint,
    frame: &crate::scene::Frame,
    quad_axes: (Vec3, Vec3),
) -> Vec<Hit> {
    let mut out = Vec::new();

    for visit in viewpoint.walk(nif) {
        // A particle system stores no geometry, so the ray meets the particles the frame
        // simulated rather than anything in the file. Each is treated as a sphere of its own
        // radius, which is what a camera facing quad covers from any angle.
        if let Block::NiParticleSystem(_) = visit.block {
            // a hidden LOD level is not on screen, so it is not selectable either. The set
            // covers particle systems as well as shapes, which is what makes this the same
            // check rather than a second one.
            if !visible.contains(&visit.index) {
                continue;
            }
            let Some(particles) = frame.particles.get(&visit.index) else {
                continue;
            };
            let model = Mat4::from(&visit.transform);
            // taken from the matrix, the same way the renderer sizes a quad
            let scale = model.x_axis.truncate().length();
            let nearest = particles
                .iter()
                .filter_map(|particle| {
                    let centre = model.transform_point3(Vec3::from(&particle.position));
                    let half = particle.drawn_radius().max(0.0) * scale;
                    quad_hit(ray, centre, half, particle.rotation, quad_axes)
                })
                .fold(f32::MAX, f32::min);
            if nearest < f32::MAX {
                out.push(Hit {
                    block: visit.index,
                    distance: nearest,
                });
            }
            continue;
        }
        if !visible.contains(&visit.index) {
            continue;
        }
        let Some((_geometry, data, triangles)) = geometry_of(nif, visit.block) else {
            continue;
        };
        // A morph replaces the stored vertices, and the renderer draws the replacement, so the
        // ray has to meet the shape where it has been carried to. Reading `data` here instead
        // leaves a morphing shape clickable at rest and nowhere near where it is drawn.
        let Some(vertices) = frame
            .morph
            .get(&visit.index)
            .map(|moved| &moved.positions)
            .or(data.vertices.as_ref())
        else {
            continue;
        };
        // the properties in force, a parent node's included, so what can be clicked matches
        // what is drawn
        if invisible(nif, visit.properties, frame) {
            continue;
        }

        // a zero scale makes the inverse matrix meaningless
        if visit.transform.scale.abs() < 1e-8 {
            continue;
        }
        let model = Mat4::from(&visit.transform);
        let inverse = model.inverse();
        let origin = inverse.transform_point3(ray.origin);
        let direction = inverse.transform_vector3(ray.direction);

        let cull = cull_of(stencil_draw_mode(nif, visit.properties));
        let mut nearest: Option<f32> = None;
        for triangle in &triangles {
            let (Some(a), Some(b), Some(c)) = (
                vertices.get(triangle.a as usize),
                vertices.get(triangle.b as usize),
                vertices.get(triangle.c as usize),
            ) else {
                continue;
            };
            let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
            let Some(local) = intersect(origin, direction, a, b, c, cull) else {
                continue;
            };
            // measure in world space, since the transform carries a scale
            let world = model.transform_point3(origin + direction * local);
            let distance = world.distance(ray.origin);
            if nearest.is_none_or(|n| distance < n) {
                nearest = Some(distance);
            }
        }
        if let Some(distance) = nearest {
            out.push(Hit {
                block: visit.index,
                distance,
            });
        }
    }

    out.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    out
}

fn stencil_draw_mode(
    nif: &Nif,
    properties: nif::walk::Properties,
) -> Option<&nif::blocks::StencilDrawMode> {
    match properties.stencil.get(&nif.blocks) {
        Some(Block::NiStencilProperty(p)) => Some(&p.draw_mode),
        _ => None,
    }
}

/// A blended shape whose material alpha is zero contributes nothing to the image.
///
/// The alpha is the one the frame is drawing with, not the one the file stores, because a
/// controller replaces it. Reading the stored value leaves a shape that has faded out still
/// clickable, and picks up one that has faded in as if it were not there.
fn invisible(nif: &Nif, properties: nif::walk::Properties, frame: &crate::scene::Frame) -> bool {
    let blends = matches!(
        properties.alpha.get(&nif.blocks),
        Some(Block::NiAlphaProperty(a)) if a.alpha_blend()
    );
    let Some(Block::NiMaterialProperty(material)) = properties.material.get(&nif.blocks) else {
        return false;
    };
    let alpha = properties
        .material
        .index()
        .and_then(|block| frame.alpha.get(&block))
        .copied()
        .unwrap_or(material.alpha);
    blends && alpha == 0.0
}

/// The quad a particle draws as, built the way the renderer builds it: the camera's own axes,
/// turned by the particle's own rotation, at its drawn radius. Testing the square rather than a
/// sphere around it is what makes a sprite's corners clickable and its gaps not.
fn quad_hit(ray: &Ray, centre: Vec3, half: f32, rotation: f32, axes: (Vec3, Vec3)) -> Option<f32> {
    if half <= 0.0 {
        return None;
    }
    let (right, up) = axes;
    let (sin, cos) = rotation.sin_cos();
    let corner = |x: f32, y: f32| {
        let (x, y) = (x * cos - y * sin, x * sin + y * cos);
        centre + right * (x * half) + up * (y * half)
    };
    let (a, b, c, d) = (
        corner(-1.0, -1.0),
        corner(1.0, -1.0),
        corner(1.0, 1.0),
        corner(-1.0, 1.0),
    );
    // a quad faces the camera, so neither triangle is culled: it can be met from either side
    let first = intersect(ray.origin, ray.direction, a, b, c, None);
    let second = intersect(ray.origin, ray.direction, a, c, d, None);
    match (first, second) {
        (Some(one), Some(two)) => Some(one.min(two)),
        (hit, None) | (None, hit) => hit,
    }
}

/// Moller-Trumbore. The sign of the determinant gives the facing, which drives culling.
fn intersect(
    origin: Vec3,
    direction: Vec3,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    cull: Option<wgpu::Face>,
) -> Option<f32> {
    const EPSILON: f32 = 1e-7;

    let edge1 = b - a;
    let edge2 = c - a;
    let pvec = direction.cross(edge2);
    let det = edge1.dot(pvec);

    let facing = match cull {
        Some(wgpu::Face::Back) => det > EPSILON,
        Some(wgpu::Face::Front) => det < -EPSILON,
        None => det.abs() > EPSILON,
    };
    if !facing {
        return None;
    }

    let inv_det = 1.0 / det;
    let tvec = origin - a;
    let u = tvec.dot(pvec) * inv_det;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let qvec = tvec.cross(edge1);
    let v = direction.dot(qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = edge2.dot(qvec) * inv_det;
    (distance > EPSILON).then_some(distance)
}

#[cfg(test)]
mod tests {

    /// The scene is drawn moved to sit near zero and picked where the file puts it, so the ray
    /// has to cross back. Getting this wrong makes every click miss on a model far from the
    /// origin while working on one near it.
    #[test]
    fn a_ray_comes_back_into_the_space_the_file_uses() {
        let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100.0, 100.0));
        let centre = egui::pos2(50.0, 50.0);
        let eye = Vec3::new(0.0, -4.0, 0.0);
        let view = nif::glam::camera::rh::view::look_at_mat4(eye, Vec3::ZERO, Vec3::Z);
        let projection = nif::glam::camera::rh::proj::directx::perspective(1.0, 1.0, 0.1, 100.0);
        let view_proj = projection * view;

        // drawn at zero and picked at zero: the ray starts near the eye
        let here = super::ray_through(view_proj, Vec3::ZERO, rect, centre).expect("a ray");
        assert!(here.origin.distance(eye) < 0.5, "{:?}", here.origin);

        // drawn at zero and picked nine thousand units out: the ray starts near the eye there
        let far = Vec3::new(9000.0, 0.0, 0.0);
        let moved = super::ray_through(view_proj, far, rect, centre).expect("a ray");
        assert!(moved.origin.distance(eye + far) < 0.5, "{:?}", moved.origin);

        // and the direction is untouched, since a translation cannot turn it
        assert!(moved.direction.abs_diff_eq(here.direction, 1e-6));
    }
    use super::*;

    fn ray(origin: Vec3, at: Vec3) -> Ray {
        Ray {
            origin,
            direction: (at - origin).normalize(),
        }
    }

    /// A quad spanned by x and y, which is what the camera's own axes come to when it looks
    /// down z.
    const AXES: (Vec3, Vec3) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    #[test]
    fn a_particle_is_hit_where_its_quad_is() {
        let centre = Vec3::new(0.0, 0.0, 10.0);
        let hit = quad_hit(&ray(Vec3::ZERO, centre), centre, 2.0, 0.0, AXES);
        // the quad is flat and faces the camera, so it is met at its own depth
        assert!(hit.is_some_and(|d| (d - 10.0).abs() < 1e-4), "got {hit:?}");
    }

    #[test]
    fn a_ray_that_passes_beside_a_particle_misses() {
        let centre = Vec3::new(0.0, 0.0, 10.0);
        let beside = ray(Vec3::ZERO, Vec3::new(5.0, 0.0, 10.0));
        assert!(quad_hit(&beside, centre, 1.0, 0.0, AXES).is_none());
    }

    #[test]
    fn a_particle_behind_the_camera_is_not_hit() {
        let behind = Vec3::new(0.0, 0.0, -10.0);
        let forward = ray(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        assert!(quad_hit(&forward, behind, 2.0, 0.0, AXES).is_none());
    }

    /// The reason this is a quad and not a sphere: a corner sits at the half width times root
    /// two, so a sphere of the half width does not reach it and the sprite's corners were not
    /// clickable. The same test the other way round is what stops the quad over-reaching.
    #[test]
    fn a_sprites_corner_is_clickable_and_the_space_past_it_is_not() {
        let centre = Vec3::new(0.0, 0.0, 10.0);
        let corner = Vec3::new(0.99, 0.99, 10.0);
        assert!(quad_hit(&ray(Vec3::ZERO, corner), centre, 1.0, 0.0, AXES).is_some());

        // just outside the same corner, which no sphere or square should catch
        let past = Vec3::new(1.02, 1.02, 10.0);
        assert!(quad_hit(&ray(Vec3::ZERO, past), centre, 1.0, 0.0, AXES).is_none());
    }

    /// A spinning sprite is picked where it now is. Turning the quad an eighth of a turn puts a
    /// corner where an edge was, so a point beyond the edge comes inside it.
    #[test]
    fn a_turned_quad_is_picked_turned() {
        let centre = Vec3::new(0.0, 0.0, 10.0);
        let beyond = Vec3::new(1.3, 0.0, 10.0);
        let eighth = std::f32::consts::FRAC_PI_4;

        assert!(quad_hit(&ray(Vec3::ZERO, beyond), centre, 1.0, 0.0, AXES).is_none());
        assert!(quad_hit(&ray(Vec3::ZERO, beyond), centre, 1.0, eighth, AXES).is_some());
    }

    // counter-clockwise seen from +Z, which is the NIF front face
    const A: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const B: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    const C: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    fn down(origin: Vec3, cull: Option<wgpu::Face>) -> Option<f32> {
        intersect(origin, Vec3::new(0.0, 0.0, -1.0), A, B, C, cull)
    }

    #[test]
    fn hits_the_front_face() {
        let distance = down(Vec3::new(0.2, 0.2, 5.0), Some(wgpu::Face::Back));
        assert_eq!(distance, Some(5.0));
    }

    #[test]
    fn misses_outside_the_triangle() {
        assert!(down(Vec3::new(0.9, 0.9, 5.0), Some(wgpu::Face::Back)).is_none());
    }

    #[test]
    fn back_faces_are_culled_but_two_sided_shapes_are_not() {
        let from_behind = Vec3::new(0.2, 0.2, -5.0);
        let up = Vec3::new(0.0, 0.0, 1.0);
        assert!(intersect(from_behind, up, A, B, C, Some(wgpu::Face::Back)).is_none());
        assert_eq!(intersect(from_behind, up, A, B, C, None), Some(5.0));
        assert_eq!(
            intersect(from_behind, up, A, B, C, Some(wgpu::Face::Front)),
            Some(5.0)
        );
    }

    #[test]
    fn geometry_behind_the_ray_does_not_count() {
        assert!(down(Vec3::new(0.2, 0.2, -5.0), None).is_none());
    }

    #[test]
    fn a_ray_in_the_triangle_plane_misses() {
        let along = intersect(
            Vec3::new(-1.0, 0.2, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            A,
            B,
            C,
            None,
        );
        assert!(along.is_none());
    }
}
