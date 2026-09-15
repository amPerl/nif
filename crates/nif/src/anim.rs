use glam::{Mat3, Quat, Vec3};

// Following a controller chain is a walk of the graph like any other and needs none of the
// maths this module is gated on, so it lives with the walks. Named here as it always was.
pub use crate::walk::controllers;

use crate::blocks::{
    Block, GeomMorpherFlags, LookAxis, MaterialColor, NiAvObject, NiGeometry, NiLookAtInterpolator,
    NiMaterialProperty, NiObjectNET, NiTexturingProperty, NiTimeController, NiTransformData,
    NiTransformInterpolator, TextureSlot, TextureTransform,
};
use crate::common::{
    BlockRef, Color4, Key, KeyGroup, KeyType, NiQuatTransform, NiTransform, Quaternion, Triangle,
    Vector3,
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
/// Whether anything could ever move this object, whatever the moment.
///
/// `transform_at` returns None for a node no controller drives, and it has to walk the chain and
/// look at every one to find that out. Whether the chain holds an active transform controller at
/// all does not change while a file is loaded, so a caller asking the same node about thousands of
/// copies a frame can ask this once instead. False is a promise that `transform_at` is None at
/// every time; true only means it is worth asking.
pub fn drives_transform(blocks: &[Block], object: &NiAvObject) -> bool {
    controllers(blocks, object.controller_ref).any(|block| {
        matches!(block, Block::NiTransformController(_))
            && block
                .as_time_controller()
                .is_some_and(NiTimeController::is_active)
    })
}

/// A node can carry several at once, and all of them run.
///
/// `NiTransformController::Update` writes only the channels its interpolator says are valid and
/// leaves the target's own alone, and `NiAVObject::UpdateObjectControllers` walks the whole chain
/// from the head calling Update on each. So four controllers on one node are four writes to the
/// same three channels, and the last to write a channel is the one that decides it.
///
/// Reading only the first is what made a roof item ten times its size: `i_discgem` and `i_lizard`
/// hang four and two controllers off one node, and the first of them carries a scale of ten that
/// a later one puts back to one.
pub fn transform_at(blocks: &[Block], object: &NiAvObject, time: f32) -> Option<NiTransform> {
    let mut found: Option<Pose> = None;
    for block in controllers(blocks, object.controller_ref) {
        let Block::NiTransformController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active() {
            continue;
        }
        let local = time_controller.local_time(time);
        // one controller carries either a transform track or a path to run along
        let pose = match controller.base.interpolator_ref.get(blocks) {
            Some(Block::NiTransformInterpolator(interpolator)) => {
                interpolator.sample(blocks, local)
            }
            Some(Block::NiPathInterpolator(interpolator)) => {
                match crate::path::pose_at(blocks, interpolator, local) {
                    Some(pose) => pose,
                    None => continue,
                }
            }
            // a look at aims the object at something else, which needs both of them placed
            // first, so only what it carries itself comes out here
            Some(Block::NiLookAtInterpolator(interpolator)) => {
                let (translation, scale, _) = look_at_parts(blocks, interpolator, local);
                Pose {
                    rotation: None,
                    translation,
                    scale,
                }
            }
            _ => continue,
        };
        if pose.is_empty() {
            continue;
        }
        // This one's channels over whatever the chain has said so far, which is the order the
        // engine writes them in.
        found = Some(match found {
            Some(held) => pose.or(held),
            None => pose,
        });
    }
    Some(found?.apply(&NiTransform::from(object)))
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

    /// `a * wa + b * wb`. A tension, continuity and bias tangent is a weighted sum of the two
    /// differences either side of a key, and this is the only arithmetic that needs.
    fn weighted(a: Self, wa: f32, b: Self, wb: f32) -> Self;

    /// `from - to`, the difference a tangent is built out of.
    fn difference(from: Self, to: Self) -> Self;
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

    fn weighted(a: Self, wa: f32, b: Self, wb: f32) -> Self {
        a * wa + b * wb
    }

    fn difference(from: Self, to: Self) -> Self {
        from - to
    }
}

impl Interpolate for Color4 {
    fn lerp(from: Self, to: Self, t: f32) -> Self {
        Color4 {
            r: f32::lerp(from.r, to.r, t),
            g: f32::lerp(from.g, to.g, t),
            b: f32::lerp(from.b, to.b, t),
            a: f32::lerp(from.a, to.a, t),
        }
    }

    fn hermite(from: Self, out_of_from: Self, to: Self, into_to: Self, t: f32) -> Self {
        Color4 {
            r: f32::hermite(from.r, out_of_from.r, to.r, into_to.r, t),
            g: f32::hermite(from.g, out_of_from.g, to.g, into_to.g, t),
            b: f32::hermite(from.b, out_of_from.b, to.b, into_to.b, t),
            a: f32::hermite(from.a, out_of_from.a, to.a, into_to.a, t),
        }
    }

    fn weighted(a: Self, wa: f32, b: Self, wb: f32) -> Self {
        Color4 {
            r: a.r * wa + b.r * wb,
            g: a.g * wa + b.g * wb,
            b: a.b * wa + b.b * wb,
            a: a.a * wa + b.a * wb,
        }
    }

    fn difference(from: Self, to: Self) -> Self {
        Color4 {
            r: from.r - to.r,
            g: from.g - to.g,
            b: from.b - to.b,
            a: from.a - to.a,
        }
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

    fn weighted(a: Self, wa: f32, b: Self, wb: f32) -> Self {
        Vector3 {
            x: a.x * wa + b.x * wb,
            y: a.y * wa + b.y * wb,
            z: a.z * wa + b.z * wb,
        }
    }

    fn difference(from: Self, to: Self) -> Self {
        Vector3 {
            x: from.x - to.x,
            y: from.y - to.y,
            z: from.z - to.z,
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

/// The value a controller drives one of an object's float extra data to at `time`. None where
/// nothing animates that attribute, so the caller keeps whatever the extra data block stores.
///
/// A shader attribute is bound from extra data by name, and a `NiFloatExtraDataController` names
/// the same attribute and replaces its value over time. So a shape whose attribute animates has
/// to be read per frame rather than once when it is loaded.
pub fn float_extra_data_at(
    blocks: &[Block],
    object: &NiAvObject,
    attribute: &str,
    time: f32,
) -> Option<f32> {
    for block in blocks {
        let Block::NiFloatExtraDataController(controller) = block else {
            continue;
        };
        if controller.extra_data_name.as_bytes() != attribute.as_bytes() {
            continue;
        }
        // a controller names its target, and one file can drive the same attribute on several
        let names_object = controller
            .target_ref
            .get(blocks)
            .and_then(Block::av_object)
            .is_some_and(|target| std::ptr::eq(target, object));
        if !names_object {
            continue;
        }
        let Some(Block::NiFloatInterpolator(interpolator)) =
            controller.interpolator_ref.get(blocks)
        else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        return match interpolator.data_ref.get(blocks) {
            Some(Block::NiFloatData(data)) => data.data.sample(time_controller.local_time(time)),
            _ => Some(interpolator.value),
        };
    }
    None
}

/// The vertex positions a geometry morpher leaves a shape at, or None where nothing morphs it.
///
/// A morph is a weighted sum of whole targets, starting from nothing, rather than a base
/// shape with offsets added to it. Each target carries a full set of vectors and its own weight
/// track, and the result replaces the geometry's stored vertices outright.
///
/// Where the data says its targets are relative, target 0 is the base and its weight is pinned
/// at 1 whatever its track says: the rest are then deltas that sum on top of it. Reading the
/// first target's track instead would fade the whole shape toward the origin.
///
/// A weight under a thousandth is skipped, which is what the engine does.
pub fn morph_at(blocks: &[Block], geometry: &NiAvObject, time: f32) -> Option<Vec<Vector3>> {
    for block in controllers(blocks, geometry.controller_ref) {
        let Block::NiGeomMorpherController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active() {
            continue;
        }
        // the controller's own clock, so a morpher that loops comes round again instead of
        // holding its last key for the rest of the file
        let time = time_controller.local_time(time);
        let Some(Block::NiMorphData(data)) = controller.data_ref.get(blocks) else {
            continue;
        };
        let vertices = data.num_vertices as usize;
        if vertices == 0 || data.morphs.is_empty() {
            continue;
        }

        let relative = data.relative_targets != 0;
        let mut out = vec![
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            vertices
        ];

        for (at, morph) in data.morphs.iter().enumerate() {
            let weight = if at == 0 && relative {
                1.0
            } else {
                let interpolator = controller
                    .interpolator_refs
                    .get(at)
                    .and_then(|reference| reference.get(blocks));
                match interpolator {
                    Some(Block::NiFloatInterpolator(interpolator)) => {
                        match interpolator.data_ref.get(blocks) {
                            Some(Block::NiFloatData(keys)) => {
                                keys.data.sample(time).unwrap_or(interpolator.value)
                            }
                            // posed rather than keyed, so it holds one weight throughout
                            _ => interpolator.value,
                        }
                    }
                    // a target with no interpolator keeps the weight the file stores for it
                    _ => morph.legacy_weight,
                }
            };
            if weight.abs() < 0.001 {
                continue;
            }
            for (vertex, vector) in morph.vectors.iter().enumerate().take(vertices) {
                out[vertex].x += vector.x * weight;
                out[vertex].y += vector.y * weight;
                out[vertex].z += vector.z * weight;
            }
        }
        return Some(out);
    }
    None
}

/// The normals a morphed shape draws with, given the vertices `morph_at` moved it to. `None`
/// where its morpher does not ask for them, where the shape stores none of its own, or where
/// the shape carries no triangles to derive them from.
///
/// Every face contributes its own unit normal to each of the three vertices it touches, and the
/// sums are normalised at the end. A face counts once however large it is, which is the engine's
/// own weighting and not the area weighted one a reference would give.
pub fn morph_normals(
    blocks: &[Block],
    geometry: &NiGeometry,
    moved: &[Vector3],
) -> Option<Vec<Vector3>> {
    if !updates_normals(blocks, geometry) {
        return None;
    }
    match geometry.data_ref.get(blocks)? {
        Block::NiTriShapeData(data) => {
            data.base.base.normals.as_ref()?;
            Some(face_normals(moved, data.triangles.as_ref()?.iter().cloned()))
        }
        Block::NiTriShapeDynamicData(data) => {
            data.base.base.base.normals.as_ref()?;
            Some(face_normals(
                moved,
                data.base.triangles.as_ref()?.iter().cloned(),
            ))
        }
        Block::NiTriStripsData(data) => {
            data.base.base.normals.as_ref()?;
            Some(face_normals(moved, data.triangles()))
        }
        _ => None,
    }
}

/// Whether an active morpher on this object asks for the normals to be recalculated as it runs.
fn updates_normals(blocks: &[Block], geometry: &NiGeometry) -> bool {
    controllers(blocks, geometry.controller_ref).any(|block| {
        matches!(
            block,
            Block::NiGeomMorpherController(controller)
                if controller.is_active()
                    && matches!(
                        controller.morpher_flags,
                        GeomMorpherFlags::UpdateNormalsEnabled
                    )
        )
    })
}

fn face_normals(moved: &[Vector3], triangles: impl Iterator<Item = Triangle>) -> Vec<Vector3> {
    let mut out = vec![Vec3::ZERO; moved.len()];
    for triangle in triangles {
        let (Some(a), Some(b), Some(c)) = (
            moved.get(triangle.a as usize).map(Vec3::from),
            moved.get(triangle.b as usize).map(Vec3::from),
            moved.get(triangle.c as usize).map(Vec3::from),
        ) else {
            continue;
        };
        let face = (b - a).cross(c - b);
        // a degenerate face has no direction to contribute, and normalising it gives NaN
        if face.length_squared() <= f32::MIN_POSITIVE {
            continue;
        }
        let face = face.normalize();
        out[triangle.a as usize] += face;
        out[triangle.b as usize] += face;
        out[triangle.c as usize] += face;
    }

    out.into_iter()
        .map(|normal| {
            let unit = normal.normalize_or_zero();
            Vector3 {
                x: unit.x,
                y: unit.y,
                z: unit.z,
            }
        })
        .collect()
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

/// The world rotation that aims `from` at `to`, as a look at interpolator builds it.
///
/// The chosen axis points along the line between them, world up fills the second axis with
/// whatever is left of it once the aim is taken out, and the third closes the set. Where the aim
/// is within a thousandth of straight up there is no world up left to use, and world y stands in.
///
/// **The aim is reversed when `flip` is clear**, not when it is set, which reads backwards until
/// you notice that an object faces along its own negative axis. Every look at in this game leaves
/// it clear.
pub fn look_at_rotation(
    from: Vec3,
    to: Vec3,
    flip: bool,
    axis: LookAxis,
    roll: f32,
) -> Option<crate::common::Matrix33> {
    let offset = to - from;
    if offset.length_squared() < 0.001 {
        return None;
    }
    let mut aim = offset.normalize();

    // straight up leaves nothing of world up to build on, so world y takes its place
    let (mut up, along) = match aim.z.abs() > 0.999 {
        true => (Vec3::Y, aim.y),
        false => (Vec3::Z, aim.z),
    };
    up = (up - aim * along).normalize_or_zero();
    if up == Vec3::ZERO {
        return None;
    }
    if !flip {
        aim = -aim;
    }
    let right = up.cross(aim);

    let basis = match axis {
        LookAxis::X => Mat3::from_cols(aim, up, -right),
        LookAxis::Y => Mat3::from_cols(right, aim, -up),
        LookAxis::Z => Mat3::from_cols(right, up, aim),
    };
    // the roll turns about the object's own z after the aim, and the engine negates it
    Some((basis * Mat3::from_rotation_z(-roll)).into())
}

/// The translation, scale and roll a look at interpolator carries of its own. Its rotation is not
/// here: that needs where the object and its target have ended up, which only a traversal knows.
pub fn look_at_parts(
    blocks: &[Block],
    interpolator: &NiLookAtInterpolator,
    time: f32,
) -> (Option<Vector3>, Option<f32>, f32) {
    let float_at = |reference: BlockRef| -> Option<f32> {
        match reference.get(blocks)? {
            Block::NiFloatInterpolator(interpolator) => match interpolator.data_ref.get(blocks) {
                Some(Block::NiFloatData(data)) => data.data.sample(time).or(Some(interpolator.value)),
                _ => Some(interpolator.value),
            },
            _ => None,
        }
    };
    let translation = match interpolator.interpolator_translation.get(blocks) {
        Some(Block::NiPoint3Interpolator(point)) => match point.data_ref.get(blocks) {
            Some(Block::NiPosData(data)) => data.data.sample(time).or(Some(point.value)),
            _ => Some(point.value),
        },
        _ => None,
    };
    (
        translation,
        float_at(interpolator.interpolator_scale),
        float_at(interpolator.interpolator_roll).unwrap_or(0.0),
    )
}

/// The dimmer a light is turned down to at `time`. None where nothing drives it, so the caller
/// keeps whatever the light itself stores.
///
/// The dimmer is not a term of its own: it scales the light's three colours, so a light driven
/// to zero goes out rather than turning black against what it was.
/// The material channel a controller drives on `material`, and its value at `time`.
///
/// Only one channel is driven per controller and only two are ever driven in this game, ambient
/// and self illumination, so a caller gets the pair and decides which of its own channels to
/// replace. The interpolator's own value stands in where its data has no key at this time.
pub fn material_color_at(
    blocks: &[Block],
    material: &NiObjectNET,
    time: f32,
) -> Option<(MaterialColor, Vector3)> {
    for block in controllers(blocks, material.controller_ref) {
        let Block::NiMaterialColorController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active() {
            continue;
        }
        let Some(Block::NiPoint3Interpolator(interpolator)) =
            controller.base.interpolator_ref.get(blocks)
        else {
            continue;
        };
        let keyed = match interpolator.data_ref.get(blocks) {
            Some(Block::NiPosData(data)) => data.data.sample(time_controller.local_time(time)),
            _ => None,
        };
        let value = keyed.unwrap_or(interpolator.value);
        return Some((controller.target_color, value));
    }
    None
}

pub fn dimmer_at(blocks: &[Block], light: &NiAvObject, time: f32) -> Option<f32> {
    for block in controllers(blocks, light.controller_ref) {
        let Block::NiLightDimmerController(controller) = block else {
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
        return Some(keyed.unwrap_or(interpolator.value));
    }
    None
}

/// The uv transform in force on one texture slot at `time`. None when the slot carries no
/// transform of its own and nothing animates it, so the caller can leave its uvs alone.
///
/// Unlike the other channels this accumulates: a property is driven by one controller per member,
/// so five of them can share a target. Each replaces its member rather than scaling it, and a
/// slot with no stored transform starts from the identity the engine substitutes.
pub fn texture_transform_at(
    blocks: &[Block],
    property: &NiTexturingProperty,
    slot: TextureSlot,
    time: f32,
) -> Option<TextureTransform> {
    let stored = property
        .texture(slot)
        .and_then(|desc| desc.transform.get())
        .cloned();
    let mut animated: Option<TextureTransform> = None;

    for block in controllers(blocks, property.controller_ref) {
        let Block::NiTextureTransformController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active() {
            continue;
        }
        let addressed = if controller.shader_map {
            Some(TextureSlot::Shader(controller.texture_slot))
        } else {
            TextureSlot::from_index(controller.texture_slot)
        };
        if addressed != Some(slot) {
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
        let value = match keyed {
            Some(value) => value,
            None if interpolator.value != INVALID => interpolator.value,
            None => continue,
        };

        let transform = animated
            .get_or_insert_with(|| stored.clone().unwrap_or_else(TextureTransform::identity));
        match controller.operation {
            TRANSLATE_U => transform.translation.u = value,
            TRANSLATE_V => transform.translation.v = value,
            ROTATE => transform.w_rotation = value,
            SCALE_U => transform.tiling.u = value,
            SCALE_V => transform.tiling.v = value,
            _ => {}
        }
    }

    animated.or(stored)
}

/// The sources a flip controller can swap into a slot, in the order it steps through them.
///
/// Empty where nothing flips that slot. The list is what the controller holds rather than what
/// it shows at a moment, so it is asked once and stands for the whole animation.
pub fn flip_frames(
    blocks: &[Block],
    property: &NiTexturingProperty,
    slot: TextureSlot,
) -> Vec<BlockRef> {
    for block in controllers(blocks, property.controller_ref) {
        let Block::NiFlipController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active()
            || TextureSlot::from_flip_index(controller.texture_slot) != Some(slot)
        {
            continue;
        }
        return controller.source_refs.clone();
    }
    Vec::new()
}

/// The source texture a flip controller has swapped into a slot at `time`. None when nothing
/// flips that slot, so the caller keeps whatever source the map itself names.
///
/// The track holds a frame index rather than a rate. The engine nudges it by 0.01 before
/// truncating, so a key sitting just under its own frame still selects it, and clamps to the
/// last source. Only the source is replaced: the map keeps its own transform, clamp and filter
/// modes.
pub fn flip_source_at(
    blocks: &[Block],
    property: &NiTexturingProperty,
    slot: TextureSlot,
    time: f32,
) -> Option<BlockRef> {
    for block in controllers(blocks, property.controller_ref) {
        let Block::NiFlipController(controller) = block else {
            continue;
        };
        let time_controller: &NiTimeController = controller;
        if !time_controller.is_active()
            || TextureSlot::from_flip_index(controller.texture_slot) != Some(slot)
        {
            continue;
        }
        let Some(last) = controller.source_refs.len().checked_sub(1) else {
            continue;
        };
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
        let value = match keyed {
            Some(value) => value,
            None if interpolator.value != INVALID => interpolator.value,
            None => continue,
        };
        let frame = ((value + 0.01) as isize).clamp(0, last as isize) as usize;
        return controller.source_refs.get(frame).copied();
    }
    None
}

const TRANSLATE_U: u32 = 0;
const TRANSLATE_V: u32 = 1;
const ROTATE: u32 = 2;
const SCALE_U: u32 = 3;
const SCALE_V: u32 = 4;

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
        Some(KeyType::Tbc) => {
            let out_of_from = tbc_tangents(keys, at).1;
            let into_to = tbc_tangents(keys, at + 1).0;
            Some(T::hermite(from.value, out_of_from, to.value, into_to, t))
        }
        _ => Some(T::lerp(from.value, to.value, t)),
    }
}

/// The pair of tangents a tension, continuity and bias key carries: the one a segment arrives on
/// and the one it leaves on. Kochanek and Bartels, as the engine implements it.
///
/// The two are not the same wherever continuity or bias is non zero, which is the point of the
/// parameterisation: continuity breaks the slope across a key and bias leans it toward one side.
/// Both are then scaled for how far apart the neighbouring keys sit, so uneven spacing does not
/// change the shape of the curve.
///
/// An end key has no neighbour on one side, so the engine mirrors the value it does have and
/// treats both spans as equal, which makes the two tangents there depend only on the one real
/// difference.
fn tbc_tangents<T>(keys: &[Key<T>], at: usize) -> (T, T)
where
    T: Interpolate + Clone + binrw::BinRead + binrw::BinWrite + 'static,
    T: for<'a> binrw::BinRead<Args<'a> = ()>,
    T: for<'a> binrw::BinWrite<Args<'a> = ()>,
{
    let Some(key) = keys.get(at) else {
        let zero = keys
            .first()
            .map(|key| T::difference(key.value, key.value))
            .expect("a sampled group has keys");
        return (zero, zero);
    };
    // a key with no tension, continuity or bias is an ordinary Catmull-Rom one, which is what
    // all three at zero comes to
    let (tension, bias, continuity) = match &key.tbc {
        Some(tbc) => (tbc.tension, tbc.bias, tbc.continuity),
        None => (0.0, 0.0, 0.0),
    };

    // the value before and after, mirrored at an end so the curve leaves it straight
    let (previous, pre_len) = match at.checked_sub(1).and_then(|before| keys.get(before)) {
        Some(before) => (before.value, key.time - before.time),
        None => match keys.get(1) {
            Some(after) => (T::weighted(key.value, 2.0, after.value, -1.0), 1.0),
            None => (key.value, 1.0),
        },
    };
    let (next, next_len) = match keys.get(at + 1) {
        Some(after) => (after.value, after.time - key.time),
        None => match at.checked_sub(1).and_then(|before| keys.get(before)) {
            Some(before) => (T::weighted(key.value, 2.0, before.value, -1.0), 1.0),
            None => (key.value, 1.0),
        },
    };

    let behind = T::difference(key.value, previous);
    let ahead = T::difference(next, key.value);

    let half_tension = 0.5 * (1.0 - tension);
    let with_continuity = half_tension * (1.0 + continuity);
    let against_continuity = half_tension * (1.0 - continuity);
    let with_bias = 1.0 + bias;
    let against_bias = 1.0 - bias;

    let incoming = T::weighted(
        behind,
        against_continuity * with_bias,
        ahead,
        with_continuity * against_bias,
    );
    let outgoing = T::weighted(
        behind,
        with_continuity * with_bias,
        ahead,
        against_continuity * against_bias,
    );

    // uneven spacing is compensated for, approximately
    let spans = pre_len + next_len;
    if spans <= 0.0 {
        return (incoming, outgoing);
    }
    let scale = 2.0 / spans;
    let zero = T::difference(key.value, key.value);
    (
        T::weighted(incoming, pre_len * scale, zero, 0.0),
        T::weighted(outgoing, next_len * scale, zero, 0.0),
    )
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

        // Found by halving rather than by walking, the way the translations and the scales
        // already are. A quaternion key carries its time as an option only because the xyz form
        // leaves both empty, and that form was answered above, so every key here has one and they
        // are in order: a binary search is as sound as the scan was and does not read the whole
        // track to find the one moment asked for. A driven node is sampled for every copy of
        // every model that carries it, which made this one of the hottest reads in a frame.
        let keys = &self.quaternion_keys;
        if keys.len() < 2 {
            return keys.first().and_then(|key| key.value);
        }
        // The first key at or after the moment, so the segment is the pair ending there. Strictly
        // before rather than at or before, because two keys can share a time: the walk took the
        // first of them as the segment's end and halving on `<=` would step past both.
        let at = keys
            .partition_point(|key| key.time.is_some_and(|start| start < time))
            .clamp(1, keys.len() - 1);
        let previous = keys.get(at - 1)?;
        let current = keys.get(at)?;
        let (start, from) = (previous.time?, previous.value.as_ref()?);
        let (end, to) = (current.time?, current.value.as_ref()?);
        let span = end - start;
        if span <= 0.0 {
            return Some(*to);
        }
        let t = ((time - start) / span).clamp(0.0, 1.0);
        Some(Quat::from(from).slerp(to.into(), t).into())
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
pub(crate) mod tests {
    use crate::common::Tbc;

    fn tbc_key(time: f32, value: f32, tension: f32, bias: f32, continuity: f32) -> Key<f32> {
        Key {
            time,
            value,
            in_tangent: None,
            out_tangent: None,
            tbc: Some(Tbc {
                tension,
                bias,
                continuity,
            }),
        }
    }

    /// With tension, continuity and bias all zero, a tension key is an ordinary Catmull-Rom
    /// spline: each tangent is half the span across the key. Worked from the engine's own
    /// arithmetic rather than from a spline reference, since the two differ in the end handling.
    #[test]
    fn a_flat_tbc_key_gives_catmull_rom_tangents() {
        // evenly spaced, so the length compensation is exactly 1
        let keys = vec![
            tbc_key(0.0, 0.0, 0.0, 0.0, 0.0),
            tbc_key(1.0, 1.0, 0.0, 0.0, 0.0),
            tbc_key(2.0, 3.0, 0.0, 0.0, 0.0),
        ];

        // the middle key: behind is 1, ahead is 2, and both tangents are half their sum
        let (incoming, outgoing) = super::tbc_tangents(&keys, 1);
        assert!((incoming - 1.5).abs() < 1e-5, "incoming {incoming}");
        assert!((outgoing - 1.5).abs() < 1e-5, "outgoing {outgoing}");

        // the first key mirrors its missing neighbour, so both differences are the real one
        let (first_in, first_out) = super::tbc_tangents(&keys, 0);
        assert!((first_in - 1.0).abs() < 1e-5, "{first_in}");
        assert!((first_out - 1.0).abs() < 1e-5, "{first_out}");
    }

    /// Continuity is what makes a key a corner: it breaks the slope across it, so the tangent
    /// arriving and the one leaving stop agreeing. A reading that used one tangent for both
    /// would pass every flat test and be wrong for every shaped key.
    #[test]
    fn continuity_breaks_the_slope_across_a_key() {
        let keys = vec![
            tbc_key(0.0, 0.0, 0.0, 0.0, 0.0),
            tbc_key(1.0, 1.0, 0.0, 0.0, 1.0),
            tbc_key(2.0, 3.0, 0.0, 0.0, 0.0),
        ];
        let (incoming, outgoing) = super::tbc_tangents(&keys, 1);
        // continuity 1 takes the incoming tangent entirely from the span ahead and the outgoing
        // entirely from the span behind, which is the two swapping
        assert!((incoming - 2.0).abs() < 1e-5, "incoming {incoming}");
        assert!((outgoing - 1.0).abs() < 1e-5, "outgoing {outgoing}");
    }

    /// Tension flattens a key toward a corner, and at 1 it removes the tangent entirely, which
    /// is the one value that is easy to check against by eye.
    #[test]
    fn full_tension_flattens_a_key() {
        let keys = vec![
            tbc_key(0.0, 0.0, 0.0, 0.0, 0.0),
            tbc_key(1.0, 1.0, 1.0, 0.0, 0.0),
            tbc_key(2.0, 3.0, 0.0, 0.0, 0.0),
        ];
        let (incoming, outgoing) = super::tbc_tangents(&keys, 1);
        assert!(incoming.abs() < 1e-6, "incoming {incoming}");
        assert!(outgoing.abs() < 1e-6, "outgoing {outgoing}");
    }

    /// A tension track sampled as if it were linear is wrong everywhere between its keys.
    #[test]
    fn a_tbc_track_departs_from_the_linear_reading() {
        let keys = vec![
            tbc_key(0.0, 0.0, 0.0, 0.0, 0.0),
            tbc_key(1.0, 1.0, 0.0, 0.0, 0.0),
            tbc_key(2.0, 0.0, 0.0, 0.0, 0.0),
        ];
        let same = || {
            vec![
                tbc_key(0.0, 0.0, 0.0, 0.0, 0.0),
                tbc_key(1.0, 1.0, 0.0, 0.0, 0.0),
                tbc_key(2.0, 0.0, 0.0, 0.0, 0.0),
            ]
        };
        let group = KeyGroup {
            keys,
            interpolation: Some(KeyType::Tbc),
        };
        let linear = KeyGroup {
            keys: same(),
            interpolation: Some(KeyType::Linear),
        };

        // it still passes through every key, which is what makes it an interpolating spline
        for at in [0.0, 1.0, 2.0] {
            let curved = group.sample(at).expect("a value");
            let straight = linear.sample(at).expect("a value");
            assert!((curved - straight).abs() < 1e-5, "at {at}");
        }

        // and between them it does not, because the curve overshoots where the line corners
        let curved = group.sample(0.5).expect("a value");
        let straight = linear.sample(0.5).expect("a value");
        assert!(
            (curved - straight).abs() > 0.05,
            "curved {curved} against straight {straight}"
        );
    }
    use super::*;
    use crate::common::{KeyType, Matrix33};

    /// A file whose only animation is a particle system still has a span, which needs its
    /// controllers to report themselves as time controllers.
    #[test]
    fn a_particle_system_has_a_timeline() {
        let bytes = std::fs::read("tests/20.nif").expect("fixture");
        let nif = crate::Nif::parse(&mut std::io::Cursor::new(&bytes)).expect("parse");

        let particle_controllers = nif
            .blocks
            .iter()
            .filter(|block| block.name().starts_with("NiPSys"))
            .filter(|block| block.as_time_controller().is_some())
            .count();

        assert!(
            particle_controllers > 0,
            "no particle controller reports itself as a time controller"
        );
        assert!(
            span(&nif.blocks).is_some(),
            "a file with particle controllers has no span"
        );
    }

    fn vector(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn time_controller(next: BlockRef) -> crate::blocks::NiTimeController {
        crate::blocks::NiTimeController {
            next_controller_ref: next,
            flags: 0x0008,
            frequency: 1.0,
            phase: 0.0,
            start_time: 0.0,
            end_time: 1.0,
            target_ref: BlockRef::None,
        }
    }

    pub(crate) fn shape_with_skin(skin: BlockRef, data: BlockRef) -> Block {
        let Block::NiTriShape(mut built) = shape(BlockRef::None, data) else {
            unreachable!()
        };
        built.base.skin_instance_ref = skin;
        Block::NiTriShape(built)
    }

    fn shape(controller: BlockRef, data: BlockRef) -> Block {
        Block::NiTriShape(crate::blocks::NiTriShape {
            base: NiGeometry {
                base: NiAvObject {
                    base: crate::blocks::NiObjectNET {
                        name: crate::blocks::NiString::from("shape"),
                        extra_data_refs: Vec::new(),
                        controller_ref: controller,
                    },
                    flags: 0,
                    translation: vector(0.0, 0.0, 0.0),
                    rotation: crate::common::Matrix33::IDENTITY,
                    scale: 1.0,
                    property_refs: Vec::new(),
                    collision_ref: BlockRef::None,
                },
                data_ref: data,
                skin_instance_ref: BlockRef::None,
                material_data: crate::blocks::MaterialData::None,
            },
        })
    }

    pub(crate) fn shape_data(
        vertices: Vec<Vector3>,
        normals: Option<Vec<Vector3>>,
        tris: Vec<Triangle>,
    ) -> Block {
        Block::NiTriShapeData(crate::blocks::NiTriShapeData {
            base: crate::blocks::NiTriBasedGeomData {
                base: crate::blocks::NiGeometryData {
                    group_id: 0,
                    keep_flags: 0,
                    compress_flags: 0,
                    vertices: Some(vertices),
                    data_flags: 0,
                    normals,
                    tangents: None,
                    binormals: None,
                    center: vector(0.0, 0.0, 0.0),
                    radius: 1.0,
                    vertex_colors: None,
                    uv_sets: Vec::new(),
                    consistency_flags: 0,
                    additional_data_ref: BlockRef::None,
                },
                num_triangles: tris.len() as u16,
            },
            triangles: Some(tris),
            match_groups: Vec::new(),
        })
    }

    fn morpher(flags: GeomMorpherFlags) -> Block {
        Block::NiGeomMorpherController(crate::blocks::NiGeomMorpherController {
            base: crate::blocks::NiInterpController {
                base: time_controller(BlockRef::None),
            },
            morpher_flags: flags,
            data_ref: BlockRef::None,
            always_update: 0,
            interpolator_refs: Vec::new(),
        })
    }

    fn float_interpolator(pose: f32, data: BlockRef) -> Block {
        Block::NiFloatInterpolator(crate::blocks::NiFloatInterpolator {
            base: crate::blocks::NiKeyBasedInterpolator {
                base: crate::blocks::NiInterpolator {},
            },
            value: pose,
            data_ref: data,
        })
    }

    fn extra_data_controller(attribute: &str, target: BlockRef, interpolator: BlockRef) -> Block {
        Block::NiFloatExtraDataController(crate::blocks::NiFloatExtraDataController {
            base: crate::blocks::NiExtraDataController {
                base: crate::blocks::NiSingleInterpController {
                    base: crate::blocks::NiInterpController {
                        base: crate::blocks::NiTimeController {
                            target_ref: target,
                            // spanning its own keys, as a file's does
                            end_time: 2.0,
                            ..time_controller(BlockRef::None)
                        },
                    },
                    interpolator_ref: interpolator,
                },
                extra_data_name: crate::blocks::NiString::from(attribute),
            },
        })
    }

    /// A morpher reads its weights on its own clock, not the file's. Nearly every morpher in
    /// this game loops over a span far shorter than the file, so reading the file's clock runs
    /// the morph once and then holds its last key for however long is left.
    #[test]
    fn a_looping_morpher_comes_round_again() {
        let target = |weight| {
            crate::blocks::Morph {
                frame_name: crate::blocks::NiString::from("target"),
                legacy_weight: weight,
                vectors: vec![vector(0.0, 0.0, 0.0), vector(1.0, 0.0, 0.0)],
            }
        };
        let blocks = vec![
            shape(BlockRef::Index(1), BlockRef::None),
            Block::NiGeomMorpherController(crate::blocks::NiGeomMorpherController {
                base: crate::blocks::NiInterpController {
                    base: crate::blocks::NiTimeController {
                        // a short span, as a morpher has, against a file that runs much longer
                        end_time: 1.0,
                        ..time_controller(BlockRef::None)
                    },
                },
                morpher_flags: GeomMorpherFlags::UpdateNormalsDisabled,
                data_ref: BlockRef::Index(2),
                always_update: 0,
                interpolator_refs: vec![BlockRef::None, BlockRef::Index(3)],
            }),
            Block::NiMorphData(crate::blocks::NiMorphData {
                num_vertices: 2,
                relative_targets: 0,
                morphs: vec![target(0.0), target(0.0)],
            }),
            float_interpolator(0.0, BlockRef::Index(4)),
            Block::NiFloatData(crate::blocks::NiFloatData {
                data: group(
                    KeyType::Linear,
                    vec![key(0.0, 0.0, 0.0, 0.0), key(1.0, 1.0, 0.0, 0.0)],
                ),
            }),
        ];
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        let at = |t: f32| morph_at(&blocks, geometry, t).expect("the shape morphs")[1].x;
        // the weight ramps across the span, so the second vertex travels with it
        let quarter = at(0.25);
        assert!(quarter > 0.0 && quarter < 1.0, "mid span gave {quarter}");

        // and the same place in a later pass gives the same answer, rather than the held end
        assert!((at(3.25) - quarter).abs() < 1e-5, "later pass gave {}", at(3.25));
        assert!((at(9.25) - quarter).abs() < 1e-5);
    }

    /// An animated shader attribute is a controller, an interpolator and a key group deep, and
    /// what picks it out of a file is a name and a target rather than a place in the graph.
    /// Verified on a real file when it was built; this is the synthetic case it never got.
    #[test]
    fn a_driven_attribute_follows_its_keys() {
        let blocks = vec![
            shape(BlockRef::None, BlockRef::None),
            extra_data_controller("WarpAlpha", BlockRef::Index(0), BlockRef::Index(2)),
            float_interpolator(0.25, BlockRef::Index(3)),
            Block::NiFloatData(crate::blocks::NiFloatData {
                data: group(KeyType::Linear, vec![key(0.0, 1.0, 0.0, 0.0), key(2.0, 5.0, 0.0, 0.0)]),
            }),
        ];
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        assert_eq!(
            float_extra_data_at(&blocks, geometry, "WarpAlpha", 1.0),
            Some(3.0)
        );
        // the keys beat the interpolator's own pose value wherever there are keys
        assert_eq!(
            float_extra_data_at(&blocks, geometry, "WarpAlpha", 0.0),
            Some(1.0)
        );
        // and an attribute nothing names keeps whatever the shape's extra data stores
        assert_eq!(
            float_extra_data_at(&blocks, geometry, "Exponent", 1.0),
            None
        );

        // the controller loops, so past its span the track comes round rather than holding its
        // last key for the rest of the file
        assert_eq!(
            float_extra_data_at(&blocks, geometry, "WarpAlpha", 3.0),
            float_extra_data_at(&blocks, geometry, "WarpAlpha", 1.0)
        );
    }

    /// A file can drive the same attribute on several shapes at once, so the controller's own
    /// target decides which one it moves. Matching on the name alone would drag every shape
    /// carrying that attribute along with the one that animates.
    #[test]
    fn a_driven_attribute_moves_only_the_shape_its_controller_names() {
        let blocks = vec![
            shape(BlockRef::None, BlockRef::None),
            shape(BlockRef::None, BlockRef::None),
            extra_data_controller("WarpAlpha", BlockRef::Index(1), BlockRef::Index(3)),
            float_interpolator(0.75, BlockRef::None),
        ];
        let (Some(Block::NiTriShape(first)), Some(Block::NiTriShape(second))) =
            (blocks.first(), blocks.get(1))
        else {
            unreachable!()
        };

        // with no key data the interpolator's pose value is what the target gets
        assert_eq!(
            float_extra_data_at(&blocks, second, "WarpAlpha", 1.0),
            Some(0.75)
        );
        assert_eq!(float_extra_data_at(&blocks, first, "WarpAlpha", 1.0), None);
    }

    /// Two faces meeting at one vertex, one a hundred times the area of the other and facing a
    /// different way. Each contributes its own unit normal, so the shared vertex splits the
    /// difference evenly. Accumulating the raw cross products instead weights by area and would
    /// leave it pointing very nearly along the large face.
    fn bent_fan() -> (Vec<Vector3>, Vec<Triangle>) {
        let vertices = vec![
            vector(0.0, 0.0, 0.0),
            vector(1.0, 0.0, 0.0),
            vector(0.0, 1.0, 0.0),
            vector(0.0, 0.0, 10.0),
            vector(10.0, 0.0, 0.0),
        ];
        let triangles = vec![
            Triangle { a: 0, b: 1, c: 2 },
            Triangle { a: 0, b: 3, c: 4 },
        ];
        (vertices, triangles)
    }

    #[test]
    fn a_recalculated_normal_weighs_every_face_the_same() {
        let (vertices, triangles) = bent_fan();
        let blocks = vec![
            shape(BlockRef::Index(1), BlockRef::Index(2)),
            morpher(GeomMorpherFlags::UpdateNormalsEnabled),
            shape_data(
                vertices.clone(),
                Some(vec![vector(0.0, 0.0, 1.0); 5]),
                triangles,
            ),
        ];
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        let normals = morph_normals(&blocks, geometry, &vertices).expect("the morpher asks");

        // the two faces face +z and +y, so the vertex they share points between them
        let shared = Vec3::from(&normals[0]);
        let half = std::f32::consts::FRAC_1_SQRT_2;
        assert!(
            (shared - Vec3::new(0.0, half, half)).length() < 1e-5,
            "shared vertex normal is {shared:?}"
        );
        // and a vertex only one face touches takes that face's normal outright
        assert!((Vec3::from(&normals[1]) - Vec3::Z).length() < 1e-5);
        assert!((Vec3::from(&normals[4]) - Vec3::Y).length() < 1e-5);
    }

    /// The engine recalculates only when the morpher asks and the shape has normals of its own,
    /// so both halves of that gate have to hold.
    #[test]
    fn normals_are_left_alone_unless_the_morpher_asks_for_them() {
        let (vertices, triangles) = bent_fan();
        let stored = vec![vector(0.0, 0.0, 1.0); 5];

        let quiet = vec![
            shape(BlockRef::Index(1), BlockRef::Index(2)),
            morpher(GeomMorpherFlags::UpdateNormalsDisabled),
            shape_data(vertices.clone(), Some(stored), triangles.clone()),
        ];
        let Some(Block::NiTriShape(geometry)) = quiet.first() else {
            unreachable!()
        };
        assert!(morph_normals(&quiet, geometry, &vertices).is_none());

        // and a shape storing none gets none whether or not the morpher asks
        let bare = vec![
            shape(BlockRef::Index(1), BlockRef::Index(2)),
            morpher(GeomMorpherFlags::UpdateNormalsEnabled),
            shape_data(vertices.clone(), None, triangles),
        ];
        let Some(Block::NiTriShape(geometry)) = bare.first() else {
            unreachable!()
        };
        assert!(morph_normals(&bare, geometry, &vertices).is_none());
    }

    /// The whole of one: the controller and the interpolator it reads, side by side.
    fn stacked(scale: Option<f32>, up: Option<f32>, next: BlockRef, at: u32) -> Vec<Block> {
        let invalid = -f32::MAX;
        vec![
            Block::NiTransformController(crate::blocks::NiTransformController {
                base: crate::blocks::NiSingleInterpController {
                    base: crate::blocks::NiInterpController {
                        base: time_controller(next),
                    },
                    interpolator_ref: BlockRef::Index(at + 1),
                },
            }),
            Block::NiTransformInterpolator(crate::blocks::NiTransformInterpolator {
                base: crate::blocks::NiKeyBasedInterpolator {
                    base: crate::blocks::NiInterpolator {},
                },
                transform: crate::common::NiQuatTransform {
                    // All three or none: the format marks a whole channel absent, and a vector
                    // with one live component in it reads as nothing at all.
                    translation: match up {
                        Some(up) => vector(0.0, 0.0, up),
                        None => vector(invalid, invalid, invalid),
                    },
                    rotation: crate::common::Quaternion {
                        x: invalid,
                        y: invalid,
                        z: invalid,
                        w: invalid,
                    },
                    scale: scale.unwrap_or(invalid),
                },
                data_ref: BlockRef::None,
            }),
        ]
    }

    /// A node carries every controller hung off it, and every one of them runs.
    ///
    /// `NiTransformController::Update` writes only the channels its interpolator says are valid,
    /// and `NiAVObject::UpdateObjectControllers` walks the chain from the head calling each in
    /// turn. So the last controller to write a channel decides it, and one that writes nothing
    /// about a channel leaves whatever came before.
    ///
    /// Reading the first alone is what drew a roof item at ten times its size: `i_discgem` hangs
    /// four off one node and the first of them carries a scale of ten that a later one puts back.
    /// Twenty nine of the vehicle files stack them, up to eight deep on one node.
    #[test]
    fn every_controller_on_a_node_gets_its_say() {
        let mut blocks = vec![shape(BlockRef::Index(1), BlockRef::None)];
        // the head carries the wrong scale, as the roof items do
        blocks.extend(stacked(Some(10.0), None, BlockRef::Index(3), 1));
        // and a later one puts it right without touching anything else
        blocks.extend(stacked(Some(1.0), None, BlockRef::Index(5), 3));
        // and a third lifts it, saying nothing about scale at all
        blocks.extend(stacked(None, Some(4.0), BlockRef::None, 5));
        let Some(Block::NiTriShape(geometry)) = blocks.first() else {
            unreachable!()
        };

        let held = transform_at(&blocks, geometry, 0.5).expect("the node is driven");
        assert!(
            (held.scale - 1.0).abs() < 1e-5,
            "the first controller won the scale: {}",
            held.scale
        );
        assert!(
            (held.translation.z - 4.0).abs() < 1e-5,
            "the lift was lost: {}",
            held.translation.z
        );
    }

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


#[cfg(test)]
mod rotations {
    use super::*;
    use crate::common::{QuatKey, Quaternion};

    /// The walk this used to do, kept so the halving can be held against it.
    fn by_walking(keys: &[QuatKey], time: f32) -> Option<Quaternion> {
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

    fn track(times: &[f32]) -> NiTransformData {
        let quaternion_keys = times
            .iter()
            .enumerate()
            .map(|(n, at)| {
                let turn = Quat::from_rotation_z(n as f32 * 0.6)
                    * Quat::from_rotation_x(n as f32 * -0.25);
                QuatKey {
                    time: Some(*at),
                    value: Some(turn.into()),
                    tbc: None,
                }
            })
            .collect();
        NiTransformData {
            num_rotation_keys: times.len() as u32,
            rotation_type: Some(crate::common::KeyType::Linear),
            quaternion_keys,
            xyz_rotations: None,
            translations: crate::common::KeyGroup {
                interpolation: None,
                keys: Vec::new(),
            },
            scales: crate::common::KeyGroup {
                interpolation: None,
                keys: Vec::new(),
            },
        }
    }

    /// Halving finds the same turn the walk did, at every moment either can be asked about.
    #[test]
    fn halving_lands_where_walking_did() {
        for times in [
            vec![],
            vec![0.0],
            vec![0.0, 1.0],
            vec![0.0, 0.5, 2.0, 2.0, 5.5],
            (0..40).map(|n| n as f32 * 0.37).collect::<Vec<_>>(),
        ] {
            let data = track(&times);
            // before the first key, on every key, between every pair, and past the last
            let mut moments = vec![-3.0f32, -0.0001, 0.0, 99.0];
            for (n, at) in times.iter().enumerate() {
                moments.push(*at);
                moments.push(at - 0.0001);
                moments.push(at + 0.0001);
                if let Some(next) = times.get(n + 1) {
                    moments.push((at + next) * 0.5);
                }
            }
            for time in moments {
                let walked = by_walking(&data.quaternion_keys, time);
                let halved = data.rotation_at(time);
                match (walked, halved) {
                    (None, None) => {}
                    (Some(a), Some(b)) => {
                        let (a, b) = (Quat::from(&a), Quat::from(&b));
                        // a turn and its negation are the same turn, and `angle_between` is an
                        // acos that goes to NaN when the dot creeps past one, which two equal
                        // turns manage
                        assert!(
                            (a.dot(b).abs() - 1.0).abs() < 1e-5,
                            "{} keys at {time}: {a:?} by walking, {b:?} by halving",
                            times.len()
                        );
                    }
                    _ => panic!("{} keys at {time}: one found a turn and the other did not", times.len()),
                }
            }
        }
    }
}
