use crate::blocks::{Block, NiLODNode};
use crate::common::NiTransform;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LodPolicy {
    /// Every level, which draws them over each other.
    #[default]
    All,
    /// The most detailed level, which is the one whose range starts nearest the viewer.
    Highest,
    /// The level whose range covers this distance.
    Distance(f32),
}

#[derive(Debug, Clone, Copy)]
pub struct Visit<'a> {
    pub index: usize,
    pub block: &'a Block,
    pub transform: NiTransform,
    pub depth: usize,
}

struct Frame {
    index: usize,
    parent: NiTransform,
    depth: usize,
}

pub struct Walk<'a> {
    blocks: &'a [Block],
    stack: Vec<Frame>,
    path: Vec<usize>,
    lod: LodPolicy,
    #[cfg(feature = "glam")]
    time: Option<f32>,
    #[cfg(feature = "glam")]
    camera: Option<crate::billboard::Camera>,
}

enum Selection {
    All,
    One(usize),
}

impl<'a> Walk<'a> {
    pub fn new(blocks: &'a [Block], root: usize) -> Self {
        Self {
            blocks,
            stack: vec![Frame {
                index: root,
                parent: NiTransform::IDENTITY,
                depth: 0,
            }],
            path: Vec::new(),
            lod: LodPolicy::All,
            #[cfg(feature = "glam")]
            time: None,
            #[cfg(feature = "glam")]
            camera: None,
        }
    }

    pub fn from_roots(blocks: &'a [Block], roots: impl IntoIterator<Item = usize>) -> Self {
        let mut stack: Vec<Frame> = roots
            .into_iter()
            .map(|index| Frame {
                index,
                parent: NiTransform::IDENTITY,
                depth: 0,
            })
            .collect();
        stack.reverse();
        Self {
            blocks,
            stack,
            path: Vec::new(),
            lod: LodPolicy::All,
            #[cfg(feature = "glam")]
            time: None,
            #[cfg(feature = "glam")]
            camera: None,
        }
    }

    pub fn with_lod(mut self, lod: LodPolicy) -> Self {
        self.lod = lod;
        self
    }

    /// Compose each object's transform as its controllers leave it at `time`, rather than as
    /// the file stores it.
    #[cfg(feature = "glam")]
    pub fn at_time(mut self, time: f32) -> Self {
        self.time = Some(time);
        self
    }

    /// Turn NiBillboardNode subtrees to face this camera, which is what makes them billboards.
    /// Without it they keep the orientation the file stores.
    #[cfg(feature = "glam")]
    pub fn seen_from(mut self, camera: crate::billboard::Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    fn select_lod(&self, node: &NiLODNode, child_count: usize) -> Selection {
        if child_count == 0 {
            return Selection::All;
        }
        if self.lod == LodPolicy::All {
            return Selection::All;
        }
        let ranges = match node.lod_level_data_ref.get(self.blocks) {
            Some(Block::NiRangeLODData(data)) => &data.lod_levels,
            _ => return Selection::One(0),
        };
        // levels are not ordered by detail, so the ranges decide which one is which
        let picked = match self.lod {
            LodPolicy::All => 0,
            LodPolicy::Highest => ranges
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.near.total_cmp(&b.near))
                .map_or(0, |(index, _)| index),
            LodPolicy::Distance(d) => ranges
                .iter()
                .position(|r| d >= r.near && d < r.far)
                .unwrap_or(0),
        };
        Selection::One(picked.min(child_count.saturating_sub(1)))
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = Visit<'a>;

    fn next(&mut self) -> Option<Visit<'a>> {
        while let Some(frame) = self.stack.pop() {
            self.path.truncate(frame.depth);
            if self.path.contains(&frame.index) {
                continue;
            }

            let Some(block) = self.blocks.get(frame.index) else {
                continue;
            };
            self.path.push(frame.index);

            let transform = match block.av_object() {
                Some(av) => {
                    #[cfg(feature = "glam")]
                    let local = self
                        .time
                        .and_then(|time| crate::anim::transform_at(self.blocks, av, time))
                        .unwrap_or_else(|| NiTransform::from(av));
                    #[cfg(not(feature = "glam"))]
                    let local = NiTransform::from(av);

                    #[allow(unused_mut)]
                    let mut world = frame.parent.compose(&local);
                    // the world transform is what gets oriented, so the whole subtree
                    // inherits the turn
                    #[cfg(feature = "glam")]
                    if let (Block::NiBillboardNode(node), Some(camera)) = (block, self.camera) {
                        if let Some(rotation) = node.billboard_mode.orient(&world, &camera) {
                            world.rotation = rotation;
                        }
                    }
                    world
                }
                None => frame.parent,
            };

            let children = block.child_refs().unwrap_or(&[]);
            if !children.is_empty() {
                let selection = match block {
                    Block::NiLODNode(lod) => self.select_lod(lod, children.len()),
                    _ => Selection::All,
                };
                let queue = |stack: &mut Vec<Frame>, r: &crate::common::BlockRef| {
                    if let Some(index) = r.index() {
                        stack.push(Frame {
                            index,
                            parent: transform,
                            depth: frame.depth + 1,
                        });
                    }
                };
                match selection {
                    Selection::All => children
                        .iter()
                        .rev()
                        .for_each(|r| queue(&mut self.stack, r)),
                    Selection::One(i) => {
                        if let Some(r) = children.get(i) {
                            queue(&mut self.stack, r);
                        }
                    }
                }
            }

            return Some(Visit {
                index: frame.index,
                block,
                transform,
                depth: frame.depth,
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Nif;
    use std::io::Cursor;

    fn load(n: u32) -> Nif {
        let bytes = std::fs::read(format!("tests/{}.nif", n)).expect("read nif");
        Nif::parse(&mut Cursor::new(bytes)).expect("parse nif")
    }

    #[test]
    fn depth_increases_one_step_at_a_time() {
        let nif = load(1);
        let visits: Vec<_> = Walk::new(&nif.blocks, 0).collect();

        assert!(visits.len() > 1);
        assert_eq!(visits[0].index, 0);
        assert_eq!(visits[0].depth, 0);
        for pair in visits.windows(2) {
            assert!(pair[1].depth <= pair[0].depth + 1);
        }
    }

    #[test]
    fn terminates_on_every_fixture() {
        for n in 1..=26 {
            let nif = load(n);
            let budget = nif.blocks.len() * 4 + 16;
            let count = Walk::new(&nif.blocks, 0).take(budget).count();
            assert!(count < budget, "file {} did not terminate", n);
        }
    }

    #[test]
    fn out_of_range_root_yields_nothing() {
        let nif = load(1);
        assert_eq!(Walk::new(&nif.blocks, 9999).count(), 0);
    }

    #[test]
    fn from_roots_visits_each_root() {
        let nif = load(1);
        let single = Walk::new(&nif.blocks, 0).count();
        let doubled = Walk::from_roots(&nif.blocks, [0, 0]).count();
        assert_eq!(doubled, single * 2);
    }

    #[test]
    fn highest_picks_the_nearest_range_not_the_first_child() {
        let nif = load(3);
        let (index, node) = nif
            .blocks
            .iter()
            .enumerate()
            .find_map(|(index, block)| match block {
                Block::NiLODNode(node) => Some((index, node)),
                _ => None,
            })
            .expect("fixture 3 has a lod node");
        let Some(Block::NiRangeLODData(data)) = node.lod_level_data_ref.get(&nif.blocks) else {
            panic!("fixture 3 has range data");
        };
        assert!(data.lod_levels.len() > 1);

        let nearest = data
            .lod_levels
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.near.total_cmp(&b.near))
            .map(|(level, _)| level)
            .expect("a level");
        assert_ne!(nearest, 0, "this file would not prove anything otherwise");

        let walk = Walk::new(&nif.blocks, 0).with_lod(LodPolicy::Highest);
        let reached: Vec<usize> = walk.map(|visit| visit.index).collect();
        let children = nif.blocks[index].child_refs().expect("children");
        let kept = children[nearest].index().expect("a child");
        let dropped = children[0].index().expect("a child");
        assert!(
            reached.contains(&kept),
            "the detailed level should be walked"
        );
        assert!(!reached.contains(&dropped), "the far level should not be");
    }

    #[test]
    fn lod_policy_narrows_what_is_reached() {
        for n in 1..=26 {
            let nif = load(n);
            if !nif.blocks.iter().any(|b| matches!(b, Block::NiLODNode(_))) {
                continue;
            }
            let all = Walk::new(&nif.blocks, 0).count();
            let highest = Walk::new(&nif.blocks, 0)
                .with_lod(LodPolicy::Highest)
                .count();
            assert!(highest <= all, "file {}: {} vs {}", n, highest, all);
            return;
        }
    }
}
