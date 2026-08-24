use super::ni_object_net::NiObjectNET;
use crate::common::{BlockRef, Matrix22, TexCoord};

use binrw::{BinRead, BinWrite};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTexturingProperty {
    pub base: NiObjectNET,
    pub apply_mode: ApplyMode,
    pub texture_count: u32,

    #[br(temp)]
    #[bw(calc = u8::from(base_texture.is_some()))]
    has_base_texture: u8,
    #[br(if(has_base_texture != 0))]
    pub base_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(dark_texture.is_some()))]
    has_dark_texture: u8,
    #[br(if(has_dark_texture != 0))]
    pub dark_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(detail_texture.is_some()))]
    has_detail_texture: u8,
    #[br(if(has_detail_texture != 0))]
    pub detail_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(gloss_texture.is_some()))]
    has_gloss_texture: u8,
    #[br(if(has_gloss_texture != 0))]
    pub gloss_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = u8::from(glow_texture.is_some()))]
    has_glow_texture: u8,
    #[br(if(has_glow_texture != 0))]
    pub glow_texture: Option<Box<TexDesc>>,

    #[br(if(texture_count > 5))]
    pub bump_map: Option<BumpMap>,

    #[br(temp, if(texture_count > 6))]
    #[bw(if(*texture_count > 6), calc = u8::from(decal0_texture.is_some()))]
    has_decal0_texture: u8,
    #[br(if(has_decal0_texture != 0))]
    pub decal0_texture: Option<Box<TexDesc>>,

    #[br(temp, if(texture_count > 7))]
    #[bw(if(*texture_count > 7), calc = u8::from(decal1_texture.is_some()))]
    has_decal1_texture: u8,
    #[br(if(has_decal1_texture != 0))]
    pub decal1_texture: Option<Box<TexDesc>>,

    #[br(temp, if(texture_count > 8))]
    #[bw(if(*texture_count > 8), calc = u8::from(decal2_texture.is_some()))]
    has_decal2_texture: u8,
    #[br(if(has_decal2_texture != 0))]
    pub decal2_texture: Option<Box<TexDesc>>,

    #[br(temp, if(texture_count > 9))]
    #[bw(if(*texture_count > 9), calc = u8::from(decal3_texture.is_some()))]
    has_decal3_texture: u8,
    #[br(if(has_decal3_texture != 0))]
    pub decal3_texture: Option<Box<TexDesc>>,

    #[br(temp)]
    #[bw(calc = shader_textures.len() as u32)]
    num_shader_textures: u32,
    #[br(count=num_shader_textures)]
    pub shader_textures: Vec<ShaderTexDesc>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum TextureSlot {
    Base,
    Dark,
    Detail,
    Gloss,
    Glow,
    BumpMap,
    Decal(u8),
    Shader(u32),
}

impl TextureSlot {
    /// The slot a controller names by index. `NiTextureTransformController` and
    /// `NiFlipController` both address a map this way, and the order is the order the maps are
    /// stored in.
    pub fn from_index(index: u32) -> Option<TextureSlot> {
        Some(match index {
            0 => TextureSlot::Base,
            1 => TextureSlot::Dark,
            2 => TextureSlot::Detail,
            3 => TextureSlot::Gloss,
            4 => TextureSlot::Glow,
            5 => TextureSlot::BumpMap,
            6..=9 => TextureSlot::Decal((index - 6) as u8),
            _ => return None,
        })
    }
}

impl std::fmt::Display for TextureSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureSlot::Base => f.write_str("base"),
            TextureSlot::Dark => f.write_str("dark"),
            TextureSlot::Detail => f.write_str("detail"),
            TextureSlot::Gloss => f.write_str("gloss"),
            TextureSlot::Glow => f.write_str("glow"),
            TextureSlot::BumpMap => f.write_str("bump map"),
            TextureSlot::Decal(i) => write!(f, "decal {i}"),
            TextureSlot::Shader(id) => write!(f, "shader {id}"),
        }
    }
}

impl NiTexturingProperty {
    /// The occupied texture slots in read order, named.
    pub fn textures(&self) -> impl Iterator<Item = (TextureSlot, &TexDesc)> + '_ {
        [
            (TextureSlot::Base, self.base_texture.as_deref()),
            (TextureSlot::Dark, self.dark_texture.as_deref()),
            (TextureSlot::Detail, self.detail_texture.as_deref()),
            (TextureSlot::Gloss, self.gloss_texture.as_deref()),
            (TextureSlot::Glow, self.glow_texture.as_deref()),
            (
                TextureSlot::BumpMap,
                self.bump_map
                    .as_ref()
                    .and_then(BumpMap::get)
                    .map(|d| d.texture.as_ref()),
            ),
            (TextureSlot::Decal(0), self.decal0_texture.as_deref()),
            (TextureSlot::Decal(1), self.decal1_texture.as_deref()),
            (TextureSlot::Decal(2), self.decal2_texture.as_deref()),
            (TextureSlot::Decal(3), self.decal3_texture.as_deref()),
        ]
        .into_iter()
        .filter_map(|(slot, desc)| Some((slot, desc?)))
        .chain(self.shader_textures.iter().filter_map(|entry| {
            let map = entry.get()?;
            Some((TextureSlot::Shader(map.map_id), &map.map))
        }))
    }

    /// The map in one slot, or None when the slot is empty.
    pub fn texture(&self, slot: TextureSlot) -> Option<&TexDesc> {
        self.textures()
            .find_map(|(found, desc)| (found == slot).then_some(desc))
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BumpMapData {
    pub texture: Box<TexDesc>,
    pub luma_scale: f32,
    pub luma_offset: f32,
    pub matrix: Matrix22,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum BumpMap {
    #[brw(magic = 0u8)]
    None,
    #[brw(magic = 1u8)]
    Present(BumpMapData),
    Invalid {
        flag: u8,
        data: BumpMapData,
    },
}

impl BumpMap {
    pub fn get(&self) -> Option<&BumpMapData> {
        match self {
            BumpMap::None => Option::None,
            BumpMap::Present(d) => Some(d),
            BumpMap::Invalid { data, .. } => Some(data),
        }
    }
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TexDesc {
    pub source_ref: BlockRef,
    pub clamp_mode: TexClampMode,
    pub filter_mode: TexFilterMode,
    pub uv_set: u32,
    pub transform: TexTransform,
}

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TextureTransform {
    pub translation: TexCoord,
    pub tiling: TexCoord,
    pub w_rotation: f32,
    pub transform_type: u32,
    pub center_offset: TexCoord,
}

impl TextureTransform {
    pub const MAYA_DEPRECATED: u32 = 0;
    pub const MAX: u32 = 1;
    pub const MAYA: u32 = 2;

    /// What the engine substitutes when a controller drives a map that carries no transform of
    /// its own.
    pub fn identity() -> TextureTransform {
        TextureTransform {
            translation: TexCoord { u: 0.0, v: 0.0 },
            tiling: TexCoord { u: 1.0, v: 1.0 },
            w_rotation: 0.0,
            transform_type: TextureTransform::MAYA,
            center_offset: TexCoord { u: 0.5, v: 0.5 },
        }
    }

    /// The 3x3 that takes a uv through this transform, as `matrix * (u, v, 1)`.
    ///
    /// The engine ships a closed form per method with the composition it came from commented
    /// out beside it. This builds the composition, because it says what the
    /// transform means rather than what it evaluates to, but the two disagree on the sign of
    /// the rotation for MAX and MAYA. The comments are scaffolding the engine says can be
    /// removed, so the shipped form decides and a test holds them together.
    #[cfg(feature = "glam")]
    pub fn matrix(&self) -> glam::Mat3 {
        use glam::{Mat3, Vec2};

        let centre = Vec2::new(self.center_offset.u, self.center_offset.v);
        let to_centre = Mat3::from_translation(centre);
        let from_centre = Mat3::from_translation(-centre);
        let scale = Mat3::from_scale(Vec2::new(self.tiling.u, self.tiling.v));
        let translation = Vec2::new(self.translation.u, self.translation.v);

        match self.transform_type {
            TextureTransform::MAX => {
                let translate = Mat3::from_translation(Vec2::new(-translation.x, translation.y));
                let rotate = Mat3::from_angle(-self.w_rotation);
                to_centre * scale * rotate * translate * from_centre
            }
            TextureTransform::MAYA_DEPRECATED => {
                let rotate = Mat3::from_angle(self.w_rotation);
                to_centre * rotate * from_centre * Mat3::from_translation(translation) * scale
            }
            _ => {
                let rotate = Mat3::from_angle(-self.w_rotation);
                // maya measures v from the opposite edge
                let from_maya =
                    Mat3::from_translation(Vec2::Y) * Mat3::from_scale(Vec2::new(1.0, -1.0));
                to_centre
                    * rotate
                    * from_centre
                    * from_maya
                    * Mat3::from_translation(translation)
                    * scale
            }
        }
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum TexTransform {
    #[brw(magic = 0u8)]
    None,
    #[brw(magic = 1u8)]
    Present(TextureTransform),
    Invalid {
        flag: u8,
        transform: TextureTransform,
    },
}

impl TexTransform {
    pub fn get(&self) -> Option<&TextureTransform> {
        match self {
            TexTransform::None => Option::None,
            TexTransform::Present(t) => Some(t),
            TexTransform::Invalid { transform, .. } => Some(transform),
        }
    }
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ShaderMap {
    pub map: TexDesc,
    pub map_id: u32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum ShaderTexDesc {
    #[brw(magic = 0u8)]
    None,
    #[brw(magic = 1u8)]
    Map(ShaderMap),
    Invalid {
        flag: u8,
        map: ShaderMap,
    },
}

impl ShaderTexDesc {
    pub fn get(&self) -> Option<&ShaderMap> {
        match self {
            ShaderTexDesc::None => Option::None,
            ShaderTexDesc::Map(m) => Some(m),
            ShaderTexDesc::Invalid { map, .. } => Some(map),
        }
    }
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum ApplyMode {
    #[brw(magic = 0u32)]
    Replace,
    #[brw(magic = 1u32)]
    Decal,
    #[brw(magic = 2u32)]
    Modulate,
    #[brw(magic = 3u32)]
    Hilight,
    #[brw(magic = 4u32)]
    Hilight2,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum TexClampMode {
    #[brw(magic = 0u32)]
    ClampSClampT,
    #[brw(magic = 1u32)]
    ClampSWrapT,
    #[brw(magic = 2u32)]
    WrapSClampT,
    #[brw(magic = 3u32)]
    WrapSWrapT,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum TexFilterMode {
    #[brw(magic = 0u32)]
    Nearest,
    #[brw(magic = 1u32)]
    Bilerp,
    #[brw(magic = 2u32)]
    Trilerp,
    #[brw(magic = 3u32)]
    NearestMipNearest,
    #[brw(magic = 4u32)]
    NearestMipLerp,
    #[brw(magic = 5u32)]
    BilerpMipNearest,
    Unknown(u32),
}

impl std::ops::Deref for NiTexturingProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[cfg(all(test, feature = "glam"))]
mod tests {
    use super::*;

    /// The closed form the engine actually ships, as columns.
    /// The composition above is derived from the comments beside it, so this is what proves the
    /// derivation: if the two disagree, the readable version is wrong.
    fn shipped(t: &TextureTransform) -> glam::Mat3 {
        let (sin, cos) = t.w_rotation.sin_cos();
        let (sx, sy) = (t.tiling.u, t.tiling.v);
        let (tx, ty) = (t.translation.u, t.translation.v);
        let (cx, cy) = (t.center_offset.u, t.center_offset.v);

        let [c0, c1, c2] = match t.transform_type {
            TextureTransform::MAX => [
                [sx * cos, sy * -sin, 0.0],
                [sx * sin, sy * cos, 0.0],
                [
                    cx + sx * (cos * (-cx - tx) + sin * (-cy + ty)),
                    cy + sy * (-sin * (-cx - tx) + cos * (-cy + ty)),
                    1.0,
                ],
            ],
            TextureTransform::MAYA_DEPRECATED => [
                [cos * sx, sin * sx, 0.0],
                [-sin * sy, cos * sy, 0.0],
                [
                    (tx - cx) * cos + (ty - cy) * -sin + cx,
                    (tx - cx) * sin + (ty - cy) * cos + cy,
                    1.0,
                ],
            ],
            _ => [
                [cos * sx, -sin * sx, 0.0],
                [-sin * sy, -cos * sy, 0.0],
                [
                    (tx - cx) * cos + (-ty - cy + 1.0) * sin + cx,
                    (-tx + cx) * sin + (-ty - cy + 1.0) * cos + cy,
                    1.0,
                ],
            ],
        };
        glam::Mat3::from_cols(c0.into(), c1.into(), c2.into())
    }

    fn cases() -> Vec<TextureTransform> {
        let mut out = Vec::new();
        for method in [
            TextureTransform::MAYA_DEPRECATED,
            TextureTransform::MAX,
            TextureTransform::MAYA,
        ] {
            for (tx, ty) in [(0.0, 0.0), (0.25, -0.5), (3.0, 1.5)] {
                for (sx, sy) in [(1.0, 1.0), (3.0, 1.0), (-0.2, 0.75)] {
                    for rotate in [0.0, 0.7, -std::f32::consts::FRAC_PI_4, 2.5] {
                        for (cx, cy) in [(0.5, 0.5), (0.0, 0.0), (0.25, 0.75)] {
                            out.push(TextureTransform {
                                translation: TexCoord { u: tx, v: ty },
                                tiling: TexCoord { u: sx, v: sy },
                                w_rotation: rotate,
                                transform_type: method,
                                center_offset: TexCoord { u: cx, v: cy },
                            });
                        }
                    }
                }
            }
        }
        out
    }

    #[test]
    fn the_composition_matches_the_engines_closed_form() {
        for case in cases() {
            let (ours, theirs) = (case.matrix(), shipped(&case));
            assert!(
                ours.abs_diff_eq(theirs, 1e-5),
                "{case:?}
  composed {ours:?}
  shipped  {theirs:?}"
            );
        }
    }

    #[test]
    fn max_translation_runs_backwards_in_u() {
        let mut t = TextureTransform {
            transform_type: TextureTransform::MAX,
            ..TextureTransform::identity()
        };
        t.translation = TexCoord { u: 0.25, v: 0.5 };
        let uv = t.matrix() * glam::Vec3::new(0.0, 0.0, 1.0);

        assert!((uv.x - -0.25).abs() < 1e-6, "u went {}", uv.x);
        assert!((uv.y - 0.5).abs() < 1e-6, "v went {}", uv.y);
    }

    #[test]
    fn the_centre_is_a_fixed_point_of_a_rotation() {
        for method in [
            TextureTransform::MAYA_DEPRECATED,
            TextureTransform::MAX,
            TextureTransform::MAYA,
        ] {
            let mut t = TextureTransform {
                transform_type: method,
                ..TextureTransform::identity()
            };
            t.w_rotation = 0.7;
            t.center_offset = TexCoord { u: 0.25, v: 0.75 };

            // maya measures v from the other edge, so the point it holds still is the flipped one
            let held = match method {
                TextureTransform::MAYA => glam::Vec3::new(0.25, 1.0 - 0.75, 1.0),
                _ => glam::Vec3::new(0.25, 0.75, 1.0),
            };
            let turned = t.matrix() * held;

            assert!(
                (turned.x - 0.25).abs() < 1e-6 && (turned.y - 0.75).abs() < 1e-6,
                "method {method} moved its own centre to {turned:?}"
            );
        }
    }

    #[test]
    fn the_maya_substitute_is_not_neutral() {
        let m = TextureTransform::identity().matrix();
        let uv = m * glam::Vec3::new(0.0, 0.25, 1.0);

        assert!((uv.x - 0.0).abs() < 1e-6, "u moved to {}", uv.x);
        assert!(
            (uv.y - 0.75).abs() < 1e-6,
            "v should flip to 0.75, went {}",
            uv.y
        );
    }
}
