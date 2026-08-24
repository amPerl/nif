use glam::{Mat3, Quat};

use crate::blocks::{
    Block, NiAvObject, NiMaterialProperty, NiTimeController, NiTransformData,
    NiTransformInterpolator,
};
use crate::common::{
    BlockRef, Key, KeyGroup, KeyType, NiQuatTransform, NiTransform, Quaternion, Vector3,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleType {
    Loop,
    Reverse,
    Clamp,
    Invalid(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pose {
    pub rotation: Option<Quaternion>,
    pub translation: Option<Vector3>,
    pub scale: Option<f32>,
}

impl Pose {
    pub fn is_empty(&self) -> bool {
        self.rotation.is_none() && self.translation.is_none() && self.scale.is_none()
    }

    fn or(self, fallback: Pose) -> Pose {
        Pose {
            rotation: self.rotation.or(fallback.rotation),
            translation: self.translation.or(fallback.translation),
            scale: self.scale.or(fallback.scale),
        }
    }
}

impl Pose {
    /// `base` with the animated channels replaced. A channel with no keys keeps its own value,
    /// which is why an absent one has to stay absent rather than become a default.
    pub fn apply(&self, base: &NiTransform) -> NiTransform {
        NiTransform {
            rotation: self
                .rotation
                .map_or(base.rotation, |q| Mat3::from_quat(q.into()).into()),
            translation: self.translation.unwrap_or(base.translation),
            scale: self.scale.unwrap_or(base.scale),
        }
    }
}

/// The span every controller in the file covers, or None when it holds no animation.
pub fn span(blocks: &[Block]) -> Option<(f32, f32)> {
    let mut span: Option<(f32, f32)> = None;
    for block in blocks {
        let Some(time) = block.as_time_controller() else {
            continue;
        };
        let (start, end) = (time.start_time, time.end_time);
        if !start.is_finite() || !end.is_finite() || end <= start {
            continue;
        }
        span = Some(match span {
            Some((lo, hi)) => (lo.min(start), hi.max(end)),
            None => (start, end),
        });
    }
    span
}

/// The transform an object holds at `time`, following its controller chain. None when nothing
/// animates it, so a caller can keep whatever it already had.
pub fn transform_at(blocks: &[Block], object: &NiAvObject, time: f32) -> Option<NiTransform> {
    for block in controllers(blocks, object.controller_ref) {
        let Block::NiTransformController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active() {
            continue;
        }
        let Some(Block::NiTransformInterpolator(interpolator)) =
            controller.base.interpolator_ref.get(blocks)
        else {
            continue;
        };
        let pose = interpolator.sample(blocks, time_controller.local_time(time));
        if pose.is_empty() {
            continue;
        }
        return Some(pose.apply(&NiTransform::from(object)));
    }
    None
}

/// The value a channel carries when it supplies nothing, so the target keeps its own.
const INVALID: f32 = -f32::MAX;

impl NiQuatTransform {
    pub fn translation(&self) -> Option<Vector3> {
        (self.translation.x != INVALID).then_some(self.translation)
    }

    pub fn rotation(&self) -> Option<Quaternion> {
        (self.rotation.x != INVALID).then_some(self.rotation)
    }

    pub fn scale(&self) -> Option<f32> {
        (self.scale != INVALID).then_some(self.scale)
    }

    pub fn pose(&self) -> Pose {
        Pose {
            rotation: self.rotation(),
            translation: self.translation(),
            scale: self.scale(),
        }
    }
}

impl NiTimeController {
    pub fn cycle_type_enum(&self) -> CycleType {
        match self.cycle_type() {
            0 => CycleType::Loop,
            1 => CycleType::Reverse,
            2 => CycleType::Clamp,
            other => CycleType::Invalid(other),
        }
    }

    pub fn duration(&self) -> f32 {
        self.end_time - self.start_time
    }

    /// Where `time` lands inside the controller's own span, after frequency, phase and the
    /// cycle rule. `Reverse` runs the span forwards then backwards over twice the duration.
    pub fn local_time(&self, time: f32) -> f32 {
        let duration = self.duration();
        let scaled = time * self.frequency + self.phase;
        if duration <= 0.0 {
            return self.start_time;
        }
        match self.cycle_type_enum() {
            CycleType::Clamp | CycleType::Invalid(_) => {
                scaled.clamp(self.start_time, self.end_time)
            }
            CycleType::Loop => {
                let offset = (scaled - self.start_time).rem_euclid(duration);
                self.start_time + offset
            }
            CycleType::Reverse => {
                let offset = (scaled - self.start_time).rem_euclid(duration * 2.0);
                self.start_time
                    + if offset <= duration {
                        offset
                    } else {
                        duration * 2.0 - offset
                    }
            }
        }
    }
}

pub trait Interpolate: Copy {
    fn lerp(from: Self, to: Self, t: f32) -> Self;

    /// Hermite, where the tangents are value deltas across the segment rather than slopes.
    /// With both tangents equal to `to - from` this is exactly linear, which is how an
    /// exporter writes a constant rate segment.
    fn hermite(from: Self, out_of_from: Self, to: Self, into_to: Self, t: f32) -> Self;
}

impl Interpolate for f32 {
    fn lerp(from: Self, to: Self, t: f32) -> Self {
        from + (to - from) * t
    }

    fn hermite(from: Self, out_of_from: Self, to: Self, into_to: Self, t: f32) -> Self {
        let (t2, t3) = (t * t, t * t * t);
        (2.0 * t3 - 3.0 * t2 + 1.0) * from
            + (t3 - 2.0 * t2 + t) * out_of_from
            + (-2.0 * t3 + 3.0 * t2) * to
            + (t3 - t2) * into_to
    }
}

impl Interpolate for Vector3 {
    fn lerp(from: Self, to: Self, t: f32) -> Self {
        Vector3 {
            x: f32::lerp(from.x, to.x, t),
            y: f32::lerp(from.y, to.y, t),
            z: f32::lerp(from.z, to.z, t),
        }
    }

    fn hermite(from: Self, out_of_from: Self, to: Self, into_to: Self, t: f32) -> Self {
        Vector3 {
            x: f32::hermite(from.x, out_of_from.x, to.x, into_to.x, t),
            y: f32::hermite(from.y, out_of_from.y, to.y, into_to.y, t),
            z: f32::hermite(from.z, out_of_from.z, to.z, into_to.z, t),
        }
    }
}

impl<T> KeyGroup<T>
where
    T: Interpolate + Clone + binrw::BinRead + binrw::BinWrite + 'static,
    T: for<'a> binrw::BinRead<Args<'a> = ()>,
    T: for<'a> binrw::BinWrite<Args<'a> = ()>,
{
    /// The value at `time`, or None when the group carries no keys at all. Outside the key
    /// range the nearest key holds.
    pub fn sample(&self, time: f32) -> Option<T> {
        sample_keys(&self.keys, self.interpolation, time)
    }
}

impl KeyGroup<u8> {
    /// A bool track steps: a key holds until the next one is reached, with no blending between
    /// them. Outside the range the nearest key holds.
    pub fn step(&self, time: f32) -> Option<u8> {
        let first = self.keys.first()?;
        if time <= first.time {
            return Some(first.value);
        }
        let at = self
            .keys
            .partition_point(|key| key.time <= time)
            .saturating_sub(1);
        self.keys.get(at).map(|key| key.value)
    }
}

/// Whether anything in the file repeats. When nothing does, every controller holds its final
/// value once its span is over, so a player should stop at the end rather than start again.
pub fn repeats(blocks: &[Block]) -> bool {
    blocks
        .iter()
        .filter_map(Block::as_time_controller)
        .filter(|controller| controller.is_active())
        .any(|controller| {
            matches!(
                controller.cycle_type_enum(),
                CycleType::Loop | CycleType::Reverse
            )
        })
}

/// The alpha a material's controller sets at `time`. None when nothing animates it, so the
/// caller keeps the value the file stores. The controller replaces that value rather than
/// scaling it.
pub fn alpha_at(blocks: &[Block], material: &NiMaterialProperty, time: f32) -> Option<f32> {
    for block in controllers(blocks, material.controller_ref) {
        let Block::NiAlphaController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active() {
            continue;
        }
        let Some(Block::NiFloatInterpolator(interpolator)) =
            controller.base.base.interpolator_ref.get(blocks)
        else {
            continue;
        };
        let keyed = match interpolator.data_ref.get(blocks) {
            Some(Block::NiFloatData(data)) => data.data.sample(time_controller.local_time(time)),
            _ => None,
        };
        // with no keys of its own the interpolator supplies a single value instead
        return Some(keyed.unwrap_or(interpolator.value));
    }
    None
}

/// Whether an object's visibility controller shows it at `time`. None when nothing animates its
/// visibility, so the caller keeps whatever the object's own flag says.
pub fn visible_at(blocks: &[Block], object: &NiAvObject, time: f32) -> Option<bool> {
    for block in controllers(blocks, object.controller_ref) {
        let Block::NiVisController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active() {
            continue;
        }
        let Some(Block::NiBoolInterpolator(interpolator)) =
            controller.base.base.interpolator_ref.get(blocks)
        else {
            continue;
        };
        let keyed = match interpolator.data_ref.get(blocks) {
            Some(Block::NiBoolData(data)) => data.data.step(time_controller.local_time(time)),
            _ => None,
        };
        // with no keys of its own the interpolator supplies a single value instead
        return match keyed {
            Some(value) => Some(value != 0),
            None => interpolator.pose_value(),
        };
    }
    None
}

/// Every controller on an object, following the chain from one to the next.
fn controllers(blocks: &[Block], first: BlockRef) -> impl Iterator<Item = &Block> {
    let mut next = first;
    let mut guard = 0;
    std::iter::from_fn(move || {
        let block = next.get(blocks)?;
        let time_controller = block.as_time_controller()?;
        next = time_controller.next_controller_ref;
        guard += 1;
        (guard <= 64).then_some(block)
    })
}

/// `Tbc` needs per key derivatives the file does not carry, so it falls back to linear here.
fn sample_keys<T>(keys: &[Key<T>], interpolation: Option<KeyType>, time: f32) -> Option<T>
where
    T: Interpolate + Clone + binrw::BinRead + binrw::BinWrite + 'static,
    T: for<'a> binrw::BinRead<Args<'a> = ()>,
    T: for<'a> binrw::BinWrite<Args<'a> = ()>,
{
    let first = keys.first()?;
    let last = keys.last()?;
    if keys.len() == 1 || time <= first.time {
        return Some(first.value);
    }
    if time >= last.time {
        return Some(last.value);
    }

    // the last key at or before `time`, which is the segment's start
    let at = keys
        .partition_point(|key| key.time <= time)
        .saturating_sub(1);
    let from = keys.get(at)?;
    let Some(to) = keys.get(at.saturating_add(1)) else {
        return Some(from.value);
    };

    let span = to.time - from.time;
    if span <= 0.0 {
        return Some(to.value);
    }
    let t = (time - from.time) / span;

    match interpolation {
        Some(KeyType::Const) => Some(from.value),
        Some(KeyType::Quadratic) => {
            // the file stores the in tangent first, so the segment leaves `from` on its
            // second tangent and arrives at `to` on its first
            match (from.out_tangent, to.in_tangent) {
                (Some(out_of_from), Some(into_to)) => {
                    Some(T::hermite(from.value, out_of_from, to.value, into_to, t))
                }
                _ => Some(T::lerp(from.value, to.value, t)),
            }
        }
        _ => Some(T::lerp(from.value, to.value, t)),
    }
}

impl NiTransformData {
    /// The rotation at `time`, from whichever of the two representations the block uses.
    pub fn rotation_at(&self, time: f32) -> Option<Quaternion> {
        if let Some(axes) = &self.xyz_rotations {
            let mut angles = [0.0f32; 3];
            let mut any = false;
            for (axis, group) in axes.iter().enumerate().take(3) {
                if let Some(angle) = group.sample(time) {
                    angles[axis] = angle;
                    any = true;
                }
            }
            // z is the outermost turn, so the x angle is the one applied to a vector first
            return any.then(|| {
                (Quat::from_rotation_z(angles[2])
                    * Quat::from_rotation_y(angles[1])
                    * Quat::from_rotation_x(angles[0]))
                .into()
            });
        }

        let keys = &self.quaternion_keys;
        let mut pairs = keys
            .iter()
            .filter_map(|key| Some((key.time?, key.value.as_ref()?)));
        let first = pairs.next()?;
        let mut previous = first;
        for current in pairs {
            if time <= current.0 {
                let span = current.0 - previous.0;
                if span <= 0.0 {
                    return Some(*current.1);
                }
                let t = ((time - previous.0) / span).clamp(0.0, 1.0);
                return Some(Quat::from(previous.1).slerp(current.1.into(), t).into());
            }
            previous = current;
        }
        Some(*previous.1)
    }

    pub fn sample(&self, time: f32) -> Pose {
        Pose {
            rotation: self.rotation_at(time),
            translation: self.translations.sample(time),
            scale: self.scales.sample(time),
        }
    }
}

impl NiTransformInterpolator {
    /// The pose at `time`. Channels the data has no keys for fall back to the interpolator's
    /// own transform, and channels that transform marks absent stay absent.
    pub fn sample(&self, blocks: &[Block], time: f32) -> Pose {
        let data = match self.data_ref.get(blocks) {
            Some(Block::NiTransformData(data)) => data.sample(time),
            _ => Pose::default(),
        };
        data.or(self.transform.pose())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{KeyType, Matrix33};

    fn key(time: f32, value: f32, in_tangent: f32, out_tangent: f32) -> Key<f32> {
        Key {
            time,
            value,
            in_tangent: Some(in_tangent),
            out_tangent: Some(out_tangent),
            tbc: None,
        }
    }

    fn group(interpolation: KeyType, keys: Vec<Key<f32>>) -> KeyGroup<f32> {
        KeyGroup {
            interpolation: Some(interpolation),
            keys,
        }
    }

    /// Tangents that are the segment's own secant, which is how a constant rate turn is
    /// written. Read the two tangent fields the other way round and this eases instead.
    #[test]
    fn a_quadratic_segment_with_secant_tangents_is_linear() {
        let turn = std::f32::consts::TAU;
        let keys = group(
            KeyType::Quadratic,
            vec![
                key(0.0, turn, -0.0, turn),
                key(3.3333335, turn * 2.0, turn, -0.0),
            ],
        );
        for (t, expected) in [
            (0.0, turn),
            (0.8333334, turn * 1.25),
            (1.6666667, turn * 1.5),
            (3.3333335, turn * 2.0),
        ] {
            let got = keys.sample(t).unwrap();
            assert!(
                (got - expected).abs() < 1e-3,
                "at {t} got {got}, wanted {expected}"
            );
        }
    }

    #[test]
    fn a_const_key_holds_until_the_next_one() {
        let keys = group(
            KeyType::Const,
            vec![key(0.0, 5.0, 0.0, 0.0), key(2.0, 9.0, 0.0, 0.0)],
        );
        assert_eq!(keys.sample(0.0), Some(5.0));
        assert_eq!(keys.sample(1.999), Some(5.0));
        assert_eq!(keys.sample(2.0), Some(9.0));
    }

    #[test]
    fn linear_runs_between_the_bracketing_keys() {
        let keys = group(
            KeyType::Linear,
            vec![
                key(0.0, 0.0, 0.0, 0.0),
                key(1.0, 10.0, 0.0, 0.0),
                key(2.0, 0.0, 0.0, 0.0),
            ],
        );
        assert_eq!(keys.sample(0.5), Some(5.0));
        assert_eq!(keys.sample(1.5), Some(5.0));
    }

    #[test]
    fn outside_the_key_range_the_nearest_key_holds() {
        let keys = group(
            KeyType::Linear,
            vec![key(1.0, 3.0, 0.0, 0.0), key(2.0, 4.0, 0.0, 0.0)],
        );
        assert_eq!(keys.sample(-5.0), Some(3.0));
        assert_eq!(keys.sample(500.0), Some(4.0));
    }

    fn angle_group(value: f32) -> KeyGroup<f32> {
        KeyGroup {
            interpolation: Some(KeyType::Linear),
            keys: vec![Key {
                time: 0.0,
                value,
                in_tangent: None,
                out_tangent: None,
                tbc: None,
            }],
        }
    }

    /// Three angles compose with z outermost. Reverse them and a rotation on two axes lands
    /// somewhere else entirely, which is the failure this pins down.
    #[test]
    fn xyz_angles_compose_with_z_outermost() {
        let data = NiTransformData {
            num_rotation_keys: 1,
            rotation_type: Some(KeyType::XyzRotation),
            quaternion_keys: Vec::new(),
            xyz_rotations: Some(vec![angle_group(0.3), angle_group(1.1), angle_group(-0.7)]),
            translations: KeyGroup {
                interpolation: None,
                keys: Vec::new(),
            },
            scales: KeyGroup {
                interpolation: None,
                keys: Vec::new(),
            },
        };
        let sampled = Quat::from(data.rotation_at(0.0).unwrap());
        let expected =
            Quat::from_rotation_z(-0.7) * Quat::from_rotation_y(1.1) * Quat::from_rotation_x(0.3);
        assert!(
            sampled.dot(expected).abs() > 0.9999,
            "{sampled} vs {expected}"
        );

        let reversed =
            Quat::from_rotation_x(0.3) * Quat::from_rotation_y(1.1) * Quat::from_rotation_z(-0.7);
        assert!(
            sampled.dot(reversed).abs() < 0.99,
            "the two orders have to differ, or the test proves nothing"
        );
    }

    #[test]
    fn a_bool_track_holds_each_key_until_the_next() {
        let keys = KeyGroup {
            interpolation: Some(KeyType::Const),
            keys: vec![
                Key {
                    time: 0.0,
                    value: 1u8,
                    in_tangent: None,
                    out_tangent: None,
                    tbc: None,
                },
                Key {
                    time: 0.1,
                    value: 0u8,
                    in_tangent: None,
                    out_tangent: None,
                    tbc: None,
                },
            ],
        };
        assert_eq!(keys.step(-1.0), Some(1));
        assert_eq!(keys.step(0.0), Some(1));
        assert_eq!(keys.step(0.099), Some(1));
        assert_eq!(keys.step(0.1), Some(0));
        assert_eq!(keys.step(50.0), Some(0));
    }

    #[test]
    fn an_empty_group_supplies_nothing() {
        assert_eq!(group(KeyType::Linear, Vec::new()).sample(0.0), None);
    }

    #[test]
    fn a_single_key_is_a_constant() {
        let keys = group(KeyType::Quadratic, vec![key(7.0, 2.5, 1.0, 1.0)]);
        assert_eq!(keys.sample(0.0), Some(2.5));
        assert_eq!(keys.sample(7.0), Some(2.5));
        assert_eq!(keys.sample(99.0), Some(2.5));
    }

    #[test]
    fn an_absent_channel_is_not_a_value() {
        let transform = NiQuatTransform {
            translation: Vector3 {
                x: -f32::MAX,
                y: -f32::MAX,
                z: -f32::MAX,
            },
            rotation: Quaternion {
                w: -0.9848942,
                x: -0.17299117,
                y: 0.0075095175,
                z: -0.0009886987,
            },
            scale: -f32::MAX,
        };
        let pose = transform.pose();
        assert_eq!(pose.translation, None);
        assert_eq!(pose.scale, None);
        assert!(pose.rotation.is_some());
    }

    #[test]
    fn an_absent_channel_keeps_the_objects_own_value() {
        let base = NiTransform {
            rotation: Matrix33::IDENTITY,
            translation: Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            scale: 4.0,
        };
        let pose = Pose {
            rotation: Some(Quat::from_rotation_z(0.5).into()),
            translation: None,
            scale: None,
        };
        let applied = pose.apply(&base);
        assert_eq!(applied.translation, base.translation);
        assert_eq!(applied.scale, base.scale);
        assert_ne!(applied.rotation, base.rotation);
    }

    #[test]
    fn a_looping_controller_wraps_and_a_clamping_one_does_not() {
        let mut controller = NiTimeController {
            next_controller_ref: crate::common::BlockRef::None,
            flags: 0x0008,
            frequency: 1.0,
            phase: 0.0,
            start_time: 0.0,
            end_time: 4.0,
            target_ref: crate::common::BlockRef::None,
        };
        assert_eq!(controller.cycle_type_enum(), CycleType::Loop);
        assert_eq!(controller.local_time(5.0), 1.0);
        assert_eq!(controller.local_time(-1.0), 3.0);

        // cycle type sits in bits 1 and 2
        controller.flags = 0x0008 | (2 << 1);
        assert_eq!(controller.cycle_type_enum(), CycleType::Clamp);
        assert_eq!(controller.local_time(5.0), 4.0);
        assert_eq!(controller.local_time(-1.0), 0.0);
    }
}
