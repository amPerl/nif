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
    /// effect sits in the same list and is passed over, as is a light switched off.
    ///
    /// A light's dimmer is deliberately not read here. The engine drops a light dimmed below
    /// 0.01 as it draws, against whatever the dimmer is at that moment, and a light stored at
    /// zero to be driven up by a controller is exactly what a static reading would throw away.
    fn with(mut self, blocks: &[Block], refs: &[BlockRef]) -> Lights {
        for r in refs {
            if self.count >= Lights::MAX {
                break;
            }
            let Some(light) = r.get(blocks).and_then(Block::light) else {
                continue;
            };
            if !light.switch_state {
                continue;
            }
            self.refs[self.count] = *r;
            self.count += 1;
        }
        self
    }
}

/// The texture effects reaching an object, gathered down the graph the same way the lights are.
/// An effect list holds both, and which is which is the block's own business.
///
/// One is all the engine reads: it takes the first environment map and leaves any others alone.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Effects {
    environment: BlockRef,
}

impl Effects {
    /// The environment map in force, which is the nearest to the root that switched itself on.
    pub fn environment(&self) -> BlockRef {
        self.environment
    }

    /// `refs` are the effects a node holds. A light sits in the same list and is gathered
    /// separately, and an effect switched off is left out the way the engine leaves it out.
    fn with(mut self, blocks: &[Block], refs: &[BlockRef]) -> Effects {
        for r in refs {
            let Some(Block::NiTextureEffect(effect)) = r.get(blocks) else {
                continue;
            };
            if !effect.switch_state {
                continue;
            }
            if self.environment.index().is_none() {
                self.environment = *r;
            }
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
    /// The texture effects reaching here, gathered the same way.
    pub effects: Effects,
}

struct Frame {
    index: usize,
    parent: NiTransform,
    depth: usize,
    hidden: bool,
    properties: Properties,
    lights: Lights,
    effects: Effects,
}

/// Which LOD node each block sits under, and which of its levels it is.
///
/// A level is a child's place in the node's child list, which is what the ranges are counted
/// against. Blocks reached without passing through a LOD node are absent; a node inside another
/// node's level claims its own children, since it is the nearer one that decides them.
pub fn lod_ancestry(nif: &crate::Nif) -> std::collections::HashMap<usize, (usize, usize)> {
    let mut found = std::collections::HashMap::new();
    let mut seen = std::collections::HashSet::new();
    let mut stack: Vec<(usize, Option<(usize, usize)>)> =
        nif.roots().map(|(index, _)| (index, None)).collect();

    while let Some((index, owner)) = stack.pop() {
        if !seen.insert(index) {
            continue;
        }
        if let Some(owner) = owner {
            found.insert(index, owner);
        }
        let Some(block) = nif.blocks.get(index) else {
            continue;
        };
        let children = block.child_refs().unwrap_or_default();
        let node = matches!(block, Block::NiLODNode(_)).then_some(index);
        for (level, child) in children.iter().enumerate() {
            let Some(child) = child.index() else { continue };
            stack.push((child, node.map(|node| (node, level)).or(owner)));
        }
    }
    found
}

/// The level of a LOD node whose range covers `distance`, falling back to the first.
///
/// The levels are not ordered by detail, so the ranges are what say which is which: the one
/// starting furthest out is the one with least in it.
pub fn lod_level_at(data: &crate::blocks::NiRangeLODData, distance: f32) -> usize {
    data.lod_levels
        .iter()
        .position(|range| distance >= range.near && distance < range.far)
        .unwrap_or(0)
}

pub struct Walk<'a> {
    /// Where every object ended up in a pass that left the look at aims alone, so the aiming
    /// pass has its targets to point at. Empty unless the file carries a look at.
    #[cfg(feature = "glam")]
    aimed_at: std::collections::HashMap<usize, crate::common::Vector3>,
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
                effects: Effects::default(),
                hidden: false,
            }],
            path: Vec::new(),
            lod: LodPolicy::All,
            #[cfg(feature = "glam")]
            time: None,
            #[cfg(feature = "glam")]
            camera: None,
            #[cfg(feature = "glam")]
            aimed_at: std::collections::HashMap::new(),
        }
    }

    pub fn from_roots(blocks: &'a [Block], roots: impl IntoIterator<Item = usize>) -> Self {
        let mut stack: Vec<Frame> = roots
            .into_iter()
            .map(|index| Frame {
                index,
                properties: Properties::default(),
                lights: Lights::default(),
                effects: Effects::default(),
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
            #[cfg(feature = "glam")]
            aimed_at: std::collections::HashMap::new(),
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
        // A look at interpolator aims one object at another, so where the other has ended up has
        // to be known before this walk reaches the node doing the aiming. A pass without them
        // supplies it, which is the same answer the engine gets: it reads the target's world
        // position from the update before and admits to being a frame behind.
        #[cfg(feature = "glam")]
        if self
            .blocks
            .iter()
            .any(|block| matches!(block, Block::NiLookAtInterpolator(_)))
        {
            let roots: Vec<usize> = self.stack.iter().map(|frame| frame.index).collect();
            self.aimed_at = Walk::from_roots(self.blocks, roots)
                .at_rough_time(time)
                .map(|visit| (visit.index, visit.transform.translation))
                .collect();
        }
        self
    }

    /// `at_time` without resolving the look at aims, which is what the pass that places their
    /// targets uses. Resolving them there would need the pass it is part of.
    #[cfg(feature = "glam")]
    fn at_rough_time(mut self, time: f32) -> Self {
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

    /// The world rotation a look at controller on this object asks for, or `None` where nothing
    /// aims it or its target cannot be placed.
    #[cfg(feature = "glam")]
    fn aim_of(
        &self,
        object: &crate::blocks::NiAvObject,
        index: usize,
        world: &NiTransform,
        time: f32,
    ) -> Option<crate::common::Matrix33> {
        if self.aimed_at.is_empty() {
            return None;
        }
        let mut next = object.controller_ref;
        for _ in 0..64 {
            let block = next.get(self.blocks)?;
            let controller = block.as_time_controller()?;
            if let Block::NiTransformController(transform) = block {
                if let Some(Block::NiLookAtInterpolator(aim)) =
                    transform.base.interpolator_ref.get(self.blocks)
                {
                    if !controller.is_active() {
                        return None;
                    }
                    let target = aim.look_at.index()?;
                    // a node aiming at itself has nothing to aim along
                    if target == index {
                        return None;
                    }
                    let at = self.aimed_at.get(&target)?;
                    let (_, _, roll) =
                        crate::anim::look_at_parts(self.blocks, aim, controller.local_time(time));
                    return crate::anim::look_at_rotation(
                        glam::Vec3::from(&world.translation),
                        glam::Vec3::from(at),
                        aim.flip(),
                        aim.axis(),
                        roll,
                    );
                }
            }
            next = controller.next_controller_ref;
        }
        None
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
                    // the aim is a world rotation already, so it replaces one rather than
                    // being composed into the parent the way the engine does it
                    #[cfg(feature = "glam")]
                    if let Some(time) = self.time {
                        if let Some(rotation) = self.aim_of(av, frame.index, &world, time) {
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
            let effect_refs = block.effect_refs().unwrap_or(&[]);
            let lights = frame.lights.with(self.blocks, effect_refs);
            let effects = frame.effects.with(self.blocks, effect_refs);

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
                            effects,
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
                effects,
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

    fn texture_effect(switch_state: bool) -> Block {
        Block::NiTextureEffect(crate::blocks::NiTextureEffect {
            base: crate::blocks::NiDynamicEffect {
                base: av_object(),
                switch_state,
                unaffected_node_refs: Vec::new(),
            },
            model_projection_matrix: crate::common::Matrix33::IDENTITY,
            model_projection_translation: crate::common::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            texture_filtering: crate::blocks::TexFilterMode::Trilerp,
            texture_clamping: crate::blocks::TexClampMode::ClampSClampT,
            texture_type: 2,
            coordinate_generation_type: 2,
            source_texture_ref: BlockRef::None,
            enable_plane: 0,
            plane: crate::common::NiPlane {
                normal: crate::common::Vector3 { x: 0.0, y: 0.0, z: 1.0 },
                constant: 0.0,
            },
        })
    }

    fn effect_on_the_shape(blocks: &[Block], at: usize) -> BlockRef {
        Walk::from_roots(blocks, [0])
            .find(|v| v.index == at)
            .expect("the shape is reachable")
            .effects
            .environment()
    }

    /// A texture effect rides the same effect list as a light and reaches down the same way, so
    /// a shape under one picks it up however deep it sits.
    #[test]
    fn a_texture_effect_reaches_the_shapes_under_it() {
        let blocks = vec![
            node(vec![1], vec![3]),
            node(vec![2], vec![]),
            shape(),
            texture_effect(true),
        ];
        assert_eq!(effect_on_the_shape(&blocks, 2), BlockRef::Index(3));
    }

    /// The engine reads one environment map and leaves the rest, so the nearest to the root wins
    /// rather than the last one gathered.
    #[test]
    fn only_the_first_environment_map_is_kept() {
        let blocks = vec![
            node(vec![1], vec![3]),
            node(vec![2], vec![4]),
            shape(),
            texture_effect(true),
            texture_effect(true),
        ];
        assert_eq!(effect_on_the_shape(&blocks, 2), BlockRef::Index(3));
    }

    /// One switched off is left out, and a light in the same list is not mistaken for one.
    #[test]
    fn a_switched_off_effect_and_a_light_are_both_passed_over() {
        let blocks = vec![
            node(vec![1], vec![2, 3]),
            shape(),
            texture_effect(false),
            directional(true, 1.0),
        ];
        assert_eq!(effect_on_the_shape(&blocks, 1), BlockRef::None);
        // and the light in that same list still comes through its own way
        assert_eq!(lights_on_the_shape(&blocks, 1), vec![BlockRef::Index(3)]);
    }

    /// A light switched off is not carried at all.
    #[test]
    fn a_light_switched_off_is_left_behind() {
        let blocks = vec![node(vec![1], vec![2]), shape(), directional(false, 1.0)];
        assert!(lights_on_the_shape(&blocks, 1).is_empty());

        // and the same light switched on does reach it, so the test is not passing vacuously
        let blocks = vec![node(vec![1], vec![2]), shape(), directional(true, 1.0)];
        assert_eq!(lights_on_the_shape(&blocks, 1), vec![BlockRef::Index(2)]);
    }

    /// A light stored at dimmer zero is still carried, because a dimmer controller drives that
    /// value and the only light in this game that animates is stored dark and driven up. Reading
    /// the stored dimmer here throws it away before the controller is ever consulted.
    #[test]
    fn a_light_dimmed_to_nothing_is_still_carried() {
        let blocks = vec![node(vec![1], vec![2]), shape(), directional(true, 0.0)];
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
