use super::Vector3;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead, Clone, Copy)]
pub struct Matrix22 {
    pub m11: f32,
    pub m21: f32,
    pub m12: f32,
    pub m22: f32,
}

#[derive(Debug, PartialEq, BinRead, Clone, Copy)]
pub struct Matrix33 {
    pub column_major: [f32; 9],
}

impl Matrix33 {
    pub const IDENTITY: Matrix33 = Matrix33 {
        column_major: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    };

    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        if row > 2 || col > 2 {
            return None;
        }
        self.column_major.get(row * 3 + col).copied()
    }

    pub fn mul(&self, rhs: &Matrix33) -> Matrix33 {
        let a = &self.column_major;
        let b = &rhs.column_major;
        let mut out = [0.0f32; 9];
        for row in 0..3 {
            for col in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += a[row * 3 + k] * b[k * 3 + col];
                }
                out[row * 3 + col] = sum;
            }
        }
        Matrix33 { column_major: out }
    }

    pub fn mul_vector(&self, v: &Vector3) -> Vector3 {
        let m = &self.column_major;
        Vector3 {
            x: m[0] * v.x + m[1] * v.y + m[2] * v.z,
            y: m[3] * v.x + m[4] * v.y + m[5] * v.z,
            z: m[6] * v.x + m[7] * v.y + m[8] * v.z,
        }
    }
}

impl Default for Matrix33 {
    fn default() -> Self {
        Matrix33::IDENTITY
    }
}

#[cfg(feature = "glam")]
impl From<&Matrix33> for glam::Mat3 {
    fn from(val: &Matrix33) -> Self {
        glam::Mat3::from_cols_array(&val.column_major).transpose()
    }
}

#[cfg(feature = "glam")]
impl From<Matrix33> for glam::Mat3 {
    fn from(val: Matrix33) -> Self {
        (&val).into()
    }
}

#[derive(Debug, PartialEq, BinRead, Clone, Copy)]
pub struct NiTransform {
    pub rotation: Matrix33,
    pub translation: Vector3,
    pub scale: f32,
}

impl NiTransform {
    pub const IDENTITY: NiTransform = NiTransform {
        rotation: Matrix33::IDENTITY,
        translation: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: 1.0,
    };

    pub fn compose(&self, child: &NiTransform) -> NiTransform {
        let rotated = self.rotation.mul_vector(&child.translation);
        NiTransform {
            rotation: self.rotation.mul(&child.rotation),
            translation: Vector3 {
                x: rotated.x * self.scale + self.translation.x,
                y: rotated.y * self.scale + self.translation.y,
                z: rotated.z * self.scale + self.translation.z,
            },
            scale: self.scale * child.scale,
        }
    }

    pub fn transform_point(&self, p: &Vector3) -> Vector3 {
        let r = self.rotation.mul_vector(p);
        Vector3 {
            x: r.x * self.scale + self.translation.x,
            y: r.y * self.scale + self.translation.y,
            z: r.z * self.scale + self.translation.z,
        }
    }

    pub fn transform_vector(&self, v: &Vector3) -> Vector3 {
        let r = self.rotation.mul_vector(v);
        Vector3 {
            x: r.x * self.scale,
            y: r.y * self.scale,
            z: r.z * self.scale,
        }
    }
}

impl Default for NiTransform {
    fn default() -> Self {
        NiTransform::IDENTITY
    }
}

impl From<&crate::blocks::NiAvObject> for NiTransform {
    fn from(av: &crate::blocks::NiAvObject) -> Self {
        NiTransform {
            rotation: av.rotation,
            translation: av.translation,
            scale: av.scale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f32 = 8.0 * f32::EPSILON;

    fn approx(a: &Vector3, b: &Vector3) -> bool {
        let close = |x: f32, y: f32| (x - y).abs() <= TOLERANCE * x.abs().max(y.abs()).max(1.0);
        close(a.x, b.x) && close(a.y, b.y) && close(a.z, b.z)
    }

    fn v(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn rot_z(degrees: f32) -> Matrix33 {
        let (s, c) = degrees.to_radians().sin_cos();
        Matrix33 {
            column_major: [c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0],
        }
    }

    fn rot_messy() -> Matrix33 {
        rot_z(37.0).mul(&Matrix33 {
            column_major: [
                1.0,
                0.0,
                0.0,
                0.0,
                23.0f32.to_radians().cos(),
                -23.0f32.to_radians().sin(),
                0.0,
                23.0f32.to_radians().sin(),
                23.0f32.to_radians().cos(),
            ],
        })
    }

    #[test]
    fn identity_is_a_no_op() {
        let p = v(1.0, 2.0, 3.0);
        assert!(approx(&NiTransform::IDENTITY.transform_point(&p), &p));
    }

    #[test]
    fn rotation_applies_about_the_right_axis() {
        let t = NiTransform {
            rotation: rot_z(90.0),
            translation: Vector3::default(),
            scale: 1.0,
        };
        let got = t.transform_point(&v(1.0, 0.0, 0.0));
        assert!(approx(&got, &v(0.0, 1.0, 0.0)), "got {:?}", got);
    }

    #[test]
    fn transform_vector_ignores_translation() {
        let t = NiTransform {
            rotation: Matrix33::IDENTITY,
            translation: v(10.0, 10.0, 10.0),
            scale: 2.0,
        };
        assert!(approx(
            &t.transform_vector(&v(1.0, 0.0, 0.0)),
            &v(2.0, 0.0, 0.0)
        ));
    }

    #[test]
    fn compose_matches_applying_in_sequence() {
        let parent = NiTransform {
            rotation: rot_messy(),
            translation: v(10.3, -0.7, 4.1),
            scale: 1.7,
        };
        let child = NiTransform {
            rotation: rot_z(37.0),
            translation: v(0.3, 3.9, -2.6),
            scale: 0.31,
        };
        let p = v(1.1, -0.9, 2.7);

        let stepwise = parent.transform_point(&child.transform_point(&p));
        let composed = parent.compose(&child).transform_point(&p);

        assert!(
            approx(&stepwise, &composed),
            "stepwise {:?} vs composed {:?}",
            stepwise,
            composed
        );
    }

    #[test]
    fn compose_accumulates_scale() {
        let a = NiTransform {
            scale: 2.0,
            ..NiTransform::IDENTITY
        };
        let b = NiTransform {
            scale: 3.0,
            ..NiTransform::IDENTITY
        };
        assert_eq!(a.compose(&b).scale, 6.0);
    }

    #[test]
    fn matrix_get_is_row_major() {
        let m = Matrix33 {
            column_major: [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        };
        assert_eq!(m.get(0, 1), Some(1.0));
        assert_eq!(m.get(2, 0), Some(6.0));
        assert_eq!(m.get(3, 0), None);
    }

    #[cfg(feature = "glam")]
    fn glam_reference(t: &NiTransform) -> glam::Mat4 {
        glam::Mat4::from_translation(t.translation.into())
            * glam::Mat4::from_mat3(t.rotation.into())
            * glam::Mat4::from_scale(glam::Vec3::splat(t.scale))
    }

    #[cfg(feature = "glam")]
    #[test]
    fn transform_point_matches_glam() {
        let t = NiTransform {
            rotation: rot_messy(),
            translation: v(5.3, -2.7, 7.1),
            scale: 3.3,
        };
        let p = v(1.1, 2.3, 3.7);

        let e = glam_reference(&t).transform_point3(p.into());
        let got = t.transform_point(&p);

        assert!(
            approx(&got, &v(e.x, e.y, e.z)),
            "got {:?}, glam says {:?}",
            got,
            e
        );
    }

    #[cfg(feature = "glam")]
    #[test]
    fn compose_matches_glam_matrix_product() {
        let parent = NiTransform {
            rotation: rot_messy(),
            translation: v(10.7, 1.3, -4.9),
            scale: 2.3,
        };
        let child = NiTransform {
            rotation: rot_z(37.0),
            translation: v(0.7, 3.1, 6.9),
            scale: 0.53,
        };
        let p = v(1.3, -1.7, 2.9);

        let e = (glam_reference(&parent) * glam_reference(&child)).transform_point3(p.into());
        let got = parent.compose(&child).transform_point(&p);

        assert!(
            approx(&got, &v(e.x, e.y, e.z)),
            "got {:?}, glam says {:?}",
            got,
            e
        );
    }
}
