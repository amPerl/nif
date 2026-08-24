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

use crate::blocks::{Block, NiPSysEmitter};
use crate::common::{Color4, Vector3};

/// The interval the simulation advances by. Forces apply before movement within a step, so how
/// far a particle travels depends on how coarse the step is and only a fixed one repeats.
pub const STEP: f32 = 1.0 / 60.0;

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
    pub color: Color4,
    /// When it was last advanced. Everything in a step measures against this, and only the
    /// move at the end of the step carries it forward.
    last_update: f32,
}

impl Particle {
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

        for (order, index) in modifiers {
            match order {
                order::AGE_DEATH => self.age_and_die(now),
                order::EMIT => self.emit_from(blocks, index, last, now),
                order::POSITION => self.integrate(now),
                _ => {}
            }
        }
        self.time = now;
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
        let Some(rate) = birth_rate(blocks, index) else {
            return;
        };
        let (start, stop) = rate.window;
        if rate.per_second <= 0.0 || start >= stop || now <= start || last >= stop {
            return;
        }

        let current_delta = if now <= stop {
            now - start
        } else {
            stop - start
        };
        let last_delta = if last >= start { last - start } else { 0.0 };
        let current_count = (rate.per_second * current_delta) as u32;
        let last_count = (rate.per_second * last_delta) as u32;
        let interval = 1.0 / rate.per_second;

        for born in last_count..current_count {
            if self.particles.len() >= self.capacity {
                return;
            }
            let age = current_delta - (born + 1) as f32 * interval;
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
}

/// The birth rate driving this emitter. A controller names the modifier it drives rather than
/// pointing at it, so the two are matched by name.
fn birth_rate(blocks: &[Block], modifier: usize) -> Option<BirthRate> {
    let name = &blocks.get(modifier)?.as_psys_modifier()?.name;
    for block in blocks {
        let Block::NiPSysEmitterCtlr(controller) = block else {
            continue;
        };
        if controller.modifier_name != *name {
            continue;
        }
        let time = &controller.base.base.base;
        let keyed = match controller.interpolator_ref.get(blocks) {
            Some(Block::NiFloatInterpolator(interpolator)) => {
                match interpolator.data_ref.get(blocks) {
                    Some(Block::NiFloatData(data)) => data.data.sample(time.start_time),
                    _ => Some(interpolator.value),
                }
            }
            _ => None,
        };
        return Some(BirthRate {
            per_second: keyed.unwrap_or(0.0),
            window: (time.start_time, time.end_time),
        });
    }
    None
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
