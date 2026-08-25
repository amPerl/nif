use glam::{Mat3, Quat, Vec3};

use crate::anim::Pose;
use crate::blocks::{Block, NiPathInterpolator};
use crate::common::{Key, KeyType, Vector3};

/// Where a path interpolator puts its target at `time`, and which way it is facing.
///
/// A path is two tracks rather than one: a curve through the `NiPosData` control points, and a
/// percent track saying how far along that curve to be. The percent is a fraction of the whole
/// path, so where the control points bunch up the object does not slow down.
pub(crate) fn pose_at(
    blocks: &[Block],
    interpolator: &NiPathInterpolator,
    time: f32,
) -> Option<Pose> {
    let Block::NiPosData(path) = interpolator.path_data_ref.get(blocks)? else {
        return None;
    };
    let Block::NiFloatData(percent) = interpolator.percent_data_ref.get(blocks)? else {
        return None;
    };
    let keys = &path.data.keys;
    if keys.len() < 2 {
        return None;
    }
    let segments: Vec<Segment> = (0..keys.len() - 1)
        .map(|at| Segment::between(keys, at, path.data.interpolation))
        .collect();

    let percent = percent.data.sample(time)?;
    let (at, t) = match interpolator.constant_velocity() {
        true => by_distance(&segments, percent),
        false => by_parameter(keys, percent),
    }?;
    let segment = segments.get(at)?;

    Some(Pose {
        translation: Some(vector(segment.at(t))),
        // without follow the object keeps whatever it is already facing and only slides
        rotation: interpolator
            .follow()
            .then(|| facing(segment.speed(t)))
            .flatten()
            .map(Into::into),
        scale: None,
    })
}

/// One span of the curve, as the cubic the engine reduces a pair of keys to. Written this way
/// rather than as a Hermite basis because the derivative is needed as often as the value is,
/// and here it falls out of the same coefficients.
///
/// `at(t) = x0 + (t0 + (a + b t) t) t`, for `t` running 0 to 1 across the span.
struct Segment {
    x0: Vec3,
    t0: Vec3,
    a: Vec3,
    b: Vec3,
}

impl Segment {
    fn between(keys: &[Key<Vector3>], at: usize, interpolation: Option<KeyType>) -> Segment {
        let x0 = keys.get(at).map(|k| Vec3::from(&k.value)).unwrap_or_default();
        let x1 = keys
            .get(at + 1)
            .map(|k| Vec3::from(&k.value))
            .unwrap_or_default();
        let delta = x1 - x0;

        // a straight span is the same cubic with its curvature terms at zero, which keeps one
        // derivative rather than one per key type
        let (t0, t1) = match interpolation {
            Some(KeyType::Quadratic) => (
                keys.get(at)
                    .and_then(|k| k.out_tangent.as_ref())
                    .map(Vec3::from)
                    .unwrap_or(delta),
                keys.get(at + 1)
                    .and_then(|k| k.in_tangent.as_ref())
                    .map(Vec3::from)
                    .unwrap_or(delta),
            ),
            _ => (delta, delta),
        };

        Segment {
            x0,
            t0,
            a: 3.0 * delta - (t1 + 2.0 * t0),
            b: t0 + t1 - 2.0 * delta,
        }
    }

    fn at(&self, t: f32) -> Vec3 {
        self.x0 + (self.t0 + (self.a + self.b * t) * t) * t
    }

    fn speed(&self, t: f32) -> Vec3 {
        self.t0 + (2.0 * self.a + self.b * (3.0 * t)) * t
    }

    /// How long the span is from its start up to `t`, by five point Gauss-Legendre quadrature
    /// of the speed. The roots are the Legendre roots mapped onto the unit interval and the
    /// weights halved to match, which is the engine's own table.
    fn length(&self, upto: f32) -> f32 {
        // the engine's own digits, kept rather than trimmed to f32
        #[allow(clippy::excessive_precision)]
        const ROOTS: [f32; 5] = [
            0.046910077,
            0.230765345,
            0.5,
            0.769234655,
            0.953089922,
        ];
        #[allow(clippy::excessive_precision)]
        const WEIGHTS: [f32; 5] = [
            0.118463442,
            0.239314335,
            0.284444444,
            0.239314335,
            0.118463442,
        ];
        let sum: f32 = ROOTS
            .iter()
            .zip(WEIGHTS)
            .map(|(root, weight)| weight * self.speed(upto * root).length())
            .sum();
        sum * upto
    }
}

/// The span and the place in it that `percent` of the path's whole length falls at.
///
/// Arc length has no closed form for a cubic, so the length up to a point is integrated and
/// then inverted by Newton's method. That is what `constant_velocity` costs, and it is what
/// keeps an object's speed even where its control points are sparse.
fn by_distance(segments: &[Segment], percent: f32) -> Option<(usize, f32)> {
    let last = segments.len().checked_sub(1)?;
    if percent == 0.0 {
        return Some((0, 0.0));
    }
    if percent == 1.0 {
        return Some((last, 1.0));
    }
    // a path loops, so a percent outside the unit interval wraps into it
    let percent = percent.rem_euclid(1.0);

    let mut partial = Vec::with_capacity(segments.len() + 1);
    partial.push(0.0f32);
    for segment in segments {
        let total = partial.last().copied().unwrap_or(0.0);
        partial.push(total + segment.length(1.0));
    }
    let total = partial.last().copied().unwrap_or(0.0);
    if total <= 0.0 {
        return Some((0, 0.0));
    }

    let target = percent * total;
    let at = partial
        .iter()
        .skip(1)
        .position(|reached| target <= *reached)
        .unwrap_or(last)
        .min(last);
    let segment = segments.get(at)?;
    let along = target - partial[at];
    let span = partial[at + 1] - partial[at];
    if span <= 0.0 {
        return Some((at, 0.0));
    }

    let mut t = along / span;
    for _ in 0..32 {
        let difference = segment.length(t) - along;
        if difference.abs() <= 1e-4 {
            break;
        }
        let speed = segment.speed(t).length();
        if speed <= f32::MIN_POSITIVE {
            break;
        }
        t -= difference / speed;
    }
    Some((at, t.clamp(0.0, 1.0)))
}

/// The span `percent` falls in when the path is read by its keys' own times rather than by
/// distance. Transcribed from the engine and not exercised by this game, where every path asks
/// for constant velocity.
fn by_parameter(keys: &[Key<Vector3>], percent: f32) -> Option<(usize, f32)> {
    let last = keys.len().checked_sub(2)?;
    let first = keys.first()?;
    if percent <= first.time {
        return Some((0, 0.0));
    }
    if percent >= keys.last()?.time {
        return Some((last, 1.0));
    }
    let at = keys
        .windows(2)
        .position(|pair| percent >= pair[0].time && percent < pair[1].time)
        .unwrap_or(last)
        .min(last);
    let span = keys[at + 1].time - keys[at].time;
    match span > 0.0 {
        true => Some((at, (percent - keys[at].time) / span)),
        false => Some((at, 0.0)),
    }
}

/// The turn that puts an object's own forward along the path.
///
/// Forward is the tangent, up is what is left of world up once the tangent is taken out of it,
/// and right closes the set. A path running straight up has no such up, and there the object
/// keeps whatever it was facing rather than being handed a degenerate frame.
fn facing(speed: Vec3) -> Option<Quat> {
    let forward = speed.normalize_or_zero();
    if forward == Vec3::ZERO {
        return None;
    }
    let up = Vec3::Z.cross(forward).normalize_or_zero();
    if up == Vec3::ZERO {
        return None;
    }
    Some(Quat::from_mat3(&Mat3::from_cols(
        forward,
        up,
        forward.cross(up),
    )))
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
    use crate::common::KeyGroup;

    fn key(time: f32, value: Vector3) -> Key<Vector3> {
        Key {
            time,
            value,
            in_tangent: None,
            out_tangent: None,
            tbc: None,
        }
    }

    fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Four evenly spaced points along x, read as straight spans. A quarter of the way along the
    /// whole path is a quarter of the way along its total length, which is the property constant
    /// velocity exists to give.
    fn straight() -> Vec<Segment> {
        let keys = vec![
            key(0.0, vec3(0.0, 0.0, 0.0)),
            key(1.0, vec3(1.0, 0.0, 0.0)),
            key(2.0, vec3(2.0, 0.0, 0.0)),
            key(3.0, vec3(3.0, 0.0, 0.0)),
        ];
        (0..keys.len() - 1)
            .map(|at| Segment::between(&keys, at, Some(KeyType::Linear)))
            .collect()
    }

    #[test]
    fn a_span_runs_between_the_keys_it_joins() {
        let segments = straight();
        assert_eq!(segments[0].at(0.0), Vec3::ZERO);
        assert_eq!(segments[0].at(1.0), Vec3::X);
        assert_eq!(segments[2].at(0.5), Vec3::new(2.5, 0.0, 0.0));
    }

    /// The engine integrates the speed rather than measuring the chord, so a straight span whose
    /// length is known exactly is the check that the quadrature and its weights are right.
    #[test]
    fn a_spans_length_is_its_own_arc_length() {
        let segments = straight();
        assert!((segments[0].length(1.0) - 1.0).abs() < 1e-4);
        assert!((segments[0].length(0.5) - 0.5).abs() < 1e-4);
    }

    #[test]
    fn constant_velocity_puts_a_percent_at_that_much_of_the_length() {
        let segments = straight();
        for (percent, expected) in [(0.0, 0.0), (0.25, 0.75), (0.5, 1.5), (0.75, 2.25)] {
            let (at, t) = super::by_distance(&segments, percent).expect("a path with spans");
            let along = segments[at].at(t).x;
            assert!(
                (along - expected).abs() < 1e-3,
                "percent {percent} reached {along}, wanted {expected}"
            );
        }
        // the far end is its own case, since the wrap would otherwise take it back to zero
        let (at, t) = super::by_distance(&segments, 1.0).expect("a path with spans");
        assert!((segments[at].at(t).x - 3.0).abs() < 1e-3);
    }

    /// Bunched control points are why this is not a lerp over the key index. The second span
    /// here is a tenth the length of the first.
    #[test]
    fn an_uneven_path_still_moves_at_one_speed() {
        let keys = vec![
            key(0.0, vec3(0.0, 0.0, 0.0)),
            key(1.0, vec3(10.0, 0.0, 0.0)),
            key(2.0, vec3(11.0, 0.0, 0.0)),
        ];
        let segments: Vec<Segment> = (0..keys.len() - 1)
            .map(|at| Segment::between(&keys, at, Some(KeyType::Linear)))
            .collect();

        // halfway by length is 5.5 along, which is still inside the long first span
        let (at, t) = super::by_distance(&segments, 0.5).expect("a path with spans");
        assert_eq!(at, 0);
        assert!((segments[at].at(t).x - 5.5).abs() < 1e-3);
    }

    /// A percent past the end wraps rather than clamping, since a path is a loop.
    #[test]
    fn a_percent_beyond_the_path_wraps_into_it() {
        let segments = straight();
        let (at, t) = super::by_distance(&segments, 1.25).expect("a path with spans");
        assert!((segments[at].at(t).x - 0.75).abs() < 1e-3);
        let (at, t) = super::by_distance(&segments, -0.75).expect("a path with spans");
        assert!((segments[at].at(t).x - 0.75).abs() < 1e-3);
    }

    /// Follow puts the object's own forward along the path. Reading the frame's columns the
    /// other way round leaves it facing across its own travel.
    #[test]
    fn following_points_forward_along_the_path() {
        let turn = facing(Vec3::X).expect("a tangent that is not straight up");
        assert!((turn * Vec3::X - Vec3::X).length() < 1e-5);

        let turn = facing(Vec3::Y).expect("a tangent that is not straight up");
        assert!(
            (turn * Vec3::X - Vec3::Y).length() < 1e-5,
            "forward went to {:?}",
            turn * Vec3::X
        );
        // and the frame stays a rotation rather than a reflection
        assert!((turn * Vec3::X).cross(turn * Vec3::Y).dot(turn * Vec3::Z) > 0.0);
    }

    /// A path running straight up has no world up left to build a frame from, so the object
    /// keeps what it was facing instead of being handed something degenerate.
    #[test]
    fn a_path_straight_up_supplies_no_facing() {
        assert!(facing(Vec3::Z).is_none());
        assert!(facing(Vec3::ZERO).is_none());
    }

    /// A quadratic span leaves on the first key's out tangent and arrives on the second key's
    /// in tangent, the same reading the time based sampler uses.
    #[test]
    fn a_quadratic_span_uses_the_tangents_the_file_stores() {
        let mut keys = vec![
            key(0.0, vec3(0.0, 0.0, 0.0)),
            key(1.0, vec3(1.0, 0.0, 0.0)),
        ];
        keys[0].out_tangent = Some(vec3(0.0, 2.0, 0.0));
        keys[1].in_tangent = Some(vec3(0.0, 2.0, 0.0));
        let segment = Segment::between(&keys, 0, Some(KeyType::Quadratic));

        // it still joins its two keys
        assert!((segment.at(0.0) - Vec3::ZERO).length() < 1e-6);
        assert!((segment.at(1.0) - Vec3::X).length() < 1e-6);
        // and it leaves along the tangent it was given rather than straight at the next key
        assert!((segment.speed(0.0) - Vec3::new(0.0, 2.0, 0.0)).length() < 1e-6);
        assert!((segment.speed(1.0) - Vec3::new(0.0, 2.0, 0.0)).length() < 1e-6);
        // so it leaves the straight line between them, swinging out and back rather than
        // bulging: both tangents point the same way, so what it gains early it gives back
        assert!(segment.at(0.25).y > 0.1, "{:?}", segment.at(0.25));
        assert!(segment.at(0.75).y < -0.1, "{:?}", segment.at(0.75));
        assert!(segment.at(0.5).y.abs() < 1e-6);
    }

    /// The percent track is what a path is read by, and it is sampled like any other float
    /// track, so a two key ramp puts the object a quarter of the way along at a quarter of its
    /// span.
    #[test]
    fn the_percent_track_is_an_ordinary_float_track() {
        let ramp: KeyGroup<f32> = KeyGroup {
            interpolation: Some(KeyType::Linear),
            keys: vec![
                Key {
                    time: 0.0,
                    value: 0.0,
                    in_tangent: None,
                    out_tangent: None,
                    tbc: None,
                },
                Key {
                    time: 4.0,
                    value: 1.0,
                    in_tangent: None,
                    out_tangent: None,
                    tbc: None,
                },
            ],
        };
        assert_eq!(ramp.sample(1.0), Some(0.25));
        assert_eq!(ramp.sample(3.0), Some(0.75));
    }
}
