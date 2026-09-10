use glam::{Mat3, Vec3};

use crate::blocks::BillboardMode;
use crate::common::{Matrix33, NiTransform, Vector3};

/// Where the viewer is and how it is oriented, all in world space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub location: Vector3,
    pub right: Vector3,
    pub up: Vector3,
    /// The way the camera looks, so a billboard turns to face the other way.
    pub direction: Vector3,
}

/// The camera's axes, with the facing already reversed into the direction a billboard turns to
/// meet.
struct Basis {
    facing: Vec3,
    up: Vec3,
    right: Vec3,
}

impl Basis {
    fn of(camera: &Camera) -> Basis {
        Basis {
            facing: -Vec3::from(&camera.direction),
            up: Vec3::from(&camera.up),
            right: Vec3::from(&camera.right),
        }
    }

    /// Turned to look directly at `at` instead of along the camera's own direction. This is
    /// what separates the `Center` modes from the `Camera` ones: the whole screen shares one
    /// orientation under the latter, while each node gets its own under the former.
    fn toward(self, camera: &Camera, at: Vec3) -> Option<Basis> {
        let to_camera = Vec3::from(&camera.location) - at;
        if to_camera.length_squared() < 0.001 {
            return None;
        }
        let to_camera = to_camera.try_normalize()?;

        let aligned = to_camera.dot(self.facing);
        if aligned >= 0.999999 {
            return Some(self);
        }
        // the turn that carries the camera's own facing onto the direction of this node
        let axis = self.facing.cross(to_camera).try_normalize()?;
        let turn = Mat3::from_axis_angle(axis, aligned.clamp(-1.0, 1.0).acos());
        Some(Basis {
            facing: to_camera,
            up: turn * self.up,
            right: turn * self.right,
        })
    }

}

impl BillboardMode {
    /// The world rotation this mode gives a node whose unmodified world transform is `world`.
    ///
    /// None means the mode declines to orient, and the node keeps its own rotation. Descendants
    /// inherit the result, so this replaces the node's world rotation rather than being applied to
    /// the geometry under it.
    pub fn orient(&self, world: &NiTransform, camera: &Camera) -> Option<Matrix33> {
        // A node's own frame is a space like any other, so this is `aim` asked from inside it:
        // the camera has already been brought in by whoever walked down to here, the node stands
        // at its own translation, and the rotation it rests at is its own. What `aim` gives back
        // is `rotation * face` with the two cancelling parts already gone, which is what this
        // used to build the long way round.
        if world.scale.abs() < 1e-8 {
            // kept from when the pivoting modes divided by it, so a node scaled to nothing still
            // declines here rather than orienting
            return None;
        }
        self.aim(
            camera,
            Vec3::from(&world.translation),
            Mat3::from(&world.rotation),
        )
        .map(Matrix33::from)
    }

    /// The turn a copy of a billboard takes, without the trip through the node's own space.
    ///
    /// `orient` answers in the node's frame, which means a caller drawing copies has to bring the
    /// camera in through each copy's placement, turn the node there, and multiply the answer back
    /// out again. Those cancel. Writing the placement as `T Q s`, the node's resting world pose as
    /// `T_r R_r s_r` and the camera's axes as the columns of `A`, what a copy is finally drawn
    /// with comes out as
    ///
    /// ```text
    ///     s * A * C * R_r^T
    /// ```
    ///
    /// with the placement's own rotation `Q` gone from both sides and `C` the roll the upright
    /// modes add. So the camera can stay in the world and no placement need ever be inverted.
    ///
    /// `pivot` is where the copy's placement leaves the node's origin, and `standing` is where it
    /// leaves the node's resting rotation, columns of unit length. What comes back still wants the
    /// transpose of that resting rotation on its right and the placement's scale on it, both of
    /// which are one value for a whole draw rather than one a copy.
    ///
    /// None means the same as it does for `orient`: the mode declines, and the node keeps what it
    /// had.
    pub fn aim(&self, camera: &Camera, pivot: Vec3, standing: Mat3) -> Option<Mat3> {
        match self {
            BillboardMode::AlwaysFaceCamera
            | BillboardMode::AlwaysFaceCenter
            | BillboardMode::RigidFaceCamera
            | BillboardMode::RigidFaceCenter => {
                let basis = Basis::of(camera);
                let basis = match self {
                    BillboardMode::AlwaysFaceCenter | BillboardMode::RigidFaceCenter => {
                        basis.toward(camera, pivot)?
                    }
                    _ => basis,
                };
                let Basis { facing, up, right } = basis;

                Some(match self {
                    // rigid drops the node's own orientation entirely, since this cancels
                    // against the rotation it is multiplied by
                    BillboardMode::RigidFaceCamera | BillboardMode::RigidFaceCenter => {
                        Mat3::from_cols(right, up, facing)
                    }
                    // The others roll the quad about the view direction so its own up stays as
                    // upright as the view allows. Which up that is, is the node's own y axis as
                    // the copy's placement leaves it standing.
                    _ => {
                        let own = standing.y_axis;
                        let (along, across) = (own.dot(up), -own.dot(right));
                        let root = (across * across + along * along).sqrt();
                        if root > 1e-6 {
                            let inverse = 1.0 / root;
                            let (cos, sin) = (along * inverse, across * inverse);
                            Mat3::from_cols(right * cos + up * sin, right * -sin + up * cos, facing)
                        } else {
                            Mat3::from_cols(-right, -up, facing)
                        }
                    }
                })
            }

            // Pivots about the node's own up, keeping x and z as the ground plane. This one does
            // not shed the placement the way the others do: the turn is about an axis of the
            // node's rather than one of the camera's, so where the copy stands turned stays in
            // the answer. The node's own scale drops out either way, since all that is taken from
            // the direction to the eye is which way it points.
            BillboardMode::RotateAboutUp
            | BillboardMode::BSRotateAboutUp
            | BillboardMode::RotateAboutUp2 => {
                let offset = Vec3::from(&camera.location) - pivot;
                let local = standing.transpose() * offset;
                let flat = glam::Vec2::new(local.x, local.z).try_normalize()?;
                Some(
                    standing
                        * Mat3::from_cols(
                            Vec3::new(flat.y, 0.0, -flat.x),
                            Vec3::Y,
                            Vec3::new(flat.x, 0.0, flat.y),
                        ),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Looking down -y at the origin, with z up, which is the frame a NIF viewer is in.
    fn camera() -> Camera {
        Camera {
            location: Vec3::new(0.0, -10.0, 0.0).into(),
            right: Vec3::X.into(),
            up: Vec3::Z.into(),
            direction: Vec3::Y.into(),
        }
    }

    fn at(translation: Vec3) -> NiTransform {
        NiTransform {
            rotation: Matrix33::IDENTITY,
            translation: translation.into(),
            scale: 1.0,
        }
    }

    fn column(oriented: &Matrix33, col: usize) -> Vec3 {
        Mat3::from(oriented).col(col)
    }

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-5
    }

    #[test]
    fn a_rigid_billboard_takes_the_cameras_own_basis() {
        let camera = camera();
        let oriented = BillboardMode::RigidFaceCamera
            .orient(&at(Vec3::new(5.0, 3.0, 0.0)), &camera)
            .unwrap();
        // the third column is what the quad's local z ends up pointing along, which is the
        // way back to the camera
        assert!(close(column(&oriented, 2), -Vec3::Y));
        assert!(close(column(&oriented, 0), camera.right.into()));
        assert!(close(column(&oriented, 1), camera.up.into()));
    }

    #[test]
    fn a_rigid_billboard_ignores_the_nodes_own_rotation() {
        let camera = camera();
        let mut turned = at(Vec3::ZERO);
        turned.rotation = Mat3::from_rotation_z(std::f32::consts::FRAC_PI_2).into();
        let oriented = BillboardMode::RigidFaceCamera
            .orient(&turned, &camera)
            .unwrap();
        let plain = BillboardMode::RigidFaceCamera
            .orient(&at(Vec3::ZERO), &camera)
            .unwrap();
        assert!(close(column(&oriented, 2), column(&plain, 2)));
        assert!(close(column(&oriented, 0), column(&plain, 0)));
    }

    #[test]
    fn face_center_points_at_the_camera_not_along_it() {
        let camera = camera();
        // off to one side, so looking at the camera differs from looking along its direction
        let world = at(Vec3::new(10.0, 0.0, 0.0));
        let oriented = BillboardMode::RigidFaceCenter
            .orient(&world, &camera)
            .unwrap();
        let facing = column(&oriented, 2);
        let expected = (Vec3::from(&camera.location) - Vec3::from(&world.translation)).normalize();
        assert!(close(facing, expected), "{facing} vs {expected}");

        // the camera mode instead gives every node the same facing
        let along = BillboardMode::RigidFaceCamera
            .orient(&world, &camera)
            .unwrap();
        assert!(!close(column(&along, 2), facing));
    }

    #[test]
    fn a_billboard_on_top_of_the_camera_declines_to_orient() {
        let camera = camera();
        assert!(BillboardMode::RigidFaceCenter
            .orient(&at(camera.location.into()), &camera)
            .is_none());
    }

    #[test]
    fn always_face_keeps_the_quad_upright() {
        let camera = camera();
        let oriented = BillboardMode::AlwaysFaceCamera
            .orient(&at(Vec3::ZERO), &camera)
            .unwrap();
        assert!(close(column(&oriented, 2), -Vec3::Y));
        // its own up has no sideways lean
        assert!(column(&oriented, 1).x.abs() < 1e-5);
    }

    #[test]
    fn the_result_stays_orthonormal() {
        let camera = camera();
        let world = at(Vec3::new(2.0, 7.0, -3.0));
        for mode in [
            BillboardMode::AlwaysFaceCamera,
            BillboardMode::RigidFaceCamera,
            BillboardMode::AlwaysFaceCenter,
            BillboardMode::RigidFaceCenter,
            BillboardMode::RotateAboutUp,
        ] {
            let oriented = Mat3::from(&mode.orient(&world, &camera).unwrap());
            assert!(
                oriented.determinant() > 0.0,
                "{mode:?} turned the geometry inside out"
            );
            for col in 0..3 {
                assert!(
                    (oriented.col(col).length() - 1.0).abs() < 1e-4,
                    "{mode:?} column {col}"
                );
            }
            for (a, b) in [(0, 1), (1, 2), (0, 2)] {
                let product = oriented.col(a).dot(oriented.col(b));
                assert!(product.abs() < 1e-4, "{mode:?} columns {a} and {b}");
            }
        }
    }
}
