use crate::blocks::{Block, NiLODNode};
use crate::common::NiTransform;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LodPolicy {
    #[default]
    All,
    Highest,
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
        }
    }

    pub fn with_lod(mut self, lod: LodPolicy) -> Self {
        self.lod = lod;
        self
    }

    fn select_lod(&self, node: &NiLODNode, child_count: usize) -> Selection {
        if child_count == 0 {
            return Selection::All;
        }
        match self.lod {
            LodPolicy::All => Selection::All,
            LodPolicy::Highest => Selection::One(0),
            LodPolicy::Distance(d) => {
                let ranges = match node.lod_level_data_ref.get(self.blocks) {
                    Some(Block::NiRangeLODData(data)) => &data.lod_levels,
                    _ => return Selection::One(0),
                };
                let picked = ranges
                    .iter()
                    .position(|r| d >= r.near && d < r.far)
                    .unwrap_or(0);
                Selection::One(picked.min(child_count.saturating_sub(1)))
            }
        }
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
                Some(av) => frame.parent.compose(&NiTransform::from(av)),
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
