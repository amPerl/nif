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

use glam::Vec3;

use crate::blocks::{Block, NiPSysColorModifier, NiPSysEmitter, NiPSysGrowFadeModifier};
use crate::common::{BlockRef, Color4, Vector3};

/// The interval the simulation advances by. Forces apply before movement within a step, so how
/// far a particle travels depends on how coarse the step is and only a fixed one repeats.
pub const STEP: f32 = 1.0 / 60.0;

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
    pub color: Color4,
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
        })
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn time(&self) -> f32 {
        self.time
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
                Some(Block::NiPSysColorModifier(m)) => self.tint(blocks, m),
                _ => match order {
                    order::AGE_DEATH => self.age_and_die(now),
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

    /// Age each particle by the delta it last saw, then drop the ones past their span.
    fn age_and_die(&mut self, now: f32) {
        for particle in &mut self.particles {
            particle.age += now - particle.last_update;
        }
        self.particles
            .retain(|particle| particle.age <= particle.life_span);
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
        if rate.per_second <= 0.0 || start >= stop || now <= start || last >= stop {
            return;
        }

        // emission counts against how long the emitter has been on, not against the clock
        let current_delta = rate.emitting_before(now);
        let last_delta = rate.emitting_before(last);
        let current_count = (rate.per_second * current_delta) as u32;
        let last_count = (rate.per_second * last_delta) as u32;
        let interval = 1.0 / rate.per_second;

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
            place(blocks, index, &mut particle, &mut self.rng);
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
}

impl BirthRate {
    /// How long the emitter has been switched on by `time`, which is the clock emission counts
    /// against. A track that is on throughout makes this the plain elapsed time.
    fn emitting_before(&self, time: f32) -> f32 {
        self.on
            .iter()
            .map(|(from, to)| (time.min(*to) - from).max(0.0))
            .sum()
    }

    /// The moment an emitter that had been on for `elapsed` reached it, which is the inverse of
    /// `emitting_before`. Past the end of the last interval it holds there, since nothing is born
    /// after that anyway.
    fn moment_at(&self, elapsed: f32) -> f32 {
        let mut left = elapsed;
        for (from, to) in &self.on {
            let length = to - from;
            if left <= length {
                return from + left;
            }
            left -= length;
        }
        self.on.last().map_or(self.window.0, |(_, to)| *to)
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
        // full size until something scales it
        size: 1.0,
        color: emitter.initial_color,
        last_update: 0.0,
    })
}

/// Where in its volume an emitter starts a particle. Only the box is placed for now; the others
/// start at the emitter's origin, which is where a volume's centre sits anyway.
fn place(blocks: &[Block], index: usize, particle: &mut Particle, rng: &mut Rng) {
    if let Some(Block::NiPSysBoxEmitter(box_emitter)) = blocks.get(index) {
        particle.position = Vector3 {
            x: box_emitter.width * (rng.unit() - 0.5),
            y: box_emitter.height * (rng.unit() - 0.5),
            z: box_emitter.depth * (rng.unit() - 0.5),
        };
    }
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
    use crate::blocks::{
        NiBoolData, NiBoolInterpolator, NiColorData, NiInterpolator, NiKeyBasedInterpolator,
        NiPSysModifier, NiString,
    };
    use crate::common::{BlockRef, Key, KeyGroup, KeyType};

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

    /// An emitter is switched on and off by a track of its own, and emission counts against how
    /// long it has actually been on rather than against the clock. Without that a burst emitter
    /// runs for its whole span, emitting long after it should stop.
    #[test]
    fn emission_counts_only_the_time_an_emitter_is_switched_on() {
        let burst = BirthRate {
            per_second: 300.0,
            window: (0.0, 3.3333333),
            on: vec![(0.0, 0.16666667)],
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
        };
        assert!((steady.emitting_before(1.0) - 1.0).abs() < 1e-6);
    }

    /// A particle's age is real elapsed time, so the moment it was due has to come back out of
    /// emitting time. Getting this wrong ages a particle by the gaps between bursts.
    #[test]
    fn a_due_moment_converts_back_out_of_emitting_time() {
        let twice = BirthRate {
            per_second: 10.0,
            window: (0.0, 4.0),
            on: vec![(0.0, 1.0), (2.0, 3.0)],
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
            color: Color4::default(),
            last_update: 0.0,
        };
        let mut system = System {
            block: 0,
            particles: vec![make(0.0), make(0.5), make(1.0)],
            capacity: 4,
            time: 0.0,
            rng: Rng::new(1),
            seed: 1,
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
            color: Color4::default(),
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
