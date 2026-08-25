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

/// Where a ray first meets a sphere, or None when it misses or the sphere is behind it.
fn sphere_hit(ray: &Ray, centre: Vec3, radius: f32) -> Option<f32> {
    if radius <= 0.0 {
        return None;
    }
    let to_centre = centre - ray.origin;
    let along = to_centre.dot(ray.direction);
    let closest = to_centre - ray.direction * along;
    let gap = radius * radius - closest.length_squared();
    if gap < 0.0 {
        return None;
    }
    let half = gap.sqrt();
    // the near intersection, unless the ray starts inside, where the far one is what it meets
    let near = along - half;
    let distance = if near >= 0.0 { near } else { along + half };
    (distance >= 0.0).then_some(distance)
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
) -> Vec<Hit> {
    let mut out = Vec::new();

    for visit in viewpoint.walk(nif) {
        // A particle system stores no geometry, so the ray meets the particles the frame
        // simulated rather than anything in the file. Each is treated as a sphere of its own
        // radius, which is what a camera facing quad covers from any angle.
        if let Block::NiParticleSystem(_) = visit.block {
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
                    sphere_hit(ray, centre, particle.drawn_radius().max(0.0) * scale)
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
        let Some(vertices) = &data.vertices else {
            continue;
        };
        // the properties in force, a parent node's included, so what can be clicked matches
        // what is drawn
        if invisible(nif, visit.properties) {
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
fn invisible(nif: &Nif, properties: nif::walk::Properties) -> bool {
    let blends = matches!(
        properties.alpha.get(&nif.blocks),
        Some(Block::NiAlphaProperty(a)) if a.alpha_blend()
    );
    let clear = matches!(
        properties.material.get(&nif.blocks),
        Some(Block::NiMaterialProperty(m)) if m.alpha == 0.0
    );
    blends && clear
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

    #[test]
    fn a_particle_is_hit_at_its_near_side() {
        let centre = Vec3::new(0.0, 0.0, 10.0);
        let hit = sphere_hit(&ray(Vec3::ZERO, centre), centre, 2.0);
        // the near surface, not the centre, so a nearer particle wins the sort
        assert!(hit.is_some_and(|d| (d - 8.0).abs() < 1e-4), "got {hit:?}");
    }

    #[test]
    fn a_ray_that_passes_beside_a_particle_misses() {
        let centre = Vec3::new(0.0, 0.0, 10.0);
        let beside = ray(Vec3::ZERO, Vec3::new(5.0, 0.0, 10.0));
        assert!(sphere_hit(&beside, centre, 1.0).is_none());
    }

    #[test]
    fn a_particle_behind_the_camera_is_not_hit() {
        let behind = Vec3::new(0.0, 0.0, -10.0);
        let forward = ray(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        assert!(sphere_hit(&forward, behind, 2.0).is_none());
    }

    #[test]
    fn a_ray_starting_inside_a_particle_leaves_through_the_far_side() {
        let forward = ray(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere_hit(&forward, Vec3::ZERO, 3.0);
        assert!(hit.is_some_and(|d| (d - 3.0).abs() < 1e-4), "got {hit:?}");
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
