use eframe::egui;
use eframe::egui_wgpu::wgpu;
use nif::blocks::Block;
use nif::glam::{Mat4, Vec3, Vec4};
use nif::Nif;

use crate::scene::{cull_of, geometry_of, model_matrix};

/// A shape the ray passed through.
pub struct Hit {
    pub block: usize,
    pub distance: f32,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

/// The ray under the cursor. The projection is DirectX style, so near is depth 0.
pub fn ray_through(view_proj: Mat4, rect: egui::Rect, pointer: egui::Pos2) -> Option<Ray> {
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
    let origin = unproject(0.0)?;
    let far = unproject(1.0)?;
    let direction = far - origin;
    (direction.length_squared() > 0.0).then(|| Ray {
        origin,
        direction: direction.normalize(),
    })
}

/// Every drawable shape the ray passes through, nearest first.
///
/// Culling matches what the renderer does, so a back face you cannot see is not pickable.
/// Shapes that blend to nothing are skipped for the same reason.
pub fn hits(nif: &Nif, ray: &Ray) -> Vec<Hit> {
    let mut out = Vec::new();

    for visit in nif.walk() {
        let Some((geometry, data, triangles)) = geometry_of(nif, visit.block) else {
            continue;
        };
        let Some(vertices) = &data.vertices else {
            continue;
        };
        if invisible(nif, &geometry.property_refs) {
            continue;
        }

        // scale 0 collapses the shape to nothing and makes the inverse meaningless
        if visit.transform.scale.abs() < 1e-8 {
            continue;
        }
        let model = model_matrix(&visit.transform);
        let inverse = model.inverse();
        let origin = inverse.transform_point3(ray.origin);
        let direction = inverse.transform_vector3(ray.direction);

        let cull = cull_of(stencil_draw_mode(nif, &geometry.property_refs));
        let mut nearest: Option<f32> = None;
        for triangle in &triangles {
            let (Some(a), Some(b), Some(c)) = (
                vertices.get(triangle.a as usize),
                vertices.get(triangle.b as usize),
                vertices.get(triangle.c as usize),
            ) else {
                continue;
            };
            let (a, b, c) = (vec(a), vec(b), vec(c));
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

fn vec(v: &nif::common::Vector3) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

fn stencil_draw_mode<'a>(
    nif: &'a Nif,
    properties: &[nif::common::BlockRef],
) -> Option<&'a nif::blocks::StencilDrawMode> {
    properties.iter().find_map(|r| match r.get(&nif.blocks) {
        Some(Block::NiStencilProperty(p)) => Some(&p.draw_mode),
        _ => None,
    })
}

/// A blended shape whose material alpha is zero contributes nothing to the image.
fn invisible(nif: &Nif, properties: &[nif::common::BlockRef]) -> bool {
    let blends = properties.iter().any(|r| match r.get(&nif.blocks) {
        Some(Block::NiAlphaProperty(a)) => a.alpha_blend(),
        _ => false,
    });
    blends
        && properties.iter().any(|r| match r.get(&nif.blocks) {
            Some(Block::NiMaterialProperty(m)) => m.alpha == 0.0,
            _ => false,
        })
}

/// Moller-Trumbore. The sign of the determinant is the facing, so culling falls out of it.
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
    use super::*;

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
