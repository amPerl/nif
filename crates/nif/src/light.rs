use glam::{Mat4, Vec3};

use crate::blocks::Block;

/// How a light falls off, which decides what a renderer has to do with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Falloff {
    /// Contributes to the scene's ambient term and nothing else.
    Ambient,
    /// Parallel rays, no attenuation, no position.
    Directional,
    /// Attenuates with distance from `position`.
    Point,
    /// A point light inside a cone.
    Spot,
}

/// One light resolved into what a renderer needs, with the file's own conventions already
/// applied: `dimmer` folded into the colours, the direction taken from the world transform, and
/// the spot cone converted out of the half angle in degrees the file stores.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lit {
    pub falloff: Falloff,
    /// Where the light is, for the two that attenuate.
    pub position: Vec3,
    /// The way the light travels, which is the opposite of the direction back to it.
    pub direction: Vec3,
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    /// Constant, linear and quadratic, in that order.
    pub attenuation: Vec3,
    /// The cosine of the cone's half angle. Outside it a spot contributes nothing.
    pub cos_cutoff: f32,
    /// How sharply a spot fades from its axis to its edge.
    pub exponent: f32,
}

/// A light as the renderer needs it, or `None` where the block is not a light.
///
/// `world` is the light's own world transform, which only a traversal knows. The direction of
/// travel is the first column of its rotation, not the third that a Z up format suggests.
///
/// `dimmer` replaces the light's own where a controller drives it.
pub fn resolve(block: &Block, world: Mat4, dimmer: Option<f32>) -> Option<Lit> {
    let light = block.light()?;
    let scale = dimmer.unwrap_or(light.dimmer);
    let colour = |c: &crate::common::Color3| Vec3::new(c.r, c.g, c.b) * scale;

    let mut out = Lit {
        falloff: Falloff::Ambient,
        position: world.w_axis.truncate(),
        direction: world.x_axis.truncate().normalize_or_zero(),
        ambient: colour(&light.ambient_color),
        diffuse: colour(&light.diffuse_color),
        specular: colour(&light.specular_color),
        attenuation: Vec3::new(1.0, 0.0, 0.0),
        cos_cutoff: -1.0,
        exponent: 0.0,
    };

    match block {
        Block::NiAmbientLight(_) => {}
        Block::NiDirectionalLight(_) => out.falloff = Falloff::Directional,
        Block::NiPointLight(point) => {
            out.falloff = Falloff::Point;
            out.attenuation = Vec3::new(
                point.constant_attenuation,
                point.linear_attenuation,
                point.quadratic_attenuation,
            );
        }
        Block::NiSpotLight(spot) => {
            out.falloff = Falloff::Spot;
            out.attenuation = Vec3::new(
                spot.constant_attenuation,
                spot.linear_attenuation,
                spot.quadratic_attenuation,
            );
            // the file stores a half angle in degrees, and the device wants the full angle in
            // radians, so the cone's own half angle is the stored value in radians
            out.cos_cutoff = spot.outer_spot_angle.to_radians().cos();
            out.exponent = spot.exponent;
        }
        _ => return None,
    }
    Some(out)
}

/// The scene's ambient term: every ambient light in force, summed.
///
/// An ambient light never becomes a light of its own. The engine folds it into one render
/// state and leaves it there.
pub fn ambient(lit: impl IntoIterator<Item = Lit>) -> Vec3 {
    lit.into_iter()
        .filter(|l| l.falloff == Falloff::Ambient)
        .map(|l| l.ambient)
        .fold(Vec3::ZERO, |sum, c| sum + c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{NiDynamicEffect, NiLight, NiPointLight, NiSpotLight, NiString};
    use crate::common::{BlockRef, Color3, Matrix33, Vector3};

    fn colour(r: f32, g: f32, b: f32) -> Color3 {
        Color3 { r, g, b }
    }

    fn base(dimmer: f32) -> NiLight {
        NiLight {
            base: NiDynamicEffect {
                base: crate::blocks::NiAvObject {
                    base: crate::blocks::NiObjectNET {
                        name: NiString::from("light"),
                        extra_data_refs: Vec::new(),
                        controller_ref: BlockRef::None,
                    },
                    flags: 0,
                    translation: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    rotation: Matrix33::IDENTITY,
                    scale: 1.0,
                    property_refs: Vec::new(),
                    collision_ref: BlockRef::None,
                },
                switch_state: true,
                unaffected_node_refs: Vec::new(),
            },
            dimmer,
            ambient_color: colour(0.1, 0.2, 0.3),
            diffuse_color: colour(0.4, 0.5, 0.6),
            specular_color: colour(0.7, 0.8, 0.9),
        }
    }

    /// The dimmer is not a term of its own: the engine multiplies it into all three colours
    /// before the light is ever uploaded, so a renderer applying it again halves everything.
    #[test]
    fn the_dimmer_is_folded_into_every_colour() {
        let block = Block::NiDirectionalLight(crate::blocks::NiDirectionalLight { base: base(0.5) });
        let lit = resolve(&block, Mat4::IDENTITY, None).expect("a light");

        assert!((lit.ambient - Vec3::new(0.05, 0.1, 0.15)).length() < 1e-6);

        // a controller driving the dimmer replaces the light's own rather than compounding it
        let driven = resolve(&block, Mat4::IDENTITY, Some(1.0)).expect("a light");
        assert!((driven.diffuse - Vec3::new(0.4, 0.5, 0.6)).length() < 1e-6);
        assert!((lit.diffuse - Vec3::new(0.2, 0.25, 0.3)).length() < 1e-6);
        assert!((lit.specular - Vec3::new(0.35, 0.4, 0.45)).length() < 1e-6);
    }

    /// A directional light travels along the first column of its world rotation. Taking the
    /// third, which is the natural guess for a Z up format, points it somewhere else entirely.
    #[test]
    fn a_directional_light_travels_along_its_own_x() {
        let block = Block::NiDirectionalLight(crate::blocks::NiDirectionalLight { base: base(1.0) });

        let lit = resolve(&block, Mat4::IDENTITY, None).expect("a light");
        assert!((lit.direction - Vec3::X).length() < 1e-6);

        // turned a quarter about z, its own x now points along world y
        let turned = Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let lit = resolve(&block, turned, None).expect("a light");
        assert!(
            (lit.direction - Vec3::Y).length() < 1e-5,
            "travels {:?}",
            lit.direction
        );
    }

    /// The file stores the cone as a half angle in degrees. The engine doubles it and converts
    /// to radians for a device that wants the full angle, so the half angle in radians is what a
    /// cosine cutoff is taken from. Doubling it here as well opens the cone to twice its size.
    #[test]
    fn a_spot_cone_comes_from_the_half_angle_the_file_stores() {
        let block = Block::NiSpotLight(NiSpotLight {
            base: NiPointLight {
                base: base(1.0),
                constant_attenuation: 1.0,
                linear_attenuation: 0.5,
                quadratic_attenuation: 0.25,
            },
            outer_spot_angle: 60.0,
            exponent: 2.0,
        });
        let lit = resolve(&block, Mat4::IDENTITY, None).expect("a light");

        assert_eq!(lit.falloff, Falloff::Spot);
        // 60 degrees off the axis, so the cone spans 120 degrees in full
        assert!((lit.cos_cutoff - 0.5).abs() < 1e-6, "got {}", lit.cos_cutoff);
        assert_eq!(lit.exponent, 2.0);
        assert!((lit.attenuation - Vec3::new(1.0, 0.5, 0.25)).length() < 1e-6);
    }

    /// A point light is where its node is, and it attenuates. A directional one is neither.
    #[test]
    fn a_point_light_sits_where_its_node_does() {
        let block = Block::NiPointLight(NiPointLight {
            base: base(1.0),
            constant_attenuation: 1.0,
            linear_attenuation: 0.0,
            quadratic_attenuation: 0.02,
        });
        let at = Mat4::from_translation(Vec3::new(3.0, -4.0, 5.0));
        let lit = resolve(&block, at, None).expect("a light");

        assert_eq!(lit.falloff, Falloff::Point);
        assert_eq!(lit.position, Vec3::new(3.0, -4.0, 5.0));
        assert_eq!(lit.attenuation, Vec3::new(1.0, 0.0, 0.02));
    }

    /// Ambient lights sum into one term rather than each becoming a light, and nothing else in
    /// the set contributes to it however much ambient colour it carries.
    #[test]
    fn ambient_lights_sum_and_the_others_stay_out_of_it() {
        let ambient = Block::NiAmbientLight(crate::blocks::NiAmbientLight { base: base(1.0) });
        let directional =
            Block::NiDirectionalLight(crate::blocks::NiDirectionalLight { base: base(1.0) });
        let lit: Vec<Lit> = [&ambient, &directional, &ambient]
            .iter()
            .filter_map(|b| resolve(b, Mat4::IDENTITY, None))
            .collect();

        // two ambients at 0.1,0.2,0.3, and the directional's own ambient left out
        let total = super::ambient(lit);
        assert!((total - Vec3::new(0.2, 0.4, 0.6)).length() < 1e-6, "{total:?}");
    }
}
