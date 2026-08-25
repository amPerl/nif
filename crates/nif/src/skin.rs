use glam::{Mat3, Mat4, Vec3};

use crate::blocks::{Block, NiGeometry};
use crate::common::Vector3;

/// Where a skinned shape's geometry actually sits, in the file's own world space.
///
/// A skinned shape stores its vertices in skin space and its own node transform does not place
/// them: the bones do. So these are world positions already and the shape draws with no model
/// transform of its own, unlike a morph, which replaces local vertices and still needs one.
pub struct Skinned {
    pub positions: Vec<Vector3>,
    /// Turned by the same matrices as the positions. `None` where the shape stores none.
    pub normals: Option<Vec<Vector3>>,
}

/// A skinned shape deformed by its bones, or `None` where nothing skins it.
///
/// `bone_world` supplies each bone's world transform by block index, which only a traversal
/// knows, so the caller passes in the walk it already did rather than this doing another.
/// A bone the lookup cannot place contributes nothing.
///
/// Each bone's matrix is its world transform times the offset the skin data stores for it,
/// and every vertex is the weighted sum of its bones' matrices applied to it. The shape's own
/// world transform takes no part; using it is how a skinned mesh ends up drawn in the wrong
/// place rather than merely unanimated. The other transform the skin data carries, the one on
/// `NiSkinData` itself, belongs to the bound and not to this.
pub fn deform<F>(blocks: &[Block], geometry: &NiGeometry, bone_world: F) -> Option<Skinned>
where
    F: Fn(usize) -> Option<Mat4>,
{
    let Block::NiSkinInstance(skin) = geometry.skin_instance_ref.get(blocks)? else {
        return None;
    };
    let Block::NiSkinData(data) = skin.data_ref.get(blocks)? else {
        return None;
    };
    let stored = geometry.data_ref.get(blocks)?.geometry_data()?;
    let vertices = stored.vertices.as_ref()?;
    if vertices.is_empty() {
        return None;
    }

    let normals = stored
        .normals
        .as_ref()
        .filter(|normals| normals.len() == vertices.len());
    let mut positions = vec![Vec3::ZERO; vertices.len()];
    let mut turned = normals.map(|_| vec![Vec3::ZERO; vertices.len()]);

    for (bone, reference) in data.bone_list.iter().zip(skin.bone_refs.iter()) {
        let Some(world) = reference.index().and_then(&bone_world) else {
            continue;
        };
        let matrix = world * Mat4::from(&bone.skin_transform);
        let rotation = Mat3::from_mat4(matrix);
        for weight in bone.vertex_weights.iter().flat_map(|weights| weights.iter()) {
            let at = weight.index as usize;
            let Some(vertex) = vertices.get(at) else {
                continue;
            };
            let Some(slot) = positions.get_mut(at) else {
                continue;
            };
            *slot += matrix.transform_point3(Vec3::from(vertex)) * weight.weight;

            if let (Some(turned), Some(normals)) = (turned.as_mut(), normals) {
                if let (Some(slot), Some(normal)) = (turned.get_mut(at), normals.get(at)) {
                    *slot += rotation * Vec3::from(normal) * weight.weight;
                }
            }
        }
    }

    Some(Skinned {
        positions: positions.into_iter().map(vector).collect(),
        // a bone can carry a scale, so the sum is renormalised rather than trusted
        normals: turned.map(|turned| {
            turned
                .into_iter()
                .map(|normal| vector(normal.normalize_or_zero()))
                .collect()
        }),
    })
}

/// Whether a shape is skinned at all, which decides whether its own transform places it.
pub fn is_skinned(blocks: &[Block], geometry: &NiGeometry) -> bool {
    matches!(
        geometry.skin_instance_ref.get(blocks),
        Some(Block::NiSkinInstance(_))
    )
}

fn vector(at: Vec3) -> Vector3 {
    Vector3 {
        x: at.x,
        y: at.y,
        z: at.z,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{BoneData, BoneVertData, NiBound, NiSkinData, NiSkinInstance};
    use crate::common::{BlockRef, Matrix33, NiTransform};

    fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn transform(translation: Vector3) -> NiTransform {
        NiTransform {
            translation,
            rotation: Matrix33::IDENTITY,
            scale: 1.0,
        }
    }

    fn bone(offset: Vector3, weights: Vec<(u16, f32)>) -> BoneData {
        BoneData {
            skin_transform: transform(offset),
            bounding_sphere: NiBound {
                center: vec3(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            num_vertices: weights.len() as u16,
            vertex_weights: Some(
                weights
                    .into_iter()
                    .map(|(index, weight)| BoneVertData { index, weight })
                    .collect(),
            ),
        }
    }

    fn skinned_shape(bones: Vec<BoneData>, bone_refs: Vec<BlockRef>) -> Vec<Block> {
        let vertices = vec![vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0)];
        let normals = vec![vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0)];
        vec![
            crate::anim::tests::shape_with_skin(BlockRef::Index(1), BlockRef::Index(3)),
            Block::NiSkinInstance(NiSkinInstance {
                data_ref: BlockRef::Index(2),
                skin_partition: -1,
                skeleton_root: 0,
                bone_refs,
            }),
            Block::NiSkinData(NiSkinData {
                skin_transform: transform(vec3(0.0, 0.0, 0.0)),
                has_vertex_weights: 1,
                bone_list: bones,
            }),
            crate::anim::tests::shape_data(vertices, Some(normals), Vec::new()),
        ]
    }

    /// A vertex owned outright by one bone lands where that bone's world transform and the
    /// offset the skin data stores for it put it. The shape's own transform takes no part.
    #[test]
    fn a_vertex_follows_the_bone_that_owns_it() {
        let blocks = skinned_shape(
            vec![bone(vec3(0.0, 0.0, 5.0), vec![(0, 1.0), (1, 1.0)])],
            vec![BlockRef::Index(4)],
        );
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        let moved = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let skinned = deform(&blocks, geometry, |index| (index == 4).then_some(moved))
            .expect("the shape is skinned");

        // vertex 0 is at the origin in skin space: offset by z 5, then the bone's own move
        assert_eq!(Vec3::from(&skinned.positions[0]), Vec3::new(10.0, 0.0, 5.0));
        assert_eq!(Vec3::from(&skinned.positions[1]), Vec3::new(11.0, 0.0, 5.0));
    }

    /// Two bones at half weight each put a vertex midway between where either alone would.
    #[test]
    fn weights_blend_the_bones_that_share_a_vertex() {
        let blocks = skinned_shape(
            vec![
                bone(vec3(0.0, 0.0, 0.0), vec![(0, 0.5)]),
                bone(vec3(0.0, 0.0, 0.0), vec![(0, 0.5)]),
            ],
            vec![BlockRef::Index(4), BlockRef::Index(5)],
        );
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        let skinned = deform(&blocks, geometry, |index| match index {
            4 => Some(Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0))),
            5 => Some(Mat4::from_translation(Vec3::new(8.0, 0.0, 0.0))),
            _ => None,
        })
        .expect("the shape is skinned");

        assert_eq!(Vec3::from(&skinned.positions[0]), Vec3::new(4.0, 0.0, 0.0));
    }

    /// Normals turn with their bones and come back unit length, so a bone carrying a scale
    /// does not brighten or darken what it moves.
    #[test]
    fn a_normal_turns_with_its_bone_and_stays_unit_length() {
        let blocks = skinned_shape(
            vec![bone(vec3(0.0, 0.0, 0.0), vec![(0, 1.0), (1, 1.0)])],
            vec![BlockRef::Index(4)],
        );
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        // a quarter turn about x takes +z to -y, and a scale that would stretch the normal
        let turn = Mat4::from_scale_rotation_translation(
            Vec3::splat(4.0),
            glam::Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            Vec3::ZERO,
        );
        let skinned =
            deform(&blocks, geometry, |_| Some(turn)).expect("the shape is skinned");
        let normals = skinned.normals.expect("the shape stores normals");

        let turned = Vec3::from(&normals[0]);
        assert!((turned - Vec3::new(0.0, -1.0, 0.0)).length() < 1e-5, "{turned:?}");
        assert!((turned.length() - 1.0).abs() < 1e-5);
    }

    /// A bone the caller cannot place contributes nothing rather than dragging what it owns to
    /// the origin, since a walk that does not reach a bone is a file problem and not a pose.
    #[test]
    fn an_unplaceable_bone_is_left_out() {
        let blocks = skinned_shape(
            vec![
                bone(vec3(0.0, 0.0, 0.0), vec![(0, 0.5)]),
                bone(vec3(0.0, 0.0, 0.0), vec![(0, 0.5)]),
            ],
            vec![BlockRef::Index(4), BlockRef::Index(5)],
        );
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        let skinned = deform(&blocks, geometry, |index| {
            (index == 4).then_some(Mat4::from_translation(Vec3::new(6.0, 0.0, 0.0)))
        })
        .expect("the shape is skinned");

        // only the half weight that could be placed, rather than a collapse toward zero
        assert_eq!(Vec3::from(&skinned.positions[0]), Vec3::new(3.0, 0.0, 0.0));
    }
}
