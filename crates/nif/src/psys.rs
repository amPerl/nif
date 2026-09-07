//! Particle systems, simulated.
//!
//! Unlike the rest of the animation model a particle system carries state: what it looks like at
//! a time depends on how it got there. The state lives in a [`System`] the caller owns, and
//! [`System::seek`] makes it a function of time alone by resimulating from the start.
//!
//! Two things make that resimulation give the same answer twice. The step is fixed, so a result
//! never depends on frame rate, and the generator is seeded and reset with the system, so the
//! same time yields the same particles. A live game needed neither; a viewer with a timeline
//! needs both.
//!
//! The particles will not sit where the game's did. Spawning is random and this generator is a
//! different one, so the behaviour is reproduced and the individual particles are not.

use glam::{Mat4, Vec3};

use crate::blocks::{
    Block, EmitFrom, ForceType, NiPSysColorModifier, NiPSysEmitter, NiPSysGravityModifier,
    NiPSysGrowFadeModifier, NiPSysRotationModifier, VelocityType,
};
use crate::common::{BlockRef, Color4, NiTransform, Vector3};

/// The interval the simulation advances by. Forces apply before movement within a step, so how
/// far a particle travels depends on how coarse the step is and only a fixed one repeats.
pub const STEP: f32 = 1.0 / 60.0;

/// What the engine multiplies a stored gravity strength by before applying it.
const GRAVITY_SCALE: f32 = 1.6;

/// Where the engine gives up unwinding a rotation and starts the turn again.
const TEN_PI: f32 = 10.0 * std::f32::consts::PI;

/// A particle never scales quite to nothing, so one that is still alive still covers something.
/// The engine shares this floor across its modifiers rather than picking one per effect.
const SIZE_FLOOR: f32 = 0.0001;

/// How far a seek simulates before giving up, so a controller with an absurd span cannot hang
/// the caller.
const MAX_STEPS: u32 = 60 * 60;

/// A modifier carries the order it runs in, and these are the values that matter here.
mod order {
    pub const AGE_DEATH: u32 = 0;
    pub const EMIT: u32 = 1000;
    pub const POSITION: u32 = 6000;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vector3,
    pub velocity: Vector3,
    /// How long it has been alive, which drives everything that varies over life.
    pub age: f32,
    pub life_span: f32,
    pub radius: f32,
    /// What the radius is scaled by, which is what a grow and fade modifier drives. 1 where
    /// nothing does, and never quite 0, so a particle always has some extent.
    pub size: f32,
    /// How far the sprite is turned, in radians. The engine spins a particle in the plane facing
    /// the camera rather than about its axis, and only a mesh particle uses the axis at all.
    pub rotation: f32,
    /// How fast that turn advances, which is fixed for a particle's whole life.
    pub rotation_speed: f32,
    pub color: Color4,
    /// How many times this particle has been spawned from another. A spawn modifier refuses to
    /// go past its own generation count, which is the only thing stopping a chain reaction.
    pub generation: u16,
    /// When it was last advanced. Everything in a step measures against this, and only the
    /// move at the end of the step carries it forward.
    last_update: f32,
}

impl Particle {
    /// The radius it actually draws at, its own scaled by whatever drives its size.
    pub fn drawn_radius(&self) -> f32 {
        self.radius * self.size
    }

    /// Where it is through its life, 0 at birth and 1 at death.
    pub fn through_life(&self) -> f32 {
        if self.life_span > 0.0 {
            (self.age / self.life_span).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// The state of one `NiParticleSystem`, advanced by its owner.
pub struct System {
    /// Which block this simulates, so a caller holding several can tell them apart.
    pub block: usize,
    particles: Vec<Particle>,
    capacity: usize,
    time: f32,
    rng: Rng,
    seed: u64,
    /// Where each object an emitter names sits in this system's space, by that object's block.
    /// The simulation does not walk the graph, so the caller resolves this and hands it over.
    /// Empty means every emitter places into the system's own space, which is only right where
    /// the two coincide.
    spaces: std::collections::HashMap<usize, Mat4>,
}

impl System {
    /// Prepares an empty system for the `NiParticleSystem` at `block`. Nothing exists until it
    /// is advanced.
    pub fn new(blocks: &[Block], block: usize) -> Option<System> {
        let Some(Block::NiParticleSystem(psys)) = blocks.get(block) else {
            return None;
        };
        let capacity = match psys.data_ref.get(blocks) {
            Some(Block::NiPSysData(data)) => data.vertex_count(),
            _ => 0,
        };
        // mixed with the block index, so two systems in one file do not emit in lockstep
        let seed = 0x9e3779b97f4a7c15 ^ block as u64;
        Some(System {
            block,
            particles: Vec::with_capacity(capacity.min(4096)),
            capacity,
            time: 0.0,
            rng: Rng::new(seed),
            seed,
            spaces: std::collections::HashMap::new(),
        })
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    /// Where the objects this system's emitters name sit relative to the system itself, keyed by
    /// the object's own block. An emitter places into its object's space, not the system's: a
    /// volume emitter names one through `emitter_object_ref` and a mesh emitter names the mesh it
    /// emits from, and in this game every one of them names something.
    ///
    /// Survives `reset`, since it describes the file rather than the simulation.
    pub fn place_against(&mut self, spaces: std::collections::HashMap<usize, Mat4>) {
        self.spaces = spaces;
    }

    /// Every block whose space this system's emitters place into, for the caller to resolve.
    pub fn emitter_objects(blocks: &[Block], block: usize) -> Vec<usize> {
        let Some(Block::NiParticleSystem(psys)) = blocks.get(block) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for modifier in psys.modifiers_refs.iter().filter_map(|r| r.get(blocks)) {
            match modifier {
                Block::NiPSysBoxEmitter(e) => out.extend(e.base.emitter_object_ref.index()),
                Block::NiPSysCylinderEmitter(e) => out.extend(e.base.emitter_object_ref.index()),
                Block::NiPSysSphereEmitter(e) => out.extend(e.base.emitter_object_ref.index()),
                Block::NiPSysMeshEmitter(e) => {
                    out.extend(e.emitter_mesh_refs.iter().filter_map(|r| r.index()))
                }
                _ => {}
            }
        }
        out
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    /// How many particles it may hold at once, as its data declares.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// The spaces its emitters place into, for handing to another system built from the same
    /// block. Resolving them means walking the file, which is worth doing once.
    pub fn spaces(&self) -> &std::collections::HashMap<usize, Mat4> {
        &self.spaces
    }

    /// Mixes `salt` into the seed and starts again.
    ///
    /// One system standing in many places is many simulations, and they are only different if
    /// their generators are. Without this every copy emits the same particle at the same moment,
    /// which reads as wrong wherever two copies can be seen at once.
    pub fn stir(&mut self, salt: u64) {
        self.seed ^= salt.wrapping_mul(0x9e3779b97f4a7c15);
        self.reset();
    }

    /// Returns the system to its state before anything was emitted.
    pub fn reset(&mut self) {
        self.particles.clear();
        self.time = 0.0;
        self.rng = Rng::new(self.seed);
    }

    /// The state at `time`. Advancing runs on from where it is; going back restarts, since a
    /// step cannot be undone.
    pub fn seek(&mut self, blocks: &[Block], time: f32) {
        if time < self.time {
            self.reset();
        }
        let mut steps = 0;
        while self.time + STEP <= time && steps < MAX_STEPS {
            self.step(blocks, STEP);
            steps += 1;
        }
    }

    /// One fixed step, running each modifier in the order the file gives it.
    fn step(&mut self, blocks: &[Block], dt: f32) {
        let last = self.time;
        let now = last + dt;
        let Some(Block::NiParticleSystem(psys)) = blocks.get(self.block) else {
            return;
        };

        let mut modifiers: Vec<(u32, usize)> = psys
            .modifiers_refs
            .iter()
            .filter_map(|r| r.index())
            .filter_map(|index| {
                let modifier = blocks.get(index)?.as_psys_modifier()?;
                modifier.active.then_some((modifier.order, index))
            })
            .collect();
        modifiers.sort_by_key(|(order, _)| *order);

        // several modifiers share one order, so the block decides the behaviour and the order
        // only decides the sequence
        for (order, index) in modifiers {
            match blocks.get(index) {
                Some(Block::NiPSysGrowFadeModifier(m)) => self.grow_and_fade(m),
                Some(Block::NiPSysRotationModifier(_)) => self.spin(now),
                Some(Block::NiPSysGravityModifier(m)) => self.pull(blocks, m, now),
                Some(Block::NiPSysColorModifier(m)) => self.tint(blocks, m),
                _ => match order {
                    order::AGE_DEATH => self.age_and_die(blocks, index, now),
                    order::EMIT => self.emit_from(blocks, index, last, now),
                    order::POSITION => self.integrate(now),
                    _ => {}
                },
            }
        }
        self.time = now;
    }

    /// Scale each particle up over its first moments and down over its last. The two are
    /// separate spans and the smaller of them wins where they overlap, so a particle whose grow
    /// and fade together outlast it never reaches full size. A zero time turns its half off.
    ///
    /// Generation is what spawning on death increments, and nothing spawns here, so every
    /// particle is of the first generation and a modifier aimed at a later one does nothing.
    fn grow_and_fade(&mut self, modifier: &NiPSysGrowFadeModifier) {
        for particle in &mut self.particles {
            let grow = if modifier.grow_generation == 0 && modifier.grow_time > 0.0 {
                (particle.age / modifier.grow_time).min(1.0)
            } else {
                1.0
            };
            let left = particle.life_span - particle.age;
            let fade = if modifier.fade_generation == 0 && modifier.fade_time > 0.0 {
                (left / modifier.fade_time).min(1.0)
            } else {
                1.0
            };
            particle.size = grow.min(fade).max(SIZE_FLOOR);
        }
    }

    /// Add a force to each particle's velocity. Position integrates afterwards, which the
    /// modifier order already arranges, so a force accumulates into velocity and then moves.
    ///
    /// The direction is the modifier's axis carried into the system's own space by the gravity
    /// object's transform, and without a gravity object the engine applies nothing at all. Every
    /// file stores the same axis, so it is the transform that decides where a system's gravity
    /// points, not the axis.
    fn pull(&mut self, blocks: &[Block], modifier: &NiPSysGravityModifier, now: f32) {
        let Some(object) = modifier.gravity_object_ref.index() else {
            return;
        };
        let Some(relative) = relative_transform(blocks, self.block, object, now) else {
            return;
        };
        let towards = relative.w_axis.truncate();
        let axis = relative
            .transform_vector3(Vec3::from(&modifier.gravity_axis))
            .normalize_or_zero();
        // the engine scales the stored strength before applying it
        let strength = modifier.strength * GRAVITY_SCALE;

        for particle in &mut self.particles {
            let delta = now - particle.last_update;
            let at = Vec3::from(&particle.position);
            // a planar force pushes the same way everywhere; a spherical one points at the
            // object, so its direction is the particle's own
            let (direction, distance) = match modifier.force_type {
                ForceType::Spherical => {
                    let to_object = towards - at;
                    (to_object.normalize_or_zero(), to_object.length())
                }
                _ => (axis, axis.dot(towards - at)),
            };
            // decay falls off with how far the particle is along that direction, and the engine
            // measures a planar distance signed and folds the sign away
            let decay = if modifier.decay == 0.0 {
                1.0
            } else {
                (-modifier.decay * distance.abs()).exp()
            };
            let push = direction * (strength * decay * delta);
            particle.velocity = (Vec3::from(&particle.velocity) + push).into();
        }
    }

    /// Advance each particle's turn by the delta it last saw. The speed is its own, set when it
    /// was born, so this only integrates. The engine keeps the angle inside a turn and drops it
    /// to zero rather than looping forever if it ever runs away.
    fn spin(&mut self, now: f32) {
        for particle in &mut self.particles {
            let delta = now - particle.last_update;
            particle.rotation += delta * particle.rotation_speed;
            if particle.rotation > TEN_PI {
                particle.rotation = 0.0;
            } else {
                while particle.rotation > std::f32::consts::TAU {
                    particle.rotation -= std::f32::consts::TAU;
                }
            }
        }
    }

    /// Take each particle's colour from a track read at how far through its life it is. The
    /// track replaces the colour rather than tinting it, and outside its own key range the
    /// nearest key holds, which is what clamping to the range does.
    fn tint(&mut self, blocks: &[Block], modifier: &NiPSysColorModifier) {
        let Some(Block::NiColorData(data)) = modifier.data_ref.get(blocks) else {
            return;
        };
        let (Some(first), Some(last)) = (data.data.keys.first(), data.data.keys.last()) else {
            return;
        };
        for particle in &mut self.particles {
            let at = particle.through_life().clamp(first.time, last.time);
            if let Some(color) = data.data.sample(at) {
                particle.color = color;
            }
        }
    }

    /// Age each particle by the delta it last saw, then drop the ones past their span. A
    /// modifier that spawns on death gets to see them first, since what it spawns is born from
    /// where the parent got to rather than from the emitter.
    fn age_and_die(&mut self, blocks: &[Block], index: usize, now: f32) {
        for particle in &mut self.particles {
            particle.age += now - particle.last_update;
        }
        let spawner = match blocks.get(index) {
            Some(Block::NiPSysAgeDeathModifier(m)) if m.spawn_on_death => m
                .spawn_modifier_ref
                .index()
                .filter(|i| matches!(blocks.get(*i), Some(Block::NiPSysSpawnModifier(_)))),
            _ => None,
        };
        if let Some(spawner) = spawner {
            let dying: Vec<Particle> = self
                .particles
                .iter()
                .filter(|p| p.age > p.life_span)
                .copied()
                .collect();
            for parent in dying {
                self.spawn_from(blocks, spawner, &parent, now);
            }
        }
        self.particles
            .retain(|particle| particle.age <= particle.life_span);
    }

    /// What one dying particle leaves behind. The children start where it ended, keep its colour
    /// and size, and take its speed and heading with the modifier's own chaos applied to both.
    fn spawn_from(&mut self, blocks: &[Block], index: usize, parent: &Particle, now: f32) {
        let Some(Block::NiPSysSpawnModifier(modifier)) = blocks.get(index) else {
            return;
        };
        if parent.generation >= modifier.num_spawn_generations
            || self.rng.unit() > modifier.percentage_spawned
        {
            return;
        }
        let spread = modifier
            .max_num_to_spawn
            .saturating_sub(modifier.min_num_to_spawn);
        let count = modifier.min_num_to_spawn + (self.rng.unit() * spread as f32).round() as u16;
        let count = count.max(1);

        let heading = Vec3::from(&parent.velocity);
        let speed = heading.length();
        // +z turned onto the parent's heading. The engine builds that rotation by hand; the
        // roll it leaves unspecified does not matter, since the planar angle below is uniform
        // over the full turn and absorbs it.
        let onto = glam::Quat::from_rotation_arc(Vec3::Z, heading.normalize_or(Vec3::Z));
        for _ in 0..count {
            if self.particles.len() >= self.capacity {
                return;
            }
            let declination = self.rng.unit() * modifier.spawn_dir_variation * std::f32::consts::PI;
            let planar = self.rng.unit() * std::f32::consts::TAU;
            let chaos = Vec3::new(
                declination.sin() * planar.cos(),
                declination.sin() * planar.sin(),
                declination.cos(),
            );
            let faster = 1.0 + modifier.spawn_speed_variation * self.rng.unit();
            let life_span =
                modifier.life_span + modifier.life_span_variation * (self.rng.unit() - 0.5);
            self.particles.push(Particle {
                position: parent.position,
                velocity: (onto * chaos * speed * faster).into(),
                // born as its parent died, which is within this step
                age: 0.0,
                life_span,
                radius: parent.radius,
                size: parent.size,
                rotation: parent.rotation,
                rotation_speed: parent.rotation_speed,
                color: parent.color,
                generation: parent.generation + 1,
                last_update: now,
            });
        }
    }

    /// Move by the velocity over the delta, and take the delta up so nothing counts it twice.
    fn integrate(&mut self, now: f32) {
        for particle in &mut self.particles {
            let delta = now - particle.last_update;
            let step = Vec3::from(&particle.velocity) * delta;
            particle.position = (Vec3::from(&particle.position) + step).into();
            particle.last_update = now;
        }
    }

    /// Emit whatever is due. The count born by a time is a closed form rather than a running
    /// total, so the population never depends on the step size, and each particle is back dated
    /// to the moment within the interval it was due.
    fn emit_from(&mut self, blocks: &[Block], index: usize, last: f32, now: f32) {
        let Some(emitter) = blocks.get(index).and_then(as_emitter) else {
            return;
        };
        let Some(rate) = birth_rate(blocks, self.block, index) else {
            return;
        };
        let (start, stop) = rate.window;
        if rate.per_second <= 0.0 || start >= stop || now <= start {
            return;
        }
        // past the end of its span an emitter is done, unless the span comes round again
        if !rate.repeats && last >= stop {
            return;
        }

        // emission counts against how long the emitter has been on, not against the clock
        let current_delta = rate.emitting_before(now);
        let last_delta = rate.emitting_before(last);
        let current_count = (rate.per_second * current_delta) as u32;
        let last_count = (rate.per_second * last_delta) as u32;
        let interval = 1.0 / rate.per_second;
        // a particle's turn is set when it is born rather than each step, so the modifier that
        // decides it is found once
        let spinner =
            system_modifiers(blocks, self.block).find_map(|index| match blocks.get(index) {
                Some(Block::NiPSysRotationModifier(m)) => Some(m),
                _ => None,
            });

        for born in last_count..current_count {
            if self.particles.len() >= self.capacity {
                return;
            }
            // its age is real elapsed time, so the moment it was due is converted back out of
            // emitting time before measuring from it
            let due = rate.moment_at((born + 1) as f32 * interval);
            let age = (now - due).max(0.0);
            let Some(mut particle) = emit(emitter, age, &mut self.rng) else {
                continue;
            };
            place(blocks, index, &self.spaces, &mut particle, &mut self.rng);
            if let Some(spin) = spinner {
                seed_rotation(spin, &mut particle, &mut self.rng);
            }
            // back dated, so the particle catches up to now within its first step
            particle.last_update = now - particle.age;
            self.particles.push(particle);
        }
    }
}

/// A birth rate and the window it applies over, from the emitter's own controller.
struct BirthRate {
    per_second: f32,
    window: (f32, f32),
    /// When the emitter is switched on, in order and within the window.
    on: Vec<(f32, f32)>,
    /// Whether the controller repeats its span, which brings the whole on and off pattern round
    /// again. A burst emitter that clamps fires once and is done.
    repeats: bool,
}

impl BirthRate {
    /// How long one pass of the span leaves the emitter switched on.
    fn per_cycle(&self) -> f32 {
        self.on.iter().map(|(from, to)| to - from).sum()
    }

    fn duration(&self) -> f32 {
        (self.window.1 - self.window.0).max(0.0)
    }

    /// Whether the pattern comes round again, which needs a span to come round in.
    fn cycles(&self) -> bool {
        self.repeats && self.duration() > 0.0 && self.per_cycle() > 0.0
    }

    /// How long the emitter has been switched on by `time`, which is the clock emission counts
    /// against. A track that is on throughout makes this the plain elapsed time.
    ///
    /// A looping emitter counts every pass, not just the first. Stopping at the end of the span
    /// leaves it firing once and then dark for the rest of the file, however long that is.
    fn emitting_before(&self, time: f32) -> f32 {
        let within = |at: f32| -> f32 {
            self.on
                .iter()
                .map(|(from, to)| (at.min(*to) - from).max(0.0))
                .sum()
        };
        if !self.cycles() {
            return within(time);
        }
        let duration = self.duration();
        let elapsed = (time - self.window.0).max(0.0);
        let passes = (elapsed / duration).floor();
        passes * self.per_cycle() + within(self.window.0 + elapsed - passes * duration)
    }

    /// The moment an emitter that had been on for `elapsed` reached it, which is the inverse of
    /// `emitting_before`. Past the end of the last interval it holds there, since nothing is born
    /// after that anyway, unless the pattern repeats and it comes round again.
    fn moment_at(&self, elapsed: f32) -> f32 {
        let (passes, mut left) = match self.cycles() {
            true => {
                let passes = (elapsed / self.per_cycle()).floor();
                (passes, elapsed - passes * self.per_cycle())
            }
            false => (0.0, elapsed),
        };
        let carried = passes * self.duration();
        for (from, to) in &self.on {
            let length = to - from;
            if left <= length {
                return from + left + carried;
            }
            left -= length;
        }
        self.on.last().map_or(self.window.0, |(_, to)| *to) + carried
    }
}

/// The birth rate driving this emitter. A controller names the modifier it drives rather than
/// pointing at it, so the two are matched by name.
fn birth_rate(blocks: &[Block], system: usize, modifier: usize) -> Option<BirthRate> {
    let name = &blocks.get(modifier)?.as_psys_modifier()?.name;
    for block in blocks {
        let Block::NiPSysEmitterCtlr(controller) = block else {
            continue;
        };
        if controller.modifier_name != *name {
            continue;
        }
        let time = &controller.base.base.base;
        // a modifier name is unique only within its own system
        if time
            .target_ref
            .index()
            .is_some_and(|target| target != system)
        {
            continue;
        }
        let keyed = match controller.interpolator_ref.get(blocks) {
            Some(Block::NiFloatInterpolator(interpolator)) => {
                match interpolator.data_ref.get(blocks) {
                    Some(Block::NiFloatData(data)) => data.data.sample(time.start_time),
                    _ => Some(interpolator.value),
                }
            }
            _ => None,
        };
        let window = (time.start_time, time.end_time);
        return Some(BirthRate {
            per_second: keyed.unwrap_or(0.0),
            window,
            on: switched_on(blocks, controller.visibility_interpolator_ref, window),
            repeats: matches!(time.cycle_type_enum(), crate::anim::CycleType::Loop),
        });
    }
    None
}

/// When an emitter is switched on, as intervals inside its controller's own span. The engine
/// walks this track's key pairs and emits once per on interval rather than across the whole span,
/// so an emitter that burst fires stops when the track says so instead of running forever.
///
/// A track with no keys leaves the emitter on for the whole span, which is what a posed
/// interpolator amounts to.
fn switched_on(blocks: &[Block], reference: BlockRef, window: (f32, f32)) -> Vec<(f32, f32)> {
    let whole = vec![window];
    let Some(Block::NiBoolInterpolator(interpolator)) = reference.get(blocks) else {
        return whole;
    };
    let Some(Block::NiBoolData(data)) = interpolator.data_ref.get(blocks) else {
        // posed rather than keyed, so it holds one value for the whole span
        return if interpolator.value == 0 {
            Vec::new()
        } else {
            whole
        };
    };
    if data.data.keys.is_empty() {
        return whole;
    }

    let mut intervals = Vec::new();
    let mut opened: Option<f32> = None;
    for key in &data.data.keys {
        match (key.value != 0, opened) {
            (true, None) => opened = Some(key.time),
            (false, Some(from)) => {
                intervals.push((from, key.time));
                opened = None;
            }
            _ => {}
        }
    }
    // a track that never switches off runs to the end of the span
    if let Some(from) = opened {
        intervals.push((from, window.1));
    }

    intervals
        .into_iter()
        .filter_map(|(from, to)| {
            let clipped = (from.max(window.0), to.min(window.1));
            (clipped.1 > clipped.0).then_some(clipped)
        })
        .collect()
}

fn as_emitter(block: &Block) -> Option<&NiPSysEmitter> {
    let emitter: &NiPSysEmitter = match block {
        Block::NiPSysBoxEmitter(e) => e,
        Block::NiPSysCylinderEmitter(e) => e,
        Block::NiPSysSphereEmitter(e) => e,
        Block::NiPSysMeshEmitter(e) => e,
        _ => return None,
    };
    Some(emitter)
}

/// What an emitter gives a particle at birth. Each field takes its variation differently, and
/// the split is deliberate: speed and life span take half of it, the angles and the radius all.
fn emit(emitter: &NiPSysEmitter, age: f32, rng: &mut Rng) -> Option<Particle> {
    let life_span = emitter.life_span + emitter.life_span_variation * (rng.unit() - 0.5);
    if age > life_span {
        return None;
    }
    let speed = emitter.speed + emitter.speed_variation * (rng.unit() - 0.5);
    let declination = emitter.declination + emitter.declination_variation * rng.symmetric();
    let planar = emitter.planar_angle + emitter.planar_angle_variation * rng.symmetric();

    let sin_declination = declination.sin();
    let direction = Vec3::new(
        sin_declination * planar.cos(),
        sin_declination * planar.sin(),
        declination.cos(),
    );

    Some(Particle {
        position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        velocity: (direction * speed).into(),
        age,
        life_span,
        radius: emitter.initial_radius + emitter.radius_variation * rng.symmetric(),
        size: 1.0,
        rotation: 0.0,
        rotation_speed: 0.0,
        color: emitter.initial_color,
        generation: 0,
        last_update: 0.0,
    })
}

/// One object's transform as of `time`, composed from the root down. A NIF links parents to
/// children and not the other way, so the chain up is found by asking which node claims each
/// block as a child.
fn world_transform(blocks: &[Block], target: usize, time: f32) -> Option<NiTransform> {
    let mut chain = vec![target];
    let mut at = target;
    // a graph is a tree here, and the guard is against a file that says otherwise
    for _ in 0..blocks.len() {
        let Some(parent) = parent_of(blocks, at) else {
            break;
        };
        chain.push(parent);
        at = parent;
    }

    let mut world = NiTransform::IDENTITY;
    for index in chain.into_iter().rev() {
        let object = blocks.get(index)?.av_object()?;
        // an animated node is wherever its controller leaves it, not where the file stores it
        let own = crate::anim::transform_at(blocks, object, time).unwrap_or(NiTransform {
            rotation: object.rotation,
            translation: object.translation,
            scale: object.scale,
        });
        world = world.compose(&own);
    }
    Some(world)
}

/// Which block names `child` among its children.
fn parent_of(blocks: &[Block], child: usize) -> Option<usize> {
    blocks.iter().position(|block| match block {
        Block::NiNode(node) => node.child_refs.iter().any(|r| r.index() == Some(child)),
        Block::NiBillboardNode(node) => node.child_refs.iter().any(|r| r.index() == Some(child)),
        Block::NiSortAdjustNode(node) => node.child_refs.iter().any(|r| r.index() == Some(child)),
        Block::NiSwitchNode(node) => node.child_refs.iter().any(|r| r.index() == Some(child)),
        Block::NiLODNode(node) => node.child_refs.iter().any(|r| r.index() == Some(child)),
        _ => false,
    })
}

/// Where one object sits in another's space, which is what carries a gravity axis and a gravity
/// object's position into the space a system's particles live in.
fn relative_transform(blocks: &[Block], system: usize, object: usize, time: f32) -> Option<Mat4> {
    let system = Mat4::from(&world_transform(blocks, system, time)?);
    let object = Mat4::from(&world_transform(blocks, object, time)?);
    Some(system.inverse() * object)
}

/// How far and how fast a newly born particle turns. The variation is applied whole here rather
/// than halved, unlike the emitter's speed and life span.
fn seed_rotation(modifier: &NiPSysRotationModifier, particle: &mut Particle, rng: &mut Rng) {
    particle.rotation = modifier.initial_rotation_angle
        + modifier.initial_rotation_angle_variation * rng.symmetric();
    let mut speed = modifier.initial_rotation_speed
        + modifier.initial_rotation_speed_variation * rng.symmetric();
    if modifier.random_rot_speed_sign && rng.unit() <= 0.5 {
        speed = -speed;
    }
    particle.rotation_speed = speed;
}

/// The modifiers one system carries, by block index.
fn system_modifiers(blocks: &[Block], system: usize) -> impl Iterator<Item = usize> + '_ {
    let refs = match blocks.get(system) {
        Some(Block::NiParticleSystem(psys)) => psys.modifiers_refs.as_slice(),
        _ => &[],
    };
    refs.iter().filter_map(|r| r.index())
}

/// Where in its volume, or on its surface, an emitter starts a particle, taken into the system's
/// space through whatever the caller resolved for the object the emitter names.
///
/// A mesh emitter also decides the direction: every one in this game takes it from the surface
/// the particle leaves rather than from the emitter's own declination.
fn place(
    blocks: &[Block],
    index: usize,
    spaces: &std::collections::HashMap<usize, Mat4>,
    particle: &mut Particle,
    rng: &mut Rng,
) {
    let (local, normal, object) = match blocks.get(index) {
        Some(Block::NiPSysBoxEmitter(e)) => (
            Vec3::new(
                e.width * (rng.unit() - 0.5),
                e.height * (rng.unit() - 0.5),
                e.depth * (rng.unit() - 0.5),
            ),
            None,
            e.base.emitter_object_ref.index(),
        ),
        Some(Block::NiPSysCylinderEmitter(e)) => {
            let angle = rng.unit() * std::f32::consts::TAU;
            // area weighted, or the middle of the disc gets far more than its share
            let radius = e.radius * rng.unit().max(0.0).sqrt();
            (
                Vec3::new(
                    radius * angle.cos(),
                    radius * angle.sin(),
                    e.height * (rng.unit() - 0.5),
                ),
                None,
                e.base.emitter_object_ref.index(),
            )
        }
        Some(Block::NiPSysSphereEmitter(e)) => {
            // volume weighted the same way, and the direction is uniform over the sphere
            let z = rng.symmetric();
            let angle = rng.unit() * std::f32::consts::TAU;
            let ring = (1.0 - z * z).max(0.0).sqrt();
            let radius = e.radius * rng.unit().max(0.0).cbrt();
            (
                Vec3::new(
                    radius * ring * angle.cos(),
                    radius * ring * angle.sin(),
                    radius * z,
                ),
                None,
                e.base.emitter_object_ref.index(),
            )
        }
        Some(Block::NiPSysMeshEmitter(e)) => {
            // one mesh each in this corpus, but the engine picks at random among them
            let count = e.emitter_mesh_refs.len();
            if count == 0 {
                return;
            }
            let which = ((rng.unit() * count as f32) as usize).min(count - 1);
            let mesh = e.emitter_mesh_refs[which].index();
            let Some((point, normal)) =
                mesh.and_then(|m| surface_point(blocks, m, &e.emission_type, rng))
            else {
                return;
            };
            // only `UseNormals` takes its direction from the surface, and it is the only kind
            // this game asks for
            let aimed = matches!(e.initial_velocity_type, VelocityType::UseNormals);
            (point, aimed.then_some(normal).flatten(), mesh)
        }
        _ => return,
    };
    let space = object
        .and_then(|object| spaces.get(&object))
        .copied()
        .unwrap_or(Mat4::IDENTITY);
    particle.position = space.transform_point3(local).into();
    if let Some(normal) = normal {
        let speed = Vec3::from(&particle.velocity).length();
        let aimed = space.transform_vector3(normal).normalize_or_zero();
        particle.velocity = (aimed * speed).into();
    }
}

/// A point on a mesh's surface and the direction to leave it by, in the mesh's own space.
///
/// The direction is the **stored vertex normals** of the corners involved, averaged and unitized,
/// not the triangle's geometric normal, and a mesh storing none leaves the direction alone. The
/// face and edge modes pick a random triangle and then a point on it or along one of its sides;
/// the centre modes are the same triangle without the random offset.
fn surface_point(
    blocks: &[Block],
    mesh: usize,
    from: &EmitFrom,
    rng: &mut Rng,
) -> Option<(Vec3, Option<Vec3>)> {
    let block = blocks.get(mesh)?;
    let (data, triangles) = block.triangles(blocks)?;
    let vertices = data.vertices.as_ref()?;
    if triangles.is_empty() {
        return None;
    }
    let picked = ((rng.unit() * triangles.len() as f32) as usize).min(triangles.len() - 1);
    let triangle = &triangles[picked];
    let at = |index: u16| vertices.get(index as usize).map(Vec3::from);
    let normal_at = |index: u16| {
        data.normals
            .as_ref()
            .and_then(|n| n.get(index as usize))
            .map(Vec3::from)
    };
    let corners = [triangle.a, triangle.b, triangle.c];
    let (a, b, c) = (at(corners[0])?, at(corners[1])?, at(corners[2])?);

    // an edge mode only ever involves two of the three corners, so the average is over those
    let edge = matches!(from, EmitFrom::EdgeCenter | EmitFrom::EdgeSurface);
    let used: &[u16] = match edge {
        true => &corners[..2],
        false => &corners,
    };
    let mut sum = Vec3::ZERO;
    let mut have = true;
    for index in used {
        match normal_at(*index) {
            Some(normal) => sum += normal,
            None => have = false,
        }
    }
    let normal = have.then(|| (sum / used.len() as f32).normalize_or_zero());

    let point = match from {
        EmitFrom::FaceSurface => {
            let (d1, d2) = (b - a, c - a);
            let root = rng.unit().max(0.0).sqrt();
            a + (d2 * rng.unit() - d1) * root + d1
        }
        EmitFrom::EdgeSurface => a + (b - a) * rng.unit(),
        EmitFrom::EdgeCenter => (a + b) / 2.0,
        _ => (a + b + c) / 3.0,
    };
    Some((point, normal))
}

/// A small generator chosen for being reproducible rather than for its statistics: the same
/// seed has to give the same stream on every platform, or a seek stops being repeatable.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(seed | 1)
    }

    /// In 0..1.
    fn unit(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        let bits = x.wrapping_mul(0x2545f4914f6cdd1d) >> 40;
        bits as f32 / (1u32 << 24) as f32
    }

    /// In -1..1, for the fields that vary either side of their value.
    fn symmetric(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }
}

/// Every particle system in a file, ready to simulate.
pub fn systems(blocks: &[Block]) -> Vec<System> {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| matches!(block, Block::NiParticleSystem(_)))
        .filter_map(|(index, _)| System::new(blocks, index))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{NiAvObject, NiNode, NiObjectNET};
    use crate::blocks::{
        NiBoolData, NiBoolInterpolator, NiColorData, NiInterpolator, NiKeyBasedInterpolator,
        NiPSysModifier, NiString,
    };
    use crate::common::{BlockRef, Key, KeyGroup, KeyType, Matrix33};

    fn parse(path: &str) -> crate::Nif {
        let bytes = std::fs::read(path).expect("fixture");
        crate::Nif::parse(&mut std::io::Cursor::new(&bytes)).expect("parse")
    }

    #[test]
    fn a_seek_gives_the_same_answer_twice() {
        let nif = parse("tests/20.nif");
        let mut systems = systems(&nif.blocks);
        assert!(!systems.is_empty(), "the fixture has no particle system");

        let mut first = Vec::new();
        for system in &mut systems {
            system.seek(&nif.blocks, 1.5);
            first.push(system.particles().to_vec());
        }

        // seeking backwards restarts, so this exercises the reset path rather than a no-op
        let mut again = Vec::new();
        for system in &mut systems {
            system.seek(&nif.blocks, 0.25);
            system.seek(&nif.blocks, 1.5);
            again.push(system.particles().to_vec());
        }

        assert_eq!(first, again, "the same time gave different particles");
    }

    #[test]
    fn the_step_size_does_not_change_how_many_are_born() {
        let nif = parse("tests/20.nif");
        // the count born by a time is a closed form, so reaching a time in one seek or several
        // has to give the same population
        for system in &mut systems(&nif.blocks) {
            system.seek(&nif.blocks, 1.0);
            let direct = system.particles().len();

            system.reset();
            for step in 1..=10 {
                system.seek(&nif.blocks, step as f32 * 0.1);
            }
            assert_eq!(direct, system.particles().len(), "population diverged");
        }
    }

    /// The other tests hold trivially for a system that emits nothing at all, so this is what
    /// says the simulation is doing any work.
    #[test]
    fn a_system_emits_and_moves() {
        let nif = parse("tests/20.nif");
        let mut alive = 0;
        let mut moved = 0;
        for system in &mut systems(&nif.blocks) {
            system.seek(&nif.blocks, 1.0);
            alive += system.particles().len();
            moved += system
                .particles()
                .iter()
                .filter(|p| {
                    let d = Vec3::from(&p.position);
                    d.length_squared() > 0.0 && p.age > 0.0
                })
                .count();
        }
        assert!(alive > 0, "no system emitted anything");
        assert!(moved > 0, "nothing moved away from where it was born");
    }

    /// Files store the same gravity axis throughout, so the transform between a system and its
    /// gravity object is what decides which way the pull points. Reading the axis alone would
    /// have a fountain accelerating upwards.
    #[test]
    fn gravity_takes_its_direction_from_the_object_rather_than_the_axis() {
        // a root, a system under it, and a gravity object turned half way over about x
        let node = |children: Vec<usize>, rotation: Matrix33| {
            Block::NiNode(NiNode {
                base: NiAvObject {
                    base: NiObjectNET {
                        name: NiString::from("node"),
                        extra_data_refs: Vec::new(),
                        controller_ref: BlockRef::None,
                    },
                    flags: 0,
                    translation: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    rotation,
                    scale: 1.0,
                    property_refs: Vec::new(),
                    collision_ref: BlockRef::None,
                },
                child_refs: children
                    .into_iter()
                    .map(|i| BlockRef::Index(i as u32))
                    .collect(),
                effect_refs: Vec::new(),
            })
        };
        // half a turn about x, which sends z to -z
        let flipped = Matrix33 {
            row_major: [1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0],
        };
        let blocks = vec![
            node(vec![1, 2], Matrix33::IDENTITY),
            node(Vec::new(), Matrix33::IDENTITY),
            node(Vec::new(), flipped),
        ];

        assert_eq!(parent_of(&blocks, 1), Some(0));
        assert_eq!(parent_of(&blocks, 2), Some(0));
        assert_eq!(parent_of(&blocks, 0), None);

        let up = Vec3::new(0.0, 0.0, 1.0);
        // the system's own space, so an object sharing its orientation leaves the axis alone
        let same = relative_transform(&blocks, 1, 1, 0.0).expect("resolves");
        assert!(same.transform_vector3(up).abs_diff_eq(up, 1e-5));

        // and the turned object sends the same axis the other way
        let turned = relative_transform(&blocks, 1, 2, 0.0).expect("resolves");
        assert!(turned.transform_vector3(up).abs_diff_eq(-up, 1e-5));
    }

    /// A rotation is a scalar turn in the plane facing the camera, and only its speed varies per
    /// particle. The engine keeps the angle inside one turn rather than letting it grow without
    /// bound, since a long lived particle would otherwise lose precision in it.
    #[test]
    fn a_turn_advances_by_its_own_speed_and_stays_inside_one_revolution() {
        let make = |rotation: f32, speed: f32| Particle {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            age: 0.0,
            life_span: 100.0,
            radius: 1.0,
            size: 1.0,
            rotation,
            rotation_speed: speed,
            color: Color4::default(),
            generation: 0,
            last_update: 0.0,
        };
        let mut system = System {
            block: 0,
            // one turning forwards, one backwards, one already near a full turn
            particles: vec![
                make(0.0, 1.0),
                make(0.0, -1.0),
                make(std::f32::consts::TAU - 0.1, 1.0),
            ],
            capacity: 8,
            time: 0.0,
            rng: Rng::new(1),
            seed: 1,
            spaces: Default::default(),
        };
        system.spin(0.5);

        assert!((system.particles[0].rotation - 0.5).abs() < 1e-6);
        // a negative speed is left alone rather than wrapped, which is what the engine does
        assert!((system.particles[1].rotation + 0.5).abs() < 1e-6);
        // and one that passes a full turn comes back inside it
        assert!(system.particles[2].rotation < std::f32::consts::TAU);
        assert!((system.particles[2].rotation - 0.4).abs() < 1e-5);
    }

    /// An emitter is switched on and off by a track of its own, and emission counts against how
    /// long it has actually been on rather than against the clock. Without that a burst emitter
    /// runs for its whole span, emitting long after it should stop.
    #[test]
    fn emission_counts_only_the_time_an_emitter_is_switched_on() {
        let burst = BirthRate {
            per_second: 300.0,
            window: (0.0, 3.3333333),
            on: vec![(0.0, 0.16666667)],
            repeats: false,
        };
        assert!((burst.emitting_before(0.1) - 0.1).abs() < 1e-6);
        // past the end of the burst the total holds, so nothing more is ever born
        assert!((burst.emitting_before(1.0) - 0.16666667).abs() < 1e-6);
        assert_eq!(burst.emitting_before(1.0), burst.emitting_before(60.0));
        // which caps the population at what the rate buys inside the burst
        assert_eq!((burst.per_second * burst.emitting_before(60.0)) as u32, 50);

        // an emitter on throughout counts the plain elapsed time, as it did before any of this
        let steady = BirthRate {
            per_second: 30.0,
            window: (0.0, 3.3333333),
            on: vec![(0.0, 3.3333333)],
            repeats: false,
        };
        assert!((steady.emitting_before(1.0) - 1.0).abs() < 1e-6);
    }

    /// A particle's age is real elapsed time, so the moment it was due has to come back out of
    /// The window guard is what actually stops a looping emitter: it returns before the birth
    /// clock is ever consulted, so getting `emitting_before` right on its own changes nothing.
    /// This drives a whole system past its emitter's span and asks whether anything is alive.
    #[test]
    fn a_looping_emitter_keeps_a_population_past_its_own_span() {
        let bytes = std::fs::read("tests/20.nif").expect("fixture");
        let nif = crate::Nif::parse(&mut std::io::Cursor::new(&bytes)).expect("parse");

        let mut looping = 0;
        for block in nif.blocks.iter() {
            let Block::NiPSysEmitterCtlr(_) = block else {
                continue;
            };
            let time = block.as_time_controller().expect("a time controller");
            if matches!(time.cycle_type_enum(), crate::anim::CycleType::Loop) && time.is_active() {
                looping += 1;
            }
        }
        assert!(looping > 0, "the fixture has no looping emitter to drive");

        // well past any emitter span in the fixture, where the old guard had gone quiet
        let mut alive = 0;
        for mut system in systems(&nif.blocks) {
            system.seek(&nif.blocks, 30.0);
            alive += system.particles().len();
        }
        assert!(alive > 0, "a looping emitter emitted nothing past its own span");
    }

    /// A looping emitter fires again every time its span comes round. Counting only the first
    /// pass leaves it dark for the rest of the file, which on a long one is nearly all of it:
    /// the pattern here is on for a third of a span that repeats for ten times its length.
    #[test]
    fn a_looping_emitter_fires_again_every_pass() {
        let blinking = BirthRate {
            per_second: 9.0,
            window: (0.0, 3.0),
            on: vec![(0.0, 1.0)],
            repeats: true,
        };

        // one second of emitting per pass, so the total climbs by one every three seconds
        assert!((blinking.emitting_before(1.0) - 1.0).abs() < 1e-5);
        assert!((blinking.emitting_before(3.0) - 1.0).abs() < 1e-5);
        assert!((blinking.emitting_before(4.0) - 2.0).abs() < 1e-5);
        assert!((blinking.emitting_before(30.0) - 10.0).abs() < 1e-5);

        // the same emitter clamped stops after its one pass, however long the file runs
        let once = BirthRate {
            repeats: false,
            ..BirthRate {
                per_second: 9.0,
                window: (0.0, 3.0),
                on: vec![(0.0, 1.0)],
                repeats: true,
            }
        };
        assert!((once.emitting_before(30.0) - 1.0).abs() < 1e-5);

        // and the two directions still invert each other across many passes, or a particle born
        // late is aged by every gap it slept through
        for tenth in 0..=100 {
            let elapsed = tenth as f32 * 0.1;
            let moment = blinking.moment_at(elapsed);
            assert!(
                (blinking.emitting_before(moment) - elapsed).abs() < 1e-4,
                "elapsed {elapsed} came back as {moment}"
            );
        }
    }

    /// emitting time. Getting this wrong ages a particle by the gaps between bursts.
    #[test]
    fn a_due_moment_converts_back_out_of_emitting_time() {
        let twice = BirthRate {
            per_second: 10.0,
            window: (0.0, 4.0),
            on: vec![(0.0, 1.0), (2.0, 3.0)],
            repeats: false,
        };
        // the gap contributes nothing to either direction
        assert!((twice.emitting_before(2.5) - 1.5).abs() < 1e-6);
        assert!((twice.moment_at(1.5) - 2.5).abs() < 1e-6);
        // and the two are inverses across the whole span
        for tenth in 0..=20 {
            let elapsed = tenth as f32 * 0.1;
            let moment = twice.moment_at(elapsed);
            assert!(
                (twice.emitting_before(moment) - elapsed).abs() < 1e-5,
                "elapsed {elapsed} came back as {moment}"
            );
        }
        // past the last interval it holds at its end rather than running away
        assert!((twice.moment_at(9.0) - 3.0).abs() < 1e-6);
    }

    /// The keys are a step function, and pairing them wrongly is the difference between a burst
    /// and a shader that never stops. A track that only ever switches on runs to the end.
    #[test]
    fn a_visibility_track_pairs_its_keys_into_intervals() {
        let keys = |values: &[(f32, u8)]| {
            let keys = values
                .iter()
                .map(|(time, value)| Key {
                    time: *time,
                    value: *value,
                    in_tangent: None,
                    out_tangent: None,
                    tbc: None,
                })
                .collect();
            vec![
                Block::NiBoolData(NiBoolData {
                    data: KeyGroup {
                        keys,
                        interpolation: Some(KeyType::Const),
                    },
                }),
                Block::NiBoolInterpolator(NiBoolInterpolator {
                    base: NiKeyBasedInterpolator {
                        base: NiInterpolator {},
                    },
                    value: 2,
                    data_ref: BlockRef::Index(0),
                }),
            ]
        };
        let window = (0.0, 3.0);
        let at = BlockRef::Index(1);

        // on, then off part way: one interval
        let blocks = keys(&[(0.0, 1), (0.5, 0), (3.0, 0)]);
        assert_eq!(switched_on(&blocks, at, window), vec![(0.0, 0.5)]);

        // off, then on later: emission starts late
        let blocks = keys(&[(0.0, 0), (1.0, 1), (2.0, 0)]);
        assert_eq!(switched_on(&blocks, at, window), vec![(1.0, 2.0)]);

        // never switched off, so it runs to the end of the span
        let blocks = keys(&[(0.0, 1)]);
        assert_eq!(switched_on(&blocks, at, window), vec![(0.0, 3.0)]);

        // nothing pointing at a track leaves it on throughout
        assert_eq!(switched_on(&[], BlockRef::None, window), vec![window]);
    }

    /// A colour track is read at how far through its life a particle is, not at the clock, and
    /// outside the track's own range the nearest key holds. A track that starts after birth or
    /// ends before death must not leave a particle uncoloured at either end.
    #[test]
    fn a_colour_track_is_read_across_a_life_and_holds_outside_it() {
        let keys = vec![
            Key {
                time: 0.25,
                value: Color4 {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
                in_tangent: None,
                out_tangent: None,
                tbc: None,
            },
            Key {
                time: 0.75,
                value: Color4 {
                    r: 0.0,
                    g: 0.0,
                    b: 1.0,
                    a: 0.0,
                },
                in_tangent: None,
                out_tangent: None,
                tbc: None,
            },
        ];
        let data = NiColorData {
            data: KeyGroup {
                keys,
                interpolation: Some(KeyType::Linear),
            },
        };
        let blocks = vec![Block::NiColorData(data)];
        let modifier = NiPSysColorModifier {
            base: NiPSysModifier {
                name: NiString::from("colour"),
                order: 3000,
                target_ref: BlockRef::None,
                active: true,
            },
            data_ref: BlockRef::Index(0),
        };

        let make = |age: f32| Particle {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            age,
            life_span: 1.0,
            radius: 1.0,
            size: 1.0,
            rotation: 0.0,
            rotation_speed: 0.0,
            color: Color4::default(),
            generation: 0,
            last_update: 0.0,
        };
        let mut system = System {
            block: 0,
            particles: vec![make(0.0), make(0.5), make(1.0)],
            capacity: 4,
            time: 0.0,
            rng: Rng::new(1),
            seed: 1,
            spaces: Default::default(),
        };
        system.tint(&blocks, &modifier);

        // before the first key, that key holds
        assert_eq!(system.particles[0].color.r, 1.0);
        assert_eq!(system.particles[0].color.a, 1.0);
        // half way between the two
        assert!((system.particles[1].color.r - 0.5).abs() < 1e-6);
        assert!((system.particles[1].color.a - 0.5).abs() < 1e-6);
        // past the last key, that key holds
        assert_eq!(system.particles[2].color.b, 1.0);
        assert_eq!(system.particles[2].color.a, 0.0);
    }

    /// A grow and fade modifier drives the size rather than the radius, and the two ends are
    /// separate spans. A particle in neither is at full size, so a file that only grows must not
    /// leave everything shrinking.
    #[test]
    fn growing_and_fading_scale_the_ends_of_a_life_and_nothing_between() {
        let mut particle = Particle {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            age: 0.0,
            life_span: 10.0,
            radius: 2.0,
            size: 1.0,
            rotation: 0.0,
            rotation_speed: 0.0,
            color: Color4::default(),
            generation: 0,
            last_update: 0.0,
        };
        let modifier = |grow: f32, fade: f32| NiPSysGrowFadeModifier {
            base: NiPSysModifier {
                name: NiString::from("grow"),
                order: 3000,
                target_ref: BlockRef::None,
                active: true,
            },
            grow_time: grow,
            grow_generation: 0,
            fade_time: fade,
            fade_generation: 0,
        };

        let mut system = System {
            block: 0,
            particles: Vec::new(),
            capacity: 4,
            time: 0.0,
            rng: Rng::new(1),
            seed: 1,
            spaces: Default::default(),
        };

        // half way into a two second grow
        particle.age = 1.0;
        system.particles = vec![particle];
        system.grow_and_fade(&modifier(2.0, 2.0));
        assert!((system.particles[0].size - 0.5).abs() < 1e-6);
        // the radius itself is untouched; only what it draws at changes
        assert_eq!(system.particles[0].radius, 2.0);
        assert!((system.particles[0].drawn_radius() - 1.0).abs() < 1e-6);

        // the middle of a life is in neither span
        particle.age = 5.0;
        system.particles = vec![particle];
        system.grow_and_fade(&modifier(2.0, 2.0));
        assert_eq!(system.particles[0].size, 1.0);

        // one second left of a two second fade
        particle.age = 9.0;
        system.particles = vec![particle];
        system.grow_and_fade(&modifier(2.0, 2.0));
        assert!((system.particles[0].size - 0.5).abs() < 1e-6);

        // a zero time turns that half off rather than dividing by it
        particle.age = 9.5;
        system.particles = vec![particle];
        system.grow_and_fade(&modifier(2.0, 0.0));
        assert_eq!(system.particles[0].size, 1.0);
        assert!(system.particles[0].size.is_finite());
    }

    #[test]
    fn particles_die_when_their_span_runs_out() {
        let nif = parse("tests/20.nif");
        for system in &mut systems(&nif.blocks) {
            system.seek(&nif.blocks, 2.0);
            for particle in system.particles() {
                assert!(
                    particle.age <= particle.life_span,
                    "a particle outlived its span"
                );
                assert!(particle.age >= 0.0, "a particle has a negative age");
            }
        }
    }
}
