use crate::blocks::{Block, NiLODNode};
use crate::common::{BlockRef, NiTransform};

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

/// The properties in force on an object, gathered down the graph. A property attached to a node
/// applies to everything under it, and one of the same kind nearer the object replaces it, so at
/// most one of each kind is ever in force.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Properties {
    pub alpha: BlockRef,
    pub dither: BlockRef,
    pub material: BlockRef,
    pub shade: BlockRef,
    pub specular: BlockRef,
    pub stencil: BlockRef,
    pub texturing: BlockRef,
    pub vertex_color: BlockRef,
    pub wireframe: BlockRef,
    pub z_buffer: BlockRef,
}

impl Properties {
    /// `refs` are an object's own, which replace whatever came from above it.
    fn with(mut self, blocks: &[Block], refs: &[BlockRef]) -> Properties {
        for r in refs {
            let Some(block) = r.get(blocks) else {
                continue;
            };
            let slot = match block {
                Block::NiAlphaProperty(_) => &mut self.alpha,
                Block::NiDitherProperty(_) => &mut self.dither,
                Block::NiMaterialProperty(_) => &mut self.material,
                Block::NiShadeProperty(_) => &mut self.shade,
                Block::NiSpecularProperty(_) => &mut self.specular,
                Block::NiStencilProperty(_) => &mut self.stencil,
                Block::NiTexturingProperty(_) => &mut self.texturing,
                Block::NiVertexColorProperty(_) => &mut self.vertex_color,
                Block::NiWireframeProperty(_) => &mut self.wireframe,
                Block::NiZBufferProperty(_) => &mut self.z_buffer,
                _ => continue,
            };
            *slot = *r;
        }
        self
    }
}

/// The lights reaching an object, gathered down the graph. An effect attached to a node reaches
/// everything under it and adds to whatever came from above, where a property of the same kind
/// would have replaced it.
///
/// Fixed at `Lights::MAX`, which is the engine's own limit on how many it will gather. A file
/// holding more than that loses the ones it names last.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Lights {
    refs: [BlockRef; Lights::MAX],
    count: usize,
}

impl Lights {
    pub const MAX: usize = 8;

    /// Every light in force, nearest the root first.
    pub fn iter(&self) -> impl Iterator<Item = BlockRef> + '_ {
        self.refs[..self.count].iter().copied()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// `refs` are the effects a node holds, which join whatever is already in force. A texture
    /// effect sits in the same list and is passed over, as is a light switched off or dimmed to
    /// nothing, which the engine skips rather than applying dark.
    fn with(mut self, blocks: &[Block], refs: &[BlockRef]) -> Lights {
        for r in refs {
            if self.count >= Lights::MAX {
                break;
            }
            let Some(light) = r.get(blocks).and_then(Block::light) else {
                continue;
            };
            if !light.switch_state || light.dimmer < 0.01 {
                continue;
            }
            self.refs[self.count] = *r;
            self.count += 1;
        }
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Visit<'a> {
    pub index: usize,
    pub block: &'a Block,
    pub transform: NiTransform,
    pub depth: usize,
    /// Culled, along with everything under it. Either the object's own flag or, when the walk
    /// is at a time, whatever its visibility controller says instead.
    pub hidden: bool,
    /// What applies here, including whatever was attached further up the graph.
    pub properties: Properties,
    /// The lights reaching here, from this object and every node above it.
    pub lights: Lights,
}

struct Frame {
    index: usize,
    parent: NiTransform,
    depth: usize,
    hidden: bool,
    properties: Properties,
    lights: Lights,
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
                properties: Properties::default(),
                lights: Lights::default(),
                hidden: false,
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
                properties: Properties::default(),
                lights: Lights::default(),
                parent: NiTransform::IDENTITY,
                depth: 0,
                hidden: false,
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

            let hidden = frame.hidden
                || match block.av_object() {
                    Some(av) => {
                        #[cfg(feature = "glam")]
                        let shown = self
                            .time
                            .and_then(|time| crate::anim::visible_at(self.blocks, av, time));
                        #[cfg(not(feature = "glam"))]
                        let shown: Option<bool> = None;
                        shown.map_or(av.is_hidden(), |shown| !shown)
                    }
                    None => false,
                };

            // an object's own replace what came from above, and the result is what its
            // children start from
            let properties = frame
                .properties
                .with(self.blocks, block.property_refs().unwrap_or(&[]));
            let lights = frame
                .lights
                .with(self.blocks, block.effect_refs().unwrap_or(&[]));

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
                            hidden,
                            properties,
                            lights,
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
                hidden,
                properties,
                lights,
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Nif;

    fn node(children: Vec<u32>, effects: Vec<u32>) -> Block {
        Block::NiNode(crate::blocks::NiNode {
            base: av_object(),
            child_refs: children.into_iter().map(BlockRef::Index).collect(),
            effect_refs: effects.into_iter().map(BlockRef::Index).collect(),
        })
    }

    fn av_object() -> crate::blocks::NiAvObject {
        crate::blocks::NiAvObject {
            base: crate::blocks::NiObjectNET {
                name: crate::blocks::NiString::from(""),
                extra_data_refs: Vec::new(),
                controller_ref: BlockRef::None,
            },
            flags: 0,
            translation: crate::common::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: crate::common::Matrix33::IDENTITY,
            scale: 1.0,
            property_refs: Vec::new(),
            collision_ref: BlockRef::None,
        }
    }

    fn directional(switch_state: bool, dimmer: f32) -> Block {
        let grey = crate::common::Color3 {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };
        Block::NiDirectionalLight(crate::blocks::NiDirectionalLight {
            base: crate::blocks::NiLight {
                base: crate::blocks::NiDynamicEffect {
                    base: av_object(),
                    switch_state,
                    unaffected_node_refs: Vec::new(),
                },
                dimmer,
                ambient_color: grey,
                diffuse_color: grey,
                specular_color: grey,
            },
        })
    }

    fn shape() -> Block {
        Block::NiTriShape(crate::blocks::NiTriShape {
            base: crate::blocks::NiGeometry {
                base: av_object(),
                data_ref: BlockRef::None,
                skin_instance_ref: BlockRef::None,
                material_data: crate::blocks::MaterialData::None,
            },
        })
    }

    fn lights_on_the_shape(blocks: &[Block], at: usize) -> Vec<BlockRef> {
        Walk::from_roots(blocks, [0])
            .find(|v| v.index == at)
            .expect("the shape is reachable")
            .lights
            .iter()
            .collect()
    }

    /// An effect on a node reaches every shape under it, however deep, which is the whole reason
    /// this is gathered by the walk rather than read off the shape. Nearly every light in this
    /// game hangs off the root, so reading a shape's own list alone would find none of them.
    #[test]
    fn a_light_on_a_node_reaches_the_shapes_under_it() {
        // root(effect 3) > node > shape, with the light off to the side
        let blocks = vec![node(vec![1], vec![3]), node(vec![2], vec![]), shape(), directional(true, 1.0)];
        assert_eq!(lights_on_the_shape(&blocks, 2), vec![BlockRef::Index(3)]);
    }

    /// Lights add rather than replace, which is where they part company with properties: two
    /// nodes each holding one leave both in force on what sits below them.
    #[test]
    fn lights_accumulate_where_a_property_would_replace() {
        let blocks = vec![
            node(vec![1], vec![3]),
            node(vec![2], vec![4]),
            shape(),
            directional(true, 1.0),
            directional(true, 1.0),
        ];
        assert_eq!(
            lights_on_the_shape(&blocks, 2),
            vec![BlockRef::Index(3), BlockRef::Index(4)]
        );
    }

    /// A light reaches down, never sideways, so one held by a sibling branch is not in force.
    #[test]
    fn a_light_does_not_reach_a_sibling_branch() {
        let blocks = vec![
            node(vec![1, 3], vec![]),
            node(vec![2], vec![4]),
            shape(),
            shape(),
            directional(true, 1.0),
        ];
        assert_eq!(lights_on_the_shape(&blocks, 2), vec![BlockRef::Index(4)]);
        assert!(lights_on_the_shape(&blocks, 3).is_empty());
    }

    /// The engine skips a light that is switched off or dimmed to nothing rather than carrying it
    /// and applying it dark, so neither reaches the shape at all.
    #[test]
    fn a_light_switched_off_or_dimmed_out_is_left_behind() {
        for light in [directional(false, 1.0), directional(true, 0.0)] {
            let blocks = vec![node(vec![1], vec![2]), shape(), light];
            assert!(
                lights_on_the_shape(&blocks, 1).is_empty(),
                "a light the engine would skip was carried anyway"
            );
        }
        // and the same light switched on does reach it, so the test is not passing vacuously
        let blocks = vec![node(vec![1], vec![2]), shape(), directional(true, 1.0)];
        assert_eq!(lights_on_the_shape(&blocks, 1), vec![BlockRef::Index(2)]);
    }

    /// A texture effect shares the effect list with the lights and is not one, so it is passed
    /// over rather than counted.
    #[test]
    fn only_lights_come_out_of_the_effect_list() {
        let blocks = vec![
            node(vec![1], vec![2, 3]),
            shape(),
            directional(true, 1.0),
            node(vec![], vec![]),
        ];
        assert_eq!(lights_on_the_shape(&blocks, 1), vec![BlockRef::Index(2)]);
    }

    /// A property attached to a node applies to everything under it. Reading only an object's own
    /// list leaves a shape with the wrong depth, blending or material, and most of the shapes in
    /// this game inherit at least one.
    #[test]
    fn a_property_on_a_node_reaches_the_shapes_under_it() {
        let bytes = std::fs::read("tests/11.nif").expect("fixture");
        let nif = Nif::parse(&mut std::io::Cursor::new(&bytes)).expect("parse");

        let mut inherited = 0;
        for visit in nif.walk() {
            let Some(geometry) = visit.block.geometry() else {
                continue;
            };
            let own = &geometry.property_refs;
            for slot in [visit.properties.z_buffer, visit.properties.vertex_color] {
                if slot.index().is_some() && !own.contains(&slot) {
                    inherited += 1;
                }
            }
        }
        assert!(
            inherited > 0,
            "no shape inherited a property, so the fixture cannot show the accumulation"
        );
    }

    #[test]
    fn an_objects_own_property_replaces_the_one_above_it() {
        let bytes = std::fs::read("tests/11.nif").expect("fixture");
        let nif = Nif::parse(&mut std::io::Cursor::new(&bytes)).expect("parse");

        for visit in nif.walk() {
            let Some(geometry) = visit.block.geometry() else {
                continue;
            };
            // whatever a shape carries has to be what is in force, never an ancestor's
            for r in &geometry.property_refs {
                let Some(Block::NiZBufferProperty(_)) = r.get(&nif.blocks) else {
                    continue;
                };
                assert_eq!(
                    visit.properties.z_buffer, *r,
                    "an ancestor's z buffer property won over the shape's own"
                );
            }
        }
    }
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
