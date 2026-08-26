use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use eframe::egui;
use eframe::egui_wgpu::{self, wgpu};
use nif::glam::{Mat4, Vec3};
use nif::{
    blocks::{
        AlphaFunction, ApplyMode, Block, LightMode, NiGeometry, NiGeometryData, StencilDrawMode,
        TestFunction, TextureSlot, VertMode, ZCompareMode,
    },
    common::{BlockRef, Triangle},
    Nif,
};
use wgpu::util::DeviceExt as _;

use crate::library::{self, TextureLibrary};
use crate::shaders::{self, Shader, Shaders};
use crate::texture::decode_texture;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// Fixed when the window is created, since the colour and depth targets are built against it.
/// Every pipeline here has to agree with it or wgpu rejects the draw.
pub const MSAA_SAMPLES: u32 = 4;

/// Where the diffuse alpha sits in the model uniform, so a controller can rewrite that float
/// alone and leave the rest of the material behind it.
const ALPHA_OFFSET: u64 = 19 * 4;

/// Where the two uv transform rows sit, for the same reason.
const UV_OFFSET: u64 = 32 * 4;

/// Where the shader's own attributes sit, so a controller driving one rewrites those four floats
/// and leaves the uv rows and the material alone.
const PARAMS_OFFSET: u64 = 64 * 4;

/// The shader's `Model` and `Camera` structs, in floats. Every buffer bound as one has to be
/// this long, the grid's included.
const MODEL_FLOATS: u64 = 84;
/// The most of a file's own lights that reach the shader at once, which is the engine's own
/// limit on how many it will gather.
const SCENE_LIGHTS: usize = nif::walk::Lights::MAX;

/// Five rows each: where it is and what kind, its cone, its diffuse colour, its attenuation,
/// and its specular colour.
const LIGHT_ROWS: usize = 5;

/// View, eye, flags, the viewer's own light, then the count and the file's own lights.
const CAMERA_FLOATS: u64 = 44 + (SCENE_LIGHTS * LIGHT_ROWS * 4) as u64;

/// Every addressing a map or a shader can ask for. A sampler is built per pair.
const ADDRESS_MODES: [wgpu::AddressMode; 3] = [
    wgpu::AddressMode::Repeat,
    wgpu::AddressMode::ClampToEdge,
    wgpu::AddressMode::MirrorRepeat,
];

/// The `TexClampMode` a map carries, as an addressing pair.
fn address_of(clamp: &nif::blocks::TexClampMode) -> (wgpu::AddressMode, wgpu::AddressMode) {
    let mode = |wraps| {
        if wraps {
            wgpu::AddressMode::Repeat
        } else {
            wgpu::AddressMode::ClampToEdge
        }
    };
    (mode(clamp.wraps_u()), mode(clamp.wraps_v()))
}

/// Position, normal, colour and three uv sets. Dark reads set 1, and `VCAlphaTextureBlender`
/// reads a different set per shader map, up to set 2. The normal is zero when the geometry
/// stores none, which the shader takes as "derive one".
const VERTEX_FLOATS: usize = 3 + 3 + 4 + 2 + 2 + 2;

/// How many texture slots one shape binds. Which maps those are is the shader's choice.
const BOUND_SLOTS: usize = shaders::SLOTS;

/// The environment map rides after the shader's own slots. It carries no uv rows of its own,
/// since a reflection generates its own coordinates rather than reading a set.
const ENV_SLOT: usize = BOUND_SLOTS;

/// The slots a bind group holds: the shader's, plus the environment map.
const GROUP_SLOTS: usize = BOUND_SLOTS + 1;

/// What the fixed function path binds, in the order their uv transforms sit in the uniform.
const DEFAULT_SLOTS: [Option<shaders::Source>; BOUND_SLOTS] = [
    Some(shaders::Source::Slot(TextureSlot::Base)),
    Some(shaders::Source::Slot(TextureSlot::Dark)),
    Some(shaders::Source::Slot(TextureSlot::Glow)),
    None,
];

/// The one light the viewer invents, since a NIF does not carry the scene's lighting. Almost no
/// file that names a custom shader contains a light block, so a faithful reading of the file
/// would leave nearly everything black.
///
/// Every path reads this: the fixed function stand in and each custom shader, so moving it moves
/// the whole scene consistently. The defaults reproduce the shading nifty had when these were
/// constants baked into the fragment shader.
#[derive(Clone, Copy, PartialEq)]
pub struct Light {
    /// The way the light travels, matching the engine's own convention, so a shader that wants
    /// the direction back to the light negates it.
    pub direction: Vec3,
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    /// A view dependent rim that is nifty's own viewing aid rather than anything the engine had.
    pub fill: f32,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            // NIF is Z up, so a key light from above means +Z. Stored as the direction of
            // travel, which is away from the eye and downward.
            direction: -Vec3::new(0.3, 0.45, 0.85).normalize(),
            ambient: Vec3::splat(0.2),
            diffuse: Vec3::splat(0.65),
            specular: Vec3::splat(1.0),
            fill: 0.3,
        }
    }
}

impl Light {
    /// The camera uniform's trailing 16 floats.
    fn uniform(&self) -> [f32; 16] {
        let d = self.direction.normalize_or_zero();
        [
            d.x,
            d.y,
            d.z,
            self.fill,
            self.ambient.x,
            self.ambient.y,
            self.ambient.z,
            0.0,
            self.diffuse.x,
            self.diffuse.y,
            self.diffuse.z,
            0.0,
            self.specular.x,
            self.specular.y,
            self.specular.z,
            0.0,
        ]
    }
}

/// Fills the camera uniform, so its layout lives in one place rather than at the call site.
pub fn camera_uniform(
    view_proj: Mat4,
    eye: Vec3,
    colors: bool,
    textures: bool,
    light: &Light,
    lights: &[nif::light::Lit],
    origin: Vec3,
) -> [f32; CAMERA_FLOATS as usize] {
    let mut out = [0.0; CAMERA_FLOATS as usize];
    out[..16].copy_from_slice(&view_proj.to_cols_array());
    out[16..19].copy_from_slice(&eye.to_array());
    out[20] = f32::from(colors);
    out[21] = f32::from(textures);
    out[24..40].copy_from_slice(&light.uniform());
    // a file's own lights replace the viewer's, and where it has none the viewer's stands in,
    // so the shader can tell the two apart by the count alone
    out[40] = lights.len() as f32;
    for (at, lit) in lights.iter().take(SCENE_LIGHTS).enumerate() {
        let row = 44 + at * LIGHT_ROWS * 4;
        let kind = match lit.falloff {
            nif::light::Falloff::Directional => 0.0,
            nif::light::Falloff::Point => 1.0,
            nif::light::Falloff::Spot => 2.0,
            // an ambient light is folded into the ambient term rather than reaching this
            nif::light::Falloff::Ambient => continue,
        };
        // the shader works in the space the scene is drawn in, which is moved to sit near zero,
        // so a light's position has to be moved with it. A direction is unaffected by a move.
        let at = match lit.falloff {
            nif::light::Falloff::Directional => lit.direction,
            _ => lit.position - origin,
        };
        out[row..row + 4].copy_from_slice(&[at.x, at.y, at.z, kind]);
        out[row + 4..row + 8].copy_from_slice(&[
            lit.direction.x,
            lit.direction.y,
            lit.direction.z,
            lit.cos_cutoff,
        ]);
        out[row + 8..row + 12].copy_from_slice(&[
            lit.diffuse.x,
            lit.diffuse.y,
            lit.diffuse.z,
            lit.exponent,
        ]);
        out[row + 12..row + 16].copy_from_slice(&[
            lit.attenuation.x,
            lit.attenuation.y,
            lit.attenuation.z,
            0.0,
        ]);
        out[row + 16..row + 20].copy_from_slice(&[
            lit.specular.x,
            lit.specular.y,
            lit.specular.z,
            0.0,
        ]);
    }
    out
}

/// The slots a flip controller is ever seen to drive, in the order `FlipState` holds them.
pub const FLIPPABLE: [TextureSlot; 3] = [TextureSlot::Base, TextureSlot::Glow, TextureSlot::Gloss];

/// Which source each flippable slot is showing, for one texturing property at one time.
///
/// A property is commonly flipped on two slots at once, so what a shape binds depends on the
/// combination rather than on any one slot's frame. This is the cache key for that, and it is
/// deliberately not a set of independent lookups: the bindings are one group.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct FlipState {
    sources: [Option<usize>; FLIPPABLE.len()],
}

impl FlipState {
    pub fn set(&mut self, slot: TextureSlot, source: usize) {
        if let Some(at) = FLIPPABLE.iter().position(|s| *s == slot) {
            self.sources[at] = Some(source);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sources.iter().all(Option::is_none)
    }

    fn source(&self, slot: TextureSlot) -> Option<usize> {
        let at = FLIPPABLE.iter().position(|s| *s == slot)?;
        self.sources[at]
    }
}

/// Lives in `callback_resources`, which is all `paint` can reach.
pub struct Preview {
    wire: wgpu::RenderPipeline,
    highlight: wgpu::RenderPipeline,
    grid: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,
    /// What the texture group is laid out as, so a flipped combination can be assembled here
    /// rather than only where the scene is built.
    texture_layout: wgpu::BindGroupLayout,
    /// Bind groups for combinations the animation has actually reached, by shape, pass and
    /// combination. Built on demand: the cross product reaches 2,601 for one property in this
    /// corpus against 102 textures, so building it up front is not an option.
    flipped: HashMap<(usize, usize, FlipState), wgpu::BindGroup>,
}

/// The texture group's own layout, in one place because it is built both when a scene is made
/// and later when a flip controller reaches a combination nothing has bound yet.
fn slot_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    slots: [(&wgpu::TextureView, &wgpu::Sampler); GROUP_SLOTS],
    cube: (&wgpu::TextureView, &wgpu::Sampler),
) -> wgpu::BindGroup {
    let mut entries: Vec<wgpu::BindGroupEntry> = slots
        .iter()
        .enumerate()
        .flat_map(|(i, (view, sampler))| {
            [
                wgpu::BindGroupEntry {
                    binding: (i * 2) as u32,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: (i * 2 + 1) as u32,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ]
        })
        .collect();
    entries.push(wgpu::BindGroupEntry {
        binding: (GROUP_SLOTS * 2) as u32,
        resource: wgpu::BindingResource::TextureView(cube.0),
    });
    entries.push(wgpu::BindGroupEntry {
        binding: (GROUP_SLOTS * 2 + 1) as u32,
        resource: wgpu::BindingResource::Sampler(cube.1),
    });
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("nifty textures"),
        layout,
        entries: &entries,
    })
}

/// Lives on the app, for building meshes when a file loads.
pub struct Gfx {
    pub render_state: egui_wgpu::RenderState,
    model_layout: wgpu::BindGroupLayout,
    texture_layout: wgpu::BindGroupLayout,
    samplers: [wgpu::Sampler; ADDRESS_MODES.len() * ADDRESS_MODES.len() * 2],
    pipeline_layout: wgpu::PipelineLayout,
    pub camera_buffer: wgpu::Buffer,
}

pub struct Mesh {
    pub shape_block: usize,
    pub data_block: usize,
    pub center: Vec3,
    /// The same centre before the shape's transform, so an animated or billboarded pose can be
    /// applied to it at draw time.
    local_center: Vec3,
    /// The NiMaterialProperty this shape draws with, which is what an alpha controller targets.
    material_block: Option<usize>,
    /// The NiTexturingProperty this shape draws with, which is what a texture transform
    /// controller targets.
    pub texturing_block: Option<usize>,
    /// The attributes this shape's shader declares and what they resolved to when the scene was
    /// built, so a controller driving one only has to replace that lane.
    pub attributes: ([&'static str; 4], [f32; 4]),
    /// The vertex buffer's contents as built, kept only for a shape whose geometry is rewritten
    /// per frame. Positions and normals are replaced and everything interleaved with them is
    /// left alone, so the rest has to still be here to write back.
    deform_source: Option<Vec<f32>>,
    /// Whether the bones place this shape rather than its own transform. A skinned shape's
    /// vertices reach the buffer already in world space, so its model matrix carries only the
    /// move that puts the scene near zero.
    pub skinned: bool,
    /// The uv set each binding reads where its shader pins one, parallel to `bound`.
    pub uv_pins: [Option<u32>; BOUND_SLOTS],
    /// Which texture slots this shape's bindings hold, which its shader decides. The uv rows
    /// in the model uniform belong to these, so where a shader draws more than one pass, they
    /// are the bindings of the first pass that samples anything.
    pub bound: [Option<shaders::Source>; BOUND_SLOTS],
    pub radius: f32,
    /// The NiLODNode this shape sits under, and which of its levels, if any.
    lod: Option<(usize, usize)>,
    /// Whether this goes in the back to front pass. A shape that blends but whose alpha property
    /// asks for no sorter is drawn where the traversal reaches it instead, among the opaque ones.
    sorted: bool,
    /// What this shape draws, in order. One entry unless its shader outlines.
    passes: Vec<MeshPass>,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    count: u32,
    edges: wgpu::Buffer,
    edge_count: u32,
    bind_group: wgpu::BindGroup,
    /// Rewritten when an animated pose moves the shape. The model matrix is its first 16 floats.
    /// One per shape rather than one per pass: every pass reads the same pose, material and
    /// attributes, and the uv rows in it are the ones `bound` names.
    model_buffer: wgpu::Buffer,
}

/// One draw a shape makes, which is one pass of its shader.
struct MeshPass {
    pipeline: wgpu::RenderPipeline,
    pipeline_unculled: wgpu::RenderPipeline,
    texture: wgpu::BindGroup,
    /// What this pass binds when nothing is flipping, kept so a flipped combination can be
    /// assembled later without rebuilding the shape.
    bindings: PassBindings,
}

/// Everything a pass's texture group is built from, retained so the group can be built again
/// with one or more slots swapped for a flip controller's current frame.
struct PassBindings {
    views: [wgpu::TextureView; GROUP_SLOTS],
    samplers: [wgpu::Sampler; GROUP_SLOTS],
    cube: (wgpu::TextureView, wgpu::Sampler),
    /// Which slot each binding position reads, so a flipped source lands in the right one.
    slots: [Option<TextureSlot>; BOUND_SLOTS],
    /// Every frame any flip controller on this property can reach, uploaded once each. Empty
    /// unless something flips this shape.
    frames: HashMap<usize, wgpu::TextureView>,
}

impl PassBindings {
    /// This pass's group with each flipped slot swapped for the frame it is showing. Falls back
    /// to whatever the shape carries wherever a frame is missing.
    fn group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        state: FlipState,
    ) -> wgpu::BindGroup {
        let views: [&wgpu::TextureView; GROUP_SLOTS] = std::array::from_fn(|position| {
            self.slots
                .get(position)
                .copied()
                .flatten()
                .and_then(|slot| state.source(slot))
                .and_then(|source| self.frames.get(&source))
                .unwrap_or(&self.views[position])
        });
        slot_bind_group(
            device,
            layout,
            std::array::from_fn(|i| (views[i], &self.samplers[i])),
            (&self.cube.0, &self.cube.1),
        )
    }
}

/// What a particle system's own transform contributes to its particles.
///
/// A system flagged world space keeps only its **scale**: its particles are already in world
/// space, so where the node sits and which way it faces are not theirs, and a node that moves
/// leaves the particles it has already emitted behind. A system not so flagged carries them with
/// it like any other child.
///
/// Every reader goes through here, the emitter placement included, because the space a particle
/// is born into and the space it is drawn in have to be the same one.
pub fn particle_space(pose: Mat4, world_space: bool) -> Mat4 {
    match world_space {
        true => Mat4::from_scale(Vec3::splat(pose.x_axis.truncate().length())),
        false => pose,
    }
}

/// A particle system's drawing side. The geometry is generated per frame rather than stored, so
/// the buffers are sized once for the system's capacity and rewritten as the simulation moves.
pub struct ParticleMesh {
    /// The NiParticleSystem this draws, which is what the frame's particles are keyed by.
    pub block: usize,
    /// Whether the particles are already in world space, so the system's own place and turn are
    /// not applied to them. See `particle_space`.
    pub world_space: bool,
    /// Where the system sits when nothing animates it. The frame's pose wins when there is one,
    /// because a system whose node moves has to be drawn where picking will look for it.
    pub model: Mat4,
    pub capacity: usize,
    /// How far a particle can get from the system before it dies, in world units. The system
    /// stores no geometry, so without this it contributes nothing to the scene bounds and the
    /// camera frames nothing.
    pub reach: f32,
    pipeline: wgpu::RenderPipeline,
    texture: wgpu::BindGroup,
    bind_group: wgpu::BindGroup,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    /// The quad outlines, so a particle shows in wireframe like any other geometry.
    edges: wgpu::Buffer,
    /// Whether this goes in the back to front pass, on the same terms as a shape.
    sorted: bool,
    /// The NiLODNode this system sits under, and which of its levels, if any. A system carries
    /// no stored geometry, so a path keyed on that skips it.
    lod: Option<(usize, usize)>,
}

/// Something drawn in the back to front pass, which sorts across both kinds.
enum Sorted<'a> {
    Shape(&'a Mesh),
    /// The system and how many of its quads are alive this frame.
    Particles(&'a ParticleMesh, usize),
}

pub struct Scene {
    /// Where the scene is drawn from. Everything uploaded to the GPU is relative to this, and
    /// everything the CPU reasons about, bounds, picking, LOD distances, stays in the file's own
    /// world space.
    ///
    /// A NIF can sit far enough from the world origin that f32 quantisation there is coarser
    /// than the gap between two coincident surfaces, so composing `model * position` at full
    /// world magnitude made them fight for the same depth. Composing near zero keeps the
    /// shape's own geometry exact.
    pub origin: Vec3,
    /// The camera the file carries, if it carries one. Never more than one in this game.
    pub camera: Option<SceneCamera>,
    /// The lights the file itself carries, in the order the walk reaches them. Empty for nearly
    /// every file, and where it is empty the viewer's own light stands in.
    ///
    /// Resolved at rest. A frame resolves them again from `light_blocks` where anything moves
    /// one or drives its dimmer.
    pub lights: Vec<nif::light::Lit>,
    /// Which block each of `lights` came from, in the same order, so a frame can resolve them
    /// again without having to work out the order a second time.
    pub light_blocks: Vec<usize>,
    /// Every ambient light in the file, summed. `None` leaves the viewer's own ambient alone.
    pub ambient: Option<Vec3>,
    pub meshes: Vec<Mesh>,
    pub particles: Vec<ParticleMesh>,
    pub center: Vec3,
    /// The bounds at rest, which is what the camera frames and what LOD measures against. An
    /// animation is not included: framing a file whose animation flings something a hundred
    /// times its own size away would leave the thing you came to look at a speck.
    pub radius: f32,
    /// The furthest anything gets from `center` at any point in the animation, never less than
    /// `radius`. Only the far plane reads this, so nothing clips out mid animation without the
    /// framing paying for it.
    ///
    /// Swept rather than guessed at: an animation can carry geometry far enough outside the
    /// resting bounds that no fixed headroom covers it.
    pub animated_radius: f32,
    pub lods: HashMap<usize, Lod>,
    pub grid: Grid,
}

/// The ground plane and axes. Sized to the file when the scene is built, since one spacing
/// cannot serve both the smallest and the largest scenes.
pub struct Grid {
    vertices: wgpu::Buffer,
    count: u32,
    model: wgpu::BindGroup,
    texture: wgpu::BindGroup,
    pub spacing: f32,
    /// Where the floor is centred, which is under the scene rather than at the world origin. A
    /// model far from the origin used to drag the grid all the way back to it, and the far plane
    /// had to clear that, which cost the depth precision coincident surfaces need.
    pub center: Vec3,
    /// How far the floor reaches from its own centre. The far plane has to clear it, or the grid is
    /// cut off rather than merely small when the camera closes in on something.
    pub half: f32,
}

/// A NiLODNode's switching distances, and the point they are measured from.
pub struct Lod {
    pub center: Vec3,
    /// The same point before the node's transform, so an animated node measures from where it
    /// now is rather than from where the file left it. A node that moves would otherwise switch
    /// level at the wrong distances, which is the same drift the particle transform had.
    pub local_center: Vec3,
    pub ranges: Vec<(f32, f32)>,
}

/// How the scene is being looked at, which decides the transforms a walk composes: where the
/// timeline sits, and where the camera is for billboards to turn towards.
///
/// Drawing and picking both walk through this, so what you can click cannot drift from what is
/// on screen.
#[derive(Clone, Copy, Default)]
pub struct Viewpoint {
    pub time: Option<f32>,
    pub camera: Option<nif::billboard::Camera>,
}

impl Viewpoint {
    pub fn walk<'a>(&self, nif: &'a Nif) -> nif::walk::Walk<'a> {
        let mut walk = nif.walk();
        if let Some(time) = self.time {
            walk = walk.at_time(time);
        }
        if let Some(camera) = self.camera {
            walk = walk.seen_from(camera);
        }
        walk
    }

    /// Nothing moves, so the transforms the scene was built with still stand.
    pub fn is_static(&self) -> bool {
        self.time.is_none() && self.camera.is_none()
    }
}

/// What the walk found for the frame being drawn: where shapes have moved, and which are culled.
/// Empty when the file has nothing that moves or hides.
#[derive(Default)]
pub struct Frame {
    pub poses: HashMap<usize, Mat4>,
    pub hidden: HashSet<usize>,
    /// Material alpha a controller has replaced, by the material's own block.
    pub alpha: HashMap<usize, f32>,
    /// Every bound slot's uv transform where a controller drives one, by shape block. All
    /// three ride together, since one controller per member means several can target one
    /// property at once. Keyed by shape rather than property because which slots a shape binds
    /// is its shader's choice, so two shapes sharing a property can want different rows.
    pub uv: HashMap<usize, [f32; BOUND_SLOTS * 8]>,
    /// Where a shape whose vertices the file does not fix have moved to, by shape block. A
    /// geometry morpher and a skin are the two things that rewrite stored geometry, and no
    /// shape in this game does both, so one map holds either.
    ///
    /// The renderer and the picker both read this. They have to read the same value: a shape
    /// tested against the vertices the file stores is clickable where it rested rather than
    /// where it has been carried to.
    pub deformed: HashMap<usize, Deformed>,
    /// A shape's shader attributes where a controller drives one of them, by shape block. Empty
    /// unless a file animates an attribute, which is rare and was easy to miss.
    pub params: HashMap<usize, [f32; 4]>,
    /// What each texturing property is flipping to this frame, across every slot a controller
    /// drives. A property flipped on two slots at once is the common case, not the exception.
    pub flip: HashMap<usize, FlipState>,
    /// Where each particle system's particles are, by the system's own block. Simulated by the
    /// caller, since the state has to outlive a scene rebuild.
    pub particles: HashMap<usize, Vec<nif::psys::Particle>>,
}

impl Frame {
    /// Where a shape sits this frame, which is not where the scene was built with it: a pose
    /// moves it, a billboard turns it and a skin replaces its vertices outright.
    ///
    /// Everything that needs to point at a shape reads this. Framing it from the resting centre
    /// instead sends the camera to where it was rather than where it is.
    pub fn center_of(&self, mesh: &Mesh) -> Vec3 {
        if mesh.skinned {
            return self.deformed_center(mesh.shape_block, mesh.local_center);
        }
        match self.poses.get(&mesh.shape_block) {
            Some(pose) => pose.transform_point3(mesh.local_center),
            None => mesh.center,
        }
    }

    /// Where a deformed shape's geometry sits this frame, for a shape whose own node pose does
    /// not place it. Its resting centre stands until something has moved it.
    pub fn deformed_center(&self, at: usize, resting: Vec3) -> Vec3 {
        let Some(deformed) = self.deformed.get(&at) else {
            return resting;
        };
        let (low, high) = deformed.positions.iter().fold(
            (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN)),
            |(low, high), v| (low.min(Vec3::from(v)), high.max(Vec3::from(v))),
        );
        match low.x <= high.x {
            true => (low + high) * 0.5,
            false => resting,
        }
    }
}

/// A shape's geometry for one frame, where the file's own vertices are not where it draws.
/// The normals travel with the positions because the two are rebuilt together or not at all,
/// and keeping them apart is how they would come to disagree.
///
/// A morph leaves its positions in the shape's own space, so the shape still draws with its
/// model transform. A skin puts them in world space and the shape draws with none, which is
/// what `Mesh::skinned` decides.
pub struct Deformed {
    pub positions: Vec<nif::common::Vector3>,
    /// Rebuilt normals. `None` leaves the shape shaded as it rests, which is what the engine
    /// does for a morph that does not ask and for a shape storing none.
    pub normals: Option<Vec<nif::common::Vector3>>,
}

/// The camera a file carries of its own, as the viewer needs it. A file holds at most one.
///
/// The frustum is symmetric in every corpus file, so the vertical angle and the shape of the
/// rectangle are all it takes to rebuild the projection the game used.
#[derive(Debug, Clone, Copy)]
pub struct SceneCamera {
    /// The block, so a frame can find where it has moved to.
    pub block: usize,
    /// Vertical field of view, in radians.
    pub fov: f32,
    /// Width over height, which is what the preview is letterboxed to.
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl SceneCamera {
    /// Where it sits and which way it points, from its world transform. The engine takes a
    /// camera's basis from the columns of its rotation: the first is where it looks, the second
    /// is its up, and the third its right.
    pub fn view(&self, world: Mat4) -> (Vec3, Vec3, Vec3) {
        (
            world.w_axis.truncate(),
            world.x_axis.truncate().normalize_or_zero(),
            world.y_axis.truncate().normalize_or_zero(),
        )
    }
}

/// Which level of each LOD node to draw.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LodMode {
    /// Every level at once, which is how overlapping levels become visible.
    All,
    /// The level the game would pick for the current camera distance.
    Auto,
    /// The level the game would pick at a distance the user chooses.
    Manual,
}

impl Scene {
    /// Whether a mesh's LOD level is the one being shown.
    pub fn shows(
        &self,
        lod: Option<(usize, usize)>,
        mode: LodMode,
        distance: f32,
        eye: Vec3,
        poses: &HashMap<usize, Mat4>,
    ) -> bool {
        let Some((node, level)) = lod else {
            return true;
        };
        let Some(lod) = self.lods.get(&node) else {
            return true;
        };
        match mode {
            LodMode::All => true,
            LodMode::Auto => level == lod.level_at(lod.center_at(poses, node).distance(eye)),
            LodMode::Manual => level == lod.level_at(distance),
        }
    }

    /// The shape blocks currently drawn, which is what picking may select. The poses are the
    /// frame's, so a LOD node that animates is measured from where it now is and picking agrees
    /// with what is on screen.
    pub fn visible_shapes(
        &self,
        mode: LodMode,
        distance: f32,
        eye: Vec3,
        poses: &HashMap<usize, Mat4>,
    ) -> HashSet<usize> {
        self.meshes
            .iter()
            .filter(|mesh| self.shows(mesh.lod, mode, distance, eye, poses))
            .map(|mesh| mesh.shape_block)
            // a particle system is drawn and picked like anything else, so a hidden level of one
            // must not be selectable either
            .chain(
                self.particles
                    .iter()
                    .filter(|mesh| self.shows(mesh.lod, mode, distance, eye, poses))
                    .map(|mesh| mesh.block),
            )
            .collect()
    }
}

impl Lod {
    /// Where the switching distances are measured from as of this frame. The pose is the node's
    /// own, so a LOD node under an animated parent follows it.
    pub fn center_at(&self, poses: &HashMap<usize, Mat4>, node: usize) -> Vec3 {
        match poses.get(&node) {
            Some(pose) => pose.transform_point3(self.local_center),
            None => self.center,
        }
    }

    /// The level whose range covers `distance`, falling back to the first, which is what
    /// `nif::walk`'s distance policy does.
    pub fn level_at(&self, distance: f32) -> usize {
        self.ranges
            .iter()
            .position(|(near, far)| distance >= *near && distance < *far)
            .unwrap_or(0)
    }

    /// The distance past which the least detailed level is already chosen, so nothing changes
    /// beyond it. The outermost range's far is effectively unbounded, so this is its near.
    pub fn last_switch(&self) -> f32 {
        self.ranges
            .iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map_or(0.0, |(near, _)| *near)
    }
}

/// The render state a shape's properties ask for. Pipelines are cached on this, so only the
/// combinations a file actually uses get built.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DrawState {
    cull: Option<wgpu::Face>,
    depth_write: bool,
    /// Testing disabled is `Always`, so this one field covers both flags.
    depth: wgpu::CompareFunction,
    blend: Option<(wgpu::BlendFactor, wgpu::BlendFactor)>,
}

impl DrawState {
    fn opaque() -> Self {
        Self {
            cull: Some(wgpu::Face::Back),
            depth_write: true,
            depth: wgpu::CompareFunction::LessEqual,
            blend: None,
        }
    }
}

/// The colour attribute a shader declares, as the shape supplies it. Same rule as a float: the
/// name does the binding, an `NiColorExtraData` on the shape wins, and the shader's declared
/// colour is what a shape carrying none falls back to. A colour takes a whole vec4 where a float
/// takes one lane of `params`, which is why it is its own field rather than four more names.
pub fn shader_color(blocks: &[Block], extra_data_refs: &[BlockRef], shader: &Shader) -> [f32; 4] {
    if shader.color_name.is_empty() {
        return shader.color;
    }
    let supplied = extra_data_refs
        .iter()
        .filter_map(|reference| reference.get(blocks))
        .find_map(|block| match block {
            Block::NiColorExtraData(color)
                if color.name.as_bytes() == shader.color_name.as_bytes() =>
            {
                Some([color.data.r, color.data.g, color.data.b, color.data.a])
            }
            _ => None,
        });
    supplied.unwrap_or(shader.color)
}

/// Which texture each of a pass's bindings actually reads, for one shape. A shader declares a
/// texture attribute with a file name, and a shape redirects it to one of its own shader maps by
/// carrying an integer extra data named after it. So the map in the file wins, exactly as a float
/// attribute does, and the declared file is what a shape that names none falls back to.
pub fn shader_slots(
    blocks: &[Block],
    extra_data_refs: &[BlockRef],
    slots: [Option<shaders::Source>; BOUND_SLOTS],
) -> [Option<shaders::Source>; BOUND_SLOTS] {
    slots.map(|slot| match slot {
        Some(shaders::Source::IndexedSlot { index, slot }) => Some(shaders::Source::Slot(
            named_map(blocks, extra_data_refs, index)
                .map(TextureSlot::Shader)
                .unwrap_or(slot),
        )),
        Some(shaders::Source::Attribute { index, file }) => {
            match named_map(blocks, extra_data_refs, index) {
                Some(map) => Some(shaders::Source::Slot(TextureSlot::Shader(map))),
                None => Some(shaders::Source::Named(file)),
            }
        }
        other => other,
    })
}

/// The shader map a shape points a texture attribute at, by the integer extra data named after
/// it. None where the shape names none and the technique's own choice stands.
fn named_map(blocks: &[Block], extra_data_refs: &[BlockRef], index: &str) -> Option<u32> {
    extra_data_refs
        .iter()
        .filter_map(|reference| reference.get(blocks))
        .find_map(|block| match block {
            Block::NiIntegerExtraData(integer) if integer.name.as_bytes() == index.as_bytes() => {
                Some(integer.value)
            }
            _ => None,
        })
}

/// One transform as the GPU is given it, moved so the scene sits near zero. The CPU keeps the
/// original: bounds, picking and LOD distances all stay in the file's own space, and only what
/// is composed per vertex moves.
fn drawn_at(origin: Vec3, model: Mat4) -> Mat4 {
    Mat4::from_translation(-origin) * model
}

/// Every corner of a box. A rotation can put any of the eight furthest out, so taking the two
/// extremes alone understates the reach of anything turned.
fn box_corners(low: Vec3, high: Vec3) -> [Vec3; 8] {
    std::array::from_fn(|corner| {
        Vec3::new(
            if corner & 1 == 0 { low.x } else { high.x },
            if corner & 2 == 0 { low.y } else { high.y },
            if corner & 4 == 0 { low.z } else { high.z },
        )
    })
}

/// The local box a morphing shape reaches over the whole animation, or `None` where nothing
/// morphs it. Its own resting box is not it: a morph replaces the stored vertices rather than
/// nudging them, so a shape can end the span nowhere near where it began.
///
/// Sampled, like the transform sweep, and for the same reason: a weight track can carry a
/// target anywhere between its keys. More steps than that sweep takes, because this runs once
/// per shape when the scene is built rather than once per walk.
fn morph_reach(nif: &Nif, geometry: &NiGeometry, span: Option<(f32, f32)>) -> Option<(Vec3, Vec3)> {
    const STEPS: u32 = 24;
    let (start, end) = span.unwrap_or((0.0, 0.0));

    let mut low = Vec3::splat(f32::MAX);
    let mut high = Vec3::splat(f32::MIN);
    let mut reached = false;
    for step in 0..=STEPS {
        let time = start + (end - start) * step as f32 / STEPS as f32;
        let moved = nif::anim::morph_at(&nif.blocks, geometry, time)?;
        for at in &moved {
            low = low.min(Vec3::from(at));
            high = high.max(Vec3::from(at));
            reached = true;
        }
    }
    reached.then_some((low, high))
}

/// The world box a skinned shape reaches over the whole animation, or `None` where nothing
/// skins it. Already in world space, since that is where its bones put it.
///
/// The transform sweep cannot stand in for this. A skinned shape's own node is usually still
/// while its bones move, so sweeping the node finds nothing at all.
fn skin_reach(nif: &Nif, geometry: &NiGeometry, span: Option<(f32, f32)>) -> Option<(Vec3, Vec3)> {
    const STEPS: u32 = 12;
    let (start, end) = span.unwrap_or((0.0, 0.0));

    let mut low = Vec3::splat(f32::MAX);
    let mut high = Vec3::splat(f32::MIN);
    let mut reached = false;
    for step in 0..=STEPS {
        let time = start + (end - start) * step as f32 / STEPS as f32;
        let pose: HashMap<usize, Mat4> = nif
            .walk()
            .at_time(time)
            .map(|visit| (visit.index, Mat4::from(&visit.transform)))
            .collect();
        let skinned = nif::skin::deform(&nif.blocks, geometry, |index| pose.get(&index).copied())?;
        for at in &skinned.positions {
            low = low.min(Vec3::from(at));
            high = high.max(Vec3::from(at));
            reached = true;
        }
    }
    reached.then_some((low, high))
}

/// How far anything gets from the resting centre over the whole animation. The far plane reads
/// this so a shape carried outside the resting bounds does not clip out part way through.
///
/// Sampled rather than solved: a transform track can move a shape any way at all between its
/// keys, and sampling costs one transform walk per step with no geometry touched.
fn swept_radius(
    nif: &Nif,
    center: Vec3,
    resting: f32,
    boxes: &HashMap<usize, (Vec3, Vec3)>,
) -> f32 {
    const STEPS: u32 = 12;
    let Some((start, end)) = nif::anim::span(&nif.blocks) else {
        return resting;
    };
    if end <= start || boxes.is_empty() {
        return resting;
    }

    let mut furthest = resting;
    for step in 0..=STEPS {
        let time = start + (end - start) * step as f32 / STEPS as f32;
        for visit in nif.walk().at_time(time) {
            let Some((low, high)) = boxes.get(&visit.index) else {
                continue;
            };
            let model = Mat4::from(&visit.transform);
            for at in box_corners(*low, *high) {
                furthest = furthest.max(center.distance(model.transform_point3(at)));
            }
        }
    }
    furthest
}

/// Whether a shape joins the back to front pass. The engine queues one only when it blends and
/// its alpha property does not ask to be left out, and draws everything else where the traversal
/// reaches it. So an unsorted blended shape still blends, but lands among the opaque geometry in
/// file order rather than after all of it.
///
/// `blends` is what the shape ends up drawing with, a shader's override included, while the hint
/// is read from the file's own property. The two are separate concerns: a shader forcing blending
/// on says nothing about how the result should be ordered.
pub fn sorts(blends: bool, alpha: Option<&nif::blocks::NiAlphaProperty>) -> bool {
    blends && !alpha.is_some_and(|a| a.no_sorter())
}

/// A z buffer property's own comparison. `LessEqual` is also the default a shape without one
/// takes, which matters where coincident geometry is drawn twice: `Less` would drop the second
/// draw instead of letting it blend over the first.
fn depth_of(zbuffer: Option<&nif::blocks::NiZBufferProperty>) -> wgpu::CompareFunction {
    let Some(zbuffer) = zbuffer else {
        return wgpu::CompareFunction::LessEqual;
    };
    if !zbuffer.depth_test() {
        return wgpu::CompareFunction::Always;
    }
    match zbuffer.function {
        ZCompareMode::ZCompAlways => wgpu::CompareFunction::Always,
        ZCompareMode::ZCompLess => wgpu::CompareFunction::Less,
        ZCompareMode::ZCompEqual => wgpu::CompareFunction::Equal,
        ZCompareMode::ZCompLessEqual => wgpu::CompareFunction::LessEqual,
        ZCompareMode::ZCompGreater => wgpu::CompareFunction::Greater,
        ZCompareMode::ZCompNotEqual => wgpu::CompareFunction::NotEqual,
        ZCompareMode::ZCompGreaterEqual => wgpu::CompareFunction::GreaterEqual,
        ZCompareMode::ZCompNever => wgpu::CompareFunction::Never,
        ZCompareMode::Unknown(_) => wgpu::CompareFunction::LessEqual,
    }
}

/// NiStencilProperty::draw_mode maps straight to D3DRS_CULLMODE: Ccw and CcwOrBoth cull
/// clockwise faces, Cw culls counter-clockwise ones, Both culls nothing.
pub(crate) fn cull_of(draw_mode: Option<&StencilDrawMode>) -> Option<wgpu::Face> {
    match draw_mode {
        Some(StencilDrawMode::Both) => None,
        Some(StencilDrawMode::Cw) => Some(wgpu::Face::Front),
        _ => Some(wgpu::Face::Back),
    }
}

fn blend_factor(function: &AlphaFunction, destination: bool) -> wgpu::BlendFactor {
    match function {
        AlphaFunction::One => wgpu::BlendFactor::One,
        AlphaFunction::Zero => wgpu::BlendFactor::Zero,
        AlphaFunction::SrcColor => wgpu::BlendFactor::Src,
        AlphaFunction::InvSrcColor => wgpu::BlendFactor::OneMinusSrc,
        AlphaFunction::DestColor => wgpu::BlendFactor::Dst,
        AlphaFunction::InvDestColor => wgpu::BlendFactor::OneMinusDst,
        AlphaFunction::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
        AlphaFunction::InvSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
        AlphaFunction::DestAlpha => wgpu::BlendFactor::DstAlpha,
        AlphaFunction::InvDestAlpha => wgpu::BlendFactor::OneMinusDstAlpha,
        // wgpu rejects a saturating destination factor
        AlphaFunction::SrcAlphaSaturate if destination => wgpu::BlendFactor::One,
        AlphaFunction::SrcAlphaSaturate => wgpu::BlendFactor::SrcAlphaSaturated,
    }
}

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    /// Absolute, so zooming does not inherit the scale of a sprawling scene.
    /// `None` frames the whole scene.
    pub distance: Option<f32>,
    pub pan: Vec3,
}

impl Camera {
    /// How far the orbit rig tilts, short of straight up and straight down where the view
    /// direction and the world up would agree and the basis would collapse.
    pub const PITCH_LIMIT: f32 = 1.5;
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            yaw: 0.7,
            pitch: 0.5,
            distance: None,
            pan: Vec3::ZERO,
        }
    }
}

impl Gfx {
    pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let device = &render_state.device;

        let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("nifty camera"),
            entries: &[uniform_entry(CAMERA_FLOATS)],
        });
        let model_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("nifty model"),
            entries: &[uniform_entry(MODEL_FLOATS)],
        });
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("nifty texture"),
            entries: &slot_layout_entries(),
        });
        // one per address mode pair. TexClampMode is per map and glow maps clamp about as
        // often as they wrap, and a shader can pin its own: ActionSpecularBand mirrors in u.
        let samplers = std::array::from_fn(|i| {
            let modes = ADDRESS_MODES.len();
            // a toon ramp asks for point sampling, which is what gives it hard bands
            let filter = if i / (modes * modes) == 0 {
                wgpu::FilterMode::Linear
            } else {
                wgpu::FilterMode::Nearest
            };
            device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("nifty sampler"),
                address_mode_u: ADDRESS_MODES[(i / modes) % modes],
                address_mode_v: ADDRESS_MODES[i % modes],
                mag_filter: filter,
                min_filter: filter,
                ..Default::default()
            })
        });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("nifty camera"),
            size: CAMERA_FLOATS * 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("nifty camera"),
            layout: &camera_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // the wire, highlight and grid passes are the contract's own entry points, so they
        // come from the fixed function module rather than from whichever shader a shape names
        let fixed = Shaders::default();
        let fixed = fixed.fixed();
        let shader = compile(device, &fixed.name, &fixed.passes[0])
            .expect("the built in fixed function shader has to compile");
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("nifty"),
            bind_group_layouts: &[
                Some(&camera_layout),
                Some(&model_layout),
                Some(&texture_layout),
            ],
            immediate_size: 0,
        });
        let highlight = build_pipeline(
            device,
            &layout,
            &shader,
            render_state.target_format,
            DrawState {
                cull: None,
                depth_write: false,
                depth: wgpu::CompareFunction::Always,
                blend: None,
            },
            wgpu::PrimitiveTopology::LineList,
            "fs_highlight",
        );
        // depth tested and depth writing, so geometry hides the part of the floor behind it
        let grid = build_pipeline(
            device,
            &layout,
            &shader,
            render_state.target_format,
            DrawState {
                cull: None,
                ..DrawState::opaque()
            },
            wgpu::PrimitiveTopology::LineList,
            "fs_line",
        );
        // line list rather than PolygonMode::Line, which needs a device feature
        let wire = build_pipeline(
            device,
            &layout,
            &shader,
            render_state.target_format,
            DrawState {
                cull: None,
                ..DrawState::opaque()
            },
            wgpu::PrimitiveTopology::LineList,
            "fs_wire",
        );

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(Preview {
                wire,
                highlight,
                grid,
                camera_bind_group,
                texture_layout: texture_layout.clone(),
                flipped: HashMap::new(),
            });

        // wgpu reports validation failures through `log`, and nothing here installs a logger
        render_state
            .device
            .on_uncaptured_error(std::sync::Arc::new(|error| {
                eprintln!("wgpu error: {error}");
            }));

        Self {
            render_state: render_state.clone(),
            model_layout,
            texture_layout,
            samplers,
            pipeline_layout: layout,
            camera_buffer,
        }
    }

    /// One pipeline per shader and draw state combination, so a shader that overrides its own
    /// blending gets its own rather than sharing whatever the file's properties asked for.
    fn pipeline(&self, state: DrawState, module: &wgpu::ShaderModule) -> wgpu::RenderPipeline {
        build_pipeline(
            &self.render_state.device,
            &self.pipeline_layout,
            module,
            self.render_state.target_format,
            state,
            wgpu::PrimitiveTopology::TriangleList,
            "fs_main",
        )
    }

    fn sampler(&self, sampling: shaders::Sampling) -> wgpu::Sampler {
        let at = |mode| ADDRESS_MODES.iter().position(|m| *m == mode).unwrap_or(0);
        let (u, v) = sampling.address;
        let filter = usize::from(sampling.filter == wgpu::FilterMode::Nearest);
        let modes = ADDRESS_MODES.len();
        self.samplers[filter * modes * modes + at(u) * modes + at(v)].clone()
    }

    fn upload_texture(&self, width: u32, height: u32, rgba: &[u8]) -> wgpu::TextureView {
        let device = &self.render_state.device;
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("nifty texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Not the Srgb variant: eframe's target is Bgra8Unorm and nothing here encodes
            // gamma on output, so decoding sRGB at sample time would make everything too dark.
            // Gamma space also matches D3D9 fixed function, which had no sRGB handling.
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.render_state.queue.write_texture(
            texture.as_image_copy(),
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            size,
        );

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// One bind group per shape, carrying the base, dark and glow slots with the address mode
    /// each `TexDesc` asked for. A slot the shape does not use gets a default that changes
    /// nothing: white for dark, since it multiplies, and black for glow, since it adds.
    fn slot_group(
        &self,
        slots: [(&wgpu::TextureView, &wgpu::Sampler); GROUP_SLOTS],
        cube: (&wgpu::TextureView, &wgpu::Sampler),
    ) -> wgpu::BindGroup {
        slot_bind_group(&self.render_state.device, &self.texture_layout, slots, cube)
    }

    /// One `NiSourceTexture`, embedded or from the library on disk.
    fn source_texture(
        &self,
        nif: &Nif,
        source_ref: nif::common::BlockRef,
        library: &TextureLibrary,
    ) -> Option<wgpu::TextureView> {
        let Some(Block::NiSourceTexture(source)) = source_ref.get(&nif.blocks) else {
            return None;
        };
        // most source textures name a file rather than carrying pixels
        if source.use_external {
            let requested = source.file_name.to_string_lossy().into_owned();
            let (width, height, rgba) = library.load(&requested)?;
            return Some(self.upload_texture(width, height, &rgba));
        }

        let Some(Block::NiPixelData(pixels)) = source.pixel_data_ref.get(&nif.blocks) else {
            return None;
        };
        let palette = match pixels.palette_ref.get(&nif.blocks) {
            Some(Block::NiPalette(palette)) => Some(palette),
            _ => None,
        };
        let (width, height, rgba) = decode_texture(pixels, palette)?;
        Some(self.upload_texture(width, height, &rgba))
    }

    /// Six faces as one cube texture. Every cube map in this game is one mipmap level, so only
    /// the top of each face is read, and they share a format and a size the way the device
    /// requires.
    fn upload_cube(&self, size: u32, faces: &[Vec<u8>; 6]) -> wgpu::TextureView {
        let device = &self.render_state.device;
        let extent = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("nifty cube"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        for (at, face) in faces.iter().enumerate() {
            self.render_state.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: at as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                face,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(size * 4),
                    rows_per_image: Some(size),
                },
                wgpu::Extent3d {
                    width: size,
                    height: size,
                    depth_or_array_layers: 1,
                },
            );
        }
        texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        })
    }

    /// A source block's texture, uploaded once per scene however many shapes name it. A failure
    /// is not remembered, since what stands in for a map that would not load differs between a
    /// slot and a reflection, and that is the caller's to decide.
    fn cached(
        cache: &mut HashMap<usize, wgpu::TextureView>,
        source: nif::common::BlockRef,
        upload: impl FnOnce() -> Option<wgpu::TextureView>,
    ) -> Option<wgpu::TextureView> {
        let key = source.index()?;
        if let Some(view) = cache.get(&key) {
            return Some(view.clone());
        }
        let view = upload()?;
        Some(cache.entry(key).or_insert(view).clone())
    }

    /// The cube map an effect names, or `None` where it names none or the faces do not agree.
    fn cube_texture(&self, nif: &Nif, source_ref: nif::common::BlockRef) -> Option<wgpu::TextureView> {
        let Some(Block::NiSourceCubeMap(source)) = source_ref.get(&nif.blocks) else {
            return None;
        };
        let Some(Block::NiPixelData(pixels)) = source.base.pixel_data_ref.get(&nif.blocks) else {
            return None;
        };
        let palette = match pixels.palette_ref.get(&nif.blocks) {
            Some(Block::NiPalette(palette)) => Some(palette),
            _ => None,
        };
        let mut size = 0;
        let mut collected: Vec<Vec<u8>> = Vec::with_capacity(6);
        for at in 0..6 {
            let (width, height, rgba) = crate::texture::decode_face(pixels, palette, at)?;
            // a cube face is square and every face matches, which the device insists on
            if width != height || (size != 0 && width != size) {
                return None;
            }
            size = width;
            collected.push(rgba);
        }
        let faces: [Vec<u8>; 6] = collected.try_into().ok()?;
        (size > 0).then(|| self.upload_cube(size, &faces))
    }

    /// A draw per shape, each carrying its own transform, not one merged mesh.
    /// Buffers for one particle system, sized once for its capacity. The vertices are rewritten
    /// every frame from the simulation, so the contents here are only a starting size.
    /// One pass's texture bindings: a group holding the maps it reads, plus a group per frame a
    /// flip controller can swap into its first binding. A slot the pass leaves unread gets the
    /// neutral texel for the way that slot combines.
    #[allow(clippy::too_many_arguments)]
    fn pass_textures(
        &self,
        nif: &Nif,
        pass: &shaders::Pass,
        slots: [Option<shaders::Source>; BOUND_SLOTS],
        property: Option<&nif::blocks::NiTexturingProperty>,
        texturing_block: Option<usize>,
        library: &TextureLibrary,
        environment: nif::common::BlockRef,
        blank_cube: &wgpu::TextureView,
        neutral: &[wgpu::TextureView; 3],
        cache: &mut HashMap<usize, wgpu::TextureView>,
        named_textures: &mut HashMap<&'static str, wgpu::TextureView>,
        missing: &wgpu::TextureView,
    ) -> (wgpu::BindGroup, PassBindings) {
        // a slot the shape does not use has to change nothing, and what that means depends
        // on how the slot is combined
        // the environment map defaults to black, since it adds rather than multiplies
        let mut views: [wgpu::TextureView; GROUP_SLOTS] = std::array::from_fn(|position| {
            match position < BOUND_SLOTS {
                true => neutral[pass.absent[position] as usize].clone(),
                false => neutral[shaders::Absent::Black as usize].clone(),
            }
        });
        let default_sampling = shaders::Sampling {
            address: (wgpu::AddressMode::Repeat, wgpu::AddressMode::Repeat),
            filter: wgpu::FilterMode::Linear,
        };
        let mut samplers: [wgpu::Sampler; GROUP_SLOTS] = std::array::from_fn(|position| {
            match position < BOUND_SLOTS {
                true => self.sampler(pass.address[position].unwrap_or(default_sampling)),
                // a reflection runs off the edge of its map, so it clamps rather than wrapping
                false => self.sampler(shaders::Sampling {
                    address: (
                        wgpu::AddressMode::ClampToEdge,
                        wgpu::AddressMode::ClampToEdge,
                    ),
                    filter: wgpu::FilterMode::Linear,
                }),
            }
        });

        for (position, slot) in slots.iter().enumerate() {
            let Some(slot) = slot else { continue };
            match slot {
                // a texture the shader names itself rather than one the file points at, so
                // it resolves by name through the library and root order picks the copy
                shaders::Source::Named(name) => {
                    if let Some((width, height, rgba)) = library.load(name) {
                        views[position] = named_textures
                            .entry(*name)
                            .or_insert_with(|| self.upload_texture(width, height, &rgba))
                            .clone();
                    }
                }
                // resolved into one of the other two before this point, since which map an
                // attribute texture reads is the shape's to say
                shaders::Source::Attribute { .. } | shaders::Source::IndexedSlot { .. } => continue,
                shaders::Source::Slot(slot) => {
                    let Some(desc) = property.and_then(|p| p.texture(*slot)) else {
                        continue;
                    };
                    // a slot that will not load shows the missing marker, which is cached with
                    // the rest so the failure is reported once rather than retried per shape
                    if desc.source_ref.index().is_some() {
                        views[position] = Self::cached(cache, desc.source_ref, || {
                            Some(
                                self.source_texture(nif, desc.source_ref, library)
                                    .unwrap_or_else(|| missing.clone()),
                            )
                        })
                        .unwrap_or_else(|| missing.clone());
                    }
                    // the shader's own sampler state beats the map's clamp mode
                    samplers[position] =
                        self.sampler(pass.address[position].unwrap_or(shaders::Sampling {
                            address: address_of(&desc.clamp_mode),
                            filter: wgpu::FilterMode::Linear,
                        }));
                }
            }
        }
        // A sphere map reflects whatever the effect names. Every one in this game asks for the
        // same filtering and clamping, so only the source varies.
        // A cube map is a second binding kind rather than another slot, so it is resolved
        // separately and the blank one stands in wherever a shape reflects nothing.
        let mut cube_view = blank_cube.clone();
        let cube_sampler = self.sampler(shaders::Sampling {
            address: (
                wgpu::AddressMode::ClampToEdge,
                wgpu::AddressMode::ClampToEdge,
            ),
            filter: wgpu::FilterMode::Linear,
        });
        if let Some(Block::NiTextureEffect(effect)) = environment.get(&nif.blocks) {
            let source = effect.source_texture_ref;
            match effect.coordinate_generation_type {
                // a cube is indexed by the reflection itself, a sphere by two of its components
                3 => {
                    if let Some(view) =
                        Self::cached(cache, source, || self.cube_texture(nif, source))
                    {
                        cube_view = view;
                    }
                }
                _ => {
                    if let Some(view) =
                        Self::cached(cache, source, || self.source_texture(nif, source, library))
                    {
                        views[ENV_SLOT] = view;
                    }
                }
            }
        }
        let texture = self.slot_group(
            std::array::from_fn(|i| (&views[i], &samplers[i])),
            (&cube_view, &cube_sampler),
        );
        // Which slot each binding position reads, so a flipped frame lands in the right one.
        // A position reading anything but a plain slot cannot be flipped and stays as it is.
        let bound_slots: [Option<TextureSlot>; BOUND_SLOTS] =
            std::array::from_fn(|position| match slots.get(position).copied().flatten() {
                Some(shaders::Source::Slot(slot)) => Some(slot),
                _ => None,
            });
        // Every frame any flip controller on this property can reach, uploaded once each. Only
        // the slots this pass actually binds are worth uploading, and only the slots anything is
        // ever seen to flip.
        let mut frames = HashMap::new();
        for block in nif.blocks.iter() {
            let Block::NiFlipController(flip) = block else {
                continue;
            };
            let Some(slot) = TextureSlot::from_flip_index(flip.texture_slot) else {
                continue;
            };
            if flip.target_ref.index() != texturing_block || !bound_slots.contains(&Some(slot)) {
                continue;
            }
            for source_ref in &flip.source_refs {
                let Some(index) = source_ref.index() else {
                    continue;
                };
                let view = cache
                    .entry(index)
                    .or_insert_with(|| {
                        self.source_texture(nif, *source_ref, library)
                            .unwrap_or_else(|| missing.clone())
                    })
                    .clone();
                frames.insert(index, view);
            }
        }
        let bindings = PassBindings {
            views,
            samplers,
            cube: (cube_view, cube_sampler),
            slots: bound_slots,
            frames,
        };
        (texture, bindings)
    }

    #[allow(clippy::too_many_arguments)]
    fn particle_mesh(
        &self,
        nif: &Nif,
        visit: &nif::walk::Visit<'_>,
        geometry: &NiGeometry,
        library: &TextureLibrary,
        module: &wgpu::ShaderModule,
        blank_cube: &wgpu::TextureView,
    ) -> Option<ParticleMesh> {
        let device = &self.render_state.device;
        let capacity = match geometry.data_ref.get(&nif.blocks) {
            Some(Block::NiPSysData(data)) => data.vertex_count(),
            _ => 0,
        };
        if capacity == 0 {
            return None;
        }

        // the furthest a particle can travel is its fastest speed over its longest life, and
        // both take their variation the way the emitter applies it
        let reach = nif
            .blocks
            .iter()
            .filter_map(|block| match block {
                Block::NiPSysBoxEmitter(e) => Some(&e.base.base),
                Block::NiPSysCylinderEmitter(e) => Some(&e.base.base),
                Block::NiPSysSphereEmitter(e) => Some(&e.base.base),
                Block::NiPSysMeshEmitter(e) => Some(&e.base),
                _ => None,
            })
            .map(|emitter| {
                let speed = emitter.speed + emitter.speed_variation * 0.5;
                let life = emitter.life_span + emitter.life_span_variation * 0.5;
                let radius = emitter.initial_radius + emitter.radius_variation;
                (speed * life).max(0.0) + radius.max(0.0)
            })
            .fold(0.0f32, f32::max);

        let alpha = match visit.properties.alpha.get(&nif.blocks) {
            Some(Block::NiAlphaProperty(p)) => Some(p),
            _ => None,
        };
        let blend = alpha.filter(|a| a.alpha_blend()).map(|a| {
            (
                blend_factor(&a.source_blend_mode(), false),
                blend_factor(&a.destination_blend_mode(), true),
            )
        });
        // a system is blended geometry like any other, so it leaves the sort on the same terms
        let sorted = sorts(blend.is_some(), alpha);
        let state = DrawState {
            // a quad already faces the camera, so culling it would only ever hide it
            cull: None,
            // particles are a haze over the scene rather than part of it
            depth_write: false,
            depth: wgpu::CompareFunction::LessEqual,
            blend,
        };

        // four corners a quad, and two triangles wound the way the renderer expects
        let mut indices: Vec<u16> = Vec::with_capacity(capacity * 6);
        let mut edges: Vec<u16> = Vec::with_capacity(capacity * 8);
        for quad in 0..capacity.min(u16::MAX as usize / 4) {
            let base = (quad * 4) as u16;
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            let corner = |i: u16| base + i;
            for side in 0..4u16 {
                edges.extend_from_slice(&[corner(side), corner((side + 1) % 4)]);
            }
        }

        let mut uniform = [0f32; MODEL_FLOATS as usize];
        // the quads are built in world space, so the model matrix has nothing left to do
        uniform[..16].copy_from_slice(&Mat4::IDENTITY.to_cols_array());
        uniform[16..20].copy_from_slice(&[1.0, 1.0, 1.0, 1.0]);
        // the particle's own colour drives emissive with lighting off, which is what makes a
        // particle glow at its own brightness rather than take the scene's
        uniform[24..28].copy_from_slice(&[1.0, 0.0, 0.0, 0.0]);
        uniform[32..64].copy_from_slice(&slot_uv_rows(
            &nif.blocks,
            None,
            DEFAULT_SLOTS,
            [None; BOUND_SLOTS],
            0.0,
        ));

        let view = match visit.properties.texturing.get(&nif.blocks) {
            Some(Block::NiTexturingProperty(p)) => Some(p),
            _ => None,
        }
        .and_then(|property| property.texture(TextureSlot::Base))
        .and_then(|desc| self.source_texture(nif, desc.source_ref, library));
        let white = self.upload_texture(1, 1, &[255, 255, 255, 255]);
        let sampler = self.sampler(shaders::Sampling {
            address: (
                wgpu::AddressMode::ClampToEdge,
                wgpu::AddressMode::ClampToEdge,
            ),
            filter: wgpu::FilterMode::Linear,
        });
        let view = view.unwrap_or(white);

        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("nifty particles model"),
            contents: bytemuck::cast_slice(&uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Some(ParticleMesh {
            block: visit.index,
            world_space: match nif.blocks.get(visit.index) {
                Some(Block::NiParticleSystem(psys)) => psys.world_space,
                _ => false,
            },
            model: Mat4::from(&visit.transform),
            capacity,
            sorted,
            // filled by the caller, which is where the LOD ancestry is known
            lod: None,
            reach: reach * visit.transform.scale.abs(),
            pipeline: self.pipeline(state, module),
            texture: self.slot_group(
                std::array::from_fn(|_| (&view, &sampler)),
                // a particle reflects nothing, so its cube is the black one
                (blank_cube, &sampler),
            ),
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("nifty particles"),
                layout: &self.model_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_buffer.as_entire_binding(),
                }],
            }),
            vertices: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("nifty particles"),
                size: (capacity * 4 * VERTEX_FLOATS * 4) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            indices: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("nifty particles"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            edges: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("nifty particle edges"),
                contents: bytemuck::cast_slice(&edges),
                usage: wgpu::BufferUsages::INDEX,
            }),
        })
    }

    /// Returns the scene and the technique names it asked for that nothing could draw, so the
    /// caller can report them rather than let a wrong render pass as a right one.
    pub fn build_scene(
        &self,
        nif: &Nif,
        library: &TextureLibrary,
        shaders: &Shaders,
    ) -> (Scene, Vec<String>, Vec<String>) {
        let lod_of = lod_ancestry(nif);
        let device = &self.render_state.device;
        // one per Absent variant, since what an unread slot stands in with depends on how the
        // slot combines: white where it multiplies, black where it adds, half where it doubles
        // a shape reflecting nothing samples a black cube, since the reflection adds
        let blank_cube = self.upload_cube(1, &std::array::from_fn(|_| vec![0u8, 0, 0, 255]));
        let neutral: [wgpu::TextureView; 3] = std::array::from_fn(|i| {
            let texel = match i {
                0 => shaders::Absent::White,
                1 => shaders::Absent::Black,
                _ => shaders::Absent::Half,
            };
            self.upload_texture(1, 1, &texel.texel())
        });
        let white = neutral[shaders::Absent::White as usize].clone();
        // shapes whose texture could not be loaded get a checker rather than white
        let missing = {
            let (width, height, rgba) = library::placeholder();
            self.upload_texture(width, height, &rgba)
        };
        let mut cache: HashMap<usize, wgpu::TextureView> = HashMap::new();
        let mut named_textures: HashMap<&'static str, wgpu::TextureView> = HashMap::new();
        // keyed by shader name and pass index, since a shader's passes are separate modules
        let mut pipelines: HashMap<((String, usize), DrawState), wgpu::RenderPipeline> =
            HashMap::new();
        let mut modules: HashMap<(String, usize), Result<wgpu::ShaderModule, String>> =
            HashMap::new();
        let fixed = shaders.fixed();
        let fixed_module = compile(device, &fixed.name, &fixed.passes[0])
            .expect("the built in fixed function shader has to compile");
        let mut unhandled: Vec<String> = Vec::new();
        // each shape's own box, kept so the animated sweep can move it without touching
        // vertices again
        let mut boxes: HashMap<usize, (Vec3, Vec3)> = HashMap::new();
        // A technique that is drawn but not wholly. Reporting only the ones nothing can draw
        // leaves a partial reading looking finished, which is the same trap the fixed function
        // fallback was: it is the not saying that makes it wrong.
        let mut partial: Vec<String> = Vec::new();
        let mut meshes = Vec::new();
        let mut particles = Vec::new();
        let mut lods: HashMap<usize, Lod> = HashMap::new();
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        // Where the scene is drawn from, taken from the first thing in it. It only has to be
        // near the geometry, not at its centre, and the centre is not known until the bounds
        // are. A file whose parts are spread over a huge range keeps some of the error, which
        // is the format's own: its transforms are f32 at that magnitude too.
        let origin = nif
            .walk()
            .find(|visit| {
                matches!(visit.block, Block::NiParticleSystem(_))
                    || geometry_of(nif, visit.block).is_some()
            })
            .map(|visit| Mat4::from(&visit.transform).w_axis.truncate())
            .unwrap_or(Vec3::ZERO);

        // one span for the whole file, since every shape's sweep runs over the same one
        let span = nif::anim::span(&nif.blocks);
        // every block's resting world transform, since a bone is a node the shape does not own
        // and placing a skinned shape means reaching one
        let rest_pose: HashMap<usize, Mat4> = nif
            .walk()
            .map(|visit| (visit.index, Mat4::from(&visit.transform)))
            .collect();

        // a light named only by an effect list is never reached by the walk, so it is placed
        // by the node naming it, which is where it would have sat as a child
        let mut light_world: HashMap<usize, Mat4> = HashMap::new();
        for visit in nif.walk() {
            for reference in visit.block.effect_refs().unwrap_or(&[]) {
                let Some(index) = reference.index() else {
                    continue;
                };
                let Some(local) = nif.blocks.get(index).and_then(Block::av_object) else {
                    continue;
                };
                let placed = rest_pose.get(&index).copied().unwrap_or_else(|| {
                    Mat4::from(&visit.transform) * Mat4::from(&nif::common::NiTransform::from(local))
                });
                light_world.insert(index, placed);
            }
        }

        // a file carries at most one camera, so the first the walk reaches is it
        let scene_camera = nif.walk().find_map(|visit| match visit.block {
            Block::NiCamera(camera) => {
                let height = camera.frustum_top - camera.frustum_bottom;
                let width = camera.frustum_right - camera.frustum_left;
                (height > 0.0 && width > 0.0 && camera.frustum_near > 0.0).then(|| SceneCamera {
                    block: visit.index,
                    // the frustum is given at the near plane, so the angle falls out of it
                    fov: 2.0 * (camera.frustum_top / camera.frustum_near).atan(),
                    aspect: width / height,
                    near: camera.frustum_near,
                    far: camera.frustum_far,
                })
            }
            _ => None,
        });

        // a light is resolved once and shapes name it by index afterwards. An ambient light
        // folds into the scene's ambient term instead of becoming one of these
        let mut scene_lights: Vec<nif::light::Lit> = Vec::new();
        let mut light_blocks: Vec<usize> = Vec::new();
        let mut light_at: HashMap<usize, usize> = HashMap::new();
        let mut ambient = Vec3::ZERO;
        let mut any_ambient = false;
        for visit in nif.walk() {
            for reference in visit.lights.iter() {
                let Some(index) = reference.index() else {
                    continue;
                };
                if light_at.contains_key(&index) {
                    continue;
                }
                let world = light_world.get(&index).copied().unwrap_or(Mat4::IDENTITY);
                let Some(lit) = nif
                    .blocks
                    .get(index)
                    .and_then(|b| nif::light::resolve(b, world, None))
                else {
                    continue;
                };
                if lit.falloff == nif::light::Falloff::Ambient {
                    ambient += lit.ambient;
                    any_ambient = true;
                    light_at.insert(index, usize::MAX);
                    continue;
                }
                if scene_lights.len() >= SCENE_LIGHTS {
                    continue;
                }
                light_at.insert(index, scene_lights.len());
                light_blocks.push(index);
                scene_lights.push(lit);
            }
        }
        let scene_ambient = any_ambient.then_some(ambient);
        for visit in nif.walk() {
            // the walk is the only place the node's world transform is known
            if let Block::NiLODNode(node) = visit.block {
                if let Some(Block::NiRangeLODData(data)) = node.lod_level_data_ref.get(&nif.blocks)
                {
                    let local = Vec3::from(&data.center);
                    lods.insert(
                        visit.index,
                        Lod {
                            center: Mat4::from(&visit.transform).transform_point3(local),
                            local_center: local,
                            ranges: data
                                .lod_levels
                                .iter()
                                .map(|range| (range.near, range.far))
                                .collect(),
                        },
                    );
                }
            }

            // A particle system carries no stored geometry, so its quads are generated per frame
            // and it takes buffers sized for its capacity. This has to come before `geometry_of`,
            // which only resolves the shapes that store triangles.
            if let Block::NiParticleSystem(psys) = visit.block {
                let mesh = self.particle_mesh(
                    nif,
                    &visit,
                    &psys.base,
                    library,
                    &fixed_module,
                    &blank_cube,
                );
                if let Some(mut mesh) = mesh {
                    mesh.lod = lod_of.get(&visit.index).copied();
                    // A particle starts at the object its emitter names, not at the system's own
                    // node, so the bounds are taken around each of those. Measuring from the node
                    // put the floor and the framing somewhere the particles never reach whenever
                    // the two differ, which in this corpus is most of the time.
                    let objects = nif::psys::System::emitter_objects(&nif.blocks, visit.index);
                    let mut from: Vec<Vec3> = objects
                        .iter()
                        .filter_map(|object| rest_pose.get(object))
                        .map(|pose| pose.transform_point3(Vec3::ZERO))
                        .collect();
                    if from.is_empty() {
                        from.push(mesh.model.transform_point3(Vec3::ZERO));
                    }
                    for centre in from {
                        min = min.min(centre - Vec3::splat(mesh.reach));
                        max = max.max(centre + Vec3::splat(mesh.reach));
                    }
                    particles.push(mesh);
                }
                continue;
            }
            let Some((geometry, data, triangles)) = geometry_of(nif, visit.block) else {
                continue;
            };
            let Some(vertices) = &data.vertices else {
                continue;
            };
            if vertices.is_empty() || triangles.is_empty() {
                continue;
            }

            // A skinned shape's own transform does not place it: the bones do, and its stored
            // vertices are in skin space. So it is deformed once here for the resting pose and
            // drawn with no model transform, and the frame replaces this whenever bones move.
            let rest =
                nif::skin::deform(&nif.blocks, geometry, |index| rest_pose.get(&index).copied());
            let skinned = rest.is_some();
            let model = match skinned {
                true => Mat4::IDENTITY,
                false => Mat4::from(&visit.transform),
            };
            let colors = data.vertex_colors.as_ref();
            let vertices = rest.as_ref().map_or(vertices, |skin| &skin.positions);
            let normals = rest
                .as_ref()
                .and_then(|skin| skin.normals.as_ref())
                .or(data.normals.as_ref());
            // the dark slot almost always reads uv set 1, so more than one set goes up and
            // each slot picks the one its own TexDesc names
            let uvs = data.uv_sets.first().map(|set| &set.uvs);
            let uvs1 = data.uv_sets.get(1).map(|set| &set.uvs).or(uvs);
            let uvs2 = data.uv_sets.get(2).map(|set| &set.uvs).or(uvs);
            let mut attributes: Vec<f32> = Vec::with_capacity(vertices.len() * VERTEX_FLOATS);
            let mut shape_min = Vec3::splat(f32::MAX);
            let mut shape_max = Vec3::splat(f32::MIN);
            let mut local_min = Vec3::splat(f32::MAX);
            let mut local_max = Vec3::splat(f32::MIN);
            for (i, v) in vertices.iter().enumerate() {
                let local = Vec3::from(v);
                let world = model.transform_point3(local);
                min = min.min(world);
                max = max.max(world);
                shape_min = shape_min.min(world);
                shape_max = shape_max.max(world);
                local_min = local_min.min(local);
                local_max = local_max.max(local);

                let color = colors
                    .and_then(|c| c.get(i))
                    .map(|c| [c.r, c.g, c.b, c.a])
                    .unwrap_or([1.0, 1.0, 1.0, 1.0]);
                let uv = uvs
                    .and_then(|set| set.get(i))
                    .map(|t| [t.u, t.v])
                    .unwrap_or([0.0, 0.0]);
                let uv1 = uvs1
                    .and_then(|set| set.get(i))
                    .map(|t| [t.u, t.v])
                    .unwrap_or(uv);
                let uv2 = uvs2
                    .and_then(|set| set.get(i))
                    .map(|t| [t.u, t.v])
                    .unwrap_or(uv);
                let normal = normals
                    .and_then(|n| n.get(i))
                    .map(|n| [n.x, n.y, n.z])
                    .unwrap_or([0.0, 0.0, 0.0]);
                attributes.extend_from_slice(&[v.x, v.y, v.z]);
                attributes.extend_from_slice(&normal);
                attributes.extend_from_slice(&color);
                attributes.extend_from_slice(&uv);
                attributes.extend_from_slice(&uv1);
                attributes.extend_from_slice(&uv2);
            }

            // A shape drawn from vertices the file does not store needs every box taken from
            // those vertices to grow and cover where it is carried. The far plane, the camera's
            // framing and the sort centre all come from these.
            let reach = match skinned {
                true => skin_reach(nif, geometry, span),
                false => morph_reach(nif, geometry, span),
            };
            if let Some((low, high)) = reach {
                local_min = local_min.min(low);
                local_max = local_max.max(high);
                // an affine transform takes a box to the hull of its own eight corners, so
                // these bound the moved shape in world space without moving every vertex
                for corner in box_corners(low, high) {
                    let world = model.transform_point3(corner);
                    min = min.min(world);
                    max = max.max(world);
                    shape_min = shape_min.min(world);
                    shape_max = shape_max.max(world);
                }
            }

            // NiMaterialProperty is a D3DMATERIAL9 verbatim. There is no ambient term: the
            // engine multiplies material ambient by the global ambient, which is black unless
            // the scene carries an NiAmbientLight.
            // the block index as well as the material, since that is what a controller targets
            let in_force = visit.properties;
            let material_index = in_force.material.index();
            let material = material_index
                .and_then(|index| nif.blocks.get(index))
                .and_then(|block| match block {
                    Block::NiMaterialProperty(m) => Some(m),
                    _ => None,
                });
            let (diffuse, emissive) = match material {
                Some(m) => (
                    [
                        m.color_diffuse.r,
                        m.color_diffuse.g,
                        m.color_diffuse.b,
                        m.alpha,
                    ],
                    [
                        m.color_emissive.r,
                        m.color_emissive.g,
                        m.color_emissive.b,
                        0.0,
                    ],
                ),
                None => ([1.0; 4], [0.0; 4]),
            };
            // the other two material channels, which the fixed function path folds into the
            // light sum and never uploads. AGCar2 reads them as flat colours instead, so a
            // shader that wants them has them.
            let (ambient, specular) = match material {
                Some(m) => (
                    [m.color_ambient.r, m.color_ambient.g, m.color_ambient.b, 1.0],
                    [
                        m.color_specular.r,
                        m.color_specular.g,
                        m.color_specular.b,
                        m.glossiness,
                    ],
                ),
                None => ([1.0; 4], [1.0; 4]),
            };

            // NiVertexColorProperty selects which source supplies each D3D material channel
            // rather than tinting. LightMode::Emissive uploads no lights, and with SourceEmissive
            // it disables lighting so the vertex colour is used directly.
            let vertex_color = match in_force.vertex_color.get(&nif.blocks) {
                Some(Block::NiVertexColorProperty(p)) => Some(p),
                _ => None,
            };
            let (emissive_from_vertex, diffuse_from_vertex, lighting) = match vertex_color {
                Some(p) => match (&p.lighting_mode, &p.vertex_mode) {
                    (LightMode::Emissive, VertMode::SourceEmissive) => (1.0, 0.0, 0.0),
                    (LightMode::Emissive, _) => (0.0, 0.0, 0.0),
                    (_, VertMode::SourceEmissive) => (1.0, 0.0, 1.0),
                    (_, VertMode::SourceAmbientDiffuse) => (0.0, 1.0, 1.0),
                    _ => (0.0, 0.0, 1.0),
                },
                None => (0.0, 0.0, 1.0),
            };

            // a base texture under APPLY_REPLACE disables lighting entirely and the stage
            // selects the texel alone
            let texturing = match in_force.texturing.get(&nif.blocks) {
                Some(Block::NiTexturingProperty(p)) => Some(p),
                _ => None,
            };
            let replace = match texturing {
                Some(p) => f32::from(p.apply_mode == ApplyMode::Replace),
                None => 0.0,
            };

            let stencil = match in_force.stencil.get(&nif.blocks) {
                Some(Block::NiStencilProperty(p)) => Some(p),
                _ => None,
            };
            let zbuffer = match in_force.z_buffer.get(&nif.blocks) {
                Some(Block::NiZBufferProperty(p)) => Some(p),
                _ => None,
            };
            let alpha = match in_force.alpha.get(&nif.blocks) {
                Some(Block::NiAlphaProperty(p)) => Some(p),
                _ => None,
            };

            // a technique nothing can draw is recorded rather than approximated, and falls
            // back to the fixed function path so the geometry is still inspectable
            let technique = geometry
                .material_data
                .shader()
                .map(|s| s.name.to_string_lossy().into_owned())
                .unwrap_or_default();
            let shader = match shaders.get(&technique) {
                Some(shader) => shader,
                None => {
                    unhandled.push(technique);
                    shaders.fixed()
                }
            };
            // the technique name is declared twice and the other one is normal mapped. Nothing
            // at the technique level tells them apart, so a shape carrying tangent space gets
            // this one with its normal map unread
            if shader.name == "AGCar2" && data.nbt_method() != 0 {
                partial.push("AGCar2 normal mapping".into());
            }
            // A shader's own pass state beats what the file's properties ask for, and the two
            // are separate: turning the alpha test off must not also decide the blending, and a
            // shader that forces blending on has to be sorted as blended even when the file
            // asks for none.
            let from_file = alpha.filter(|a| a.alpha_blend()).map(|a| {
                (
                    blend_factor(&a.source_blend_mode(), false),
                    blend_factor(&a.destination_blend_mode(), true),
                )
            });
            // the last pass decides how the shape blends and sorts, since that is the one whose
            // result lands over the others. Every pass a shader draws is state of its own.
            let last = shader.passes.last().expect("a shader draws at least once");
            let blend = last.state.blend.unwrap_or(from_file);
            let tested = alpha.filter(|_| last.state.alpha_test != Some(false));
            // the shader implements TestGreater only. TestAlways never discards.
            let (alpha_test, alpha_threshold) = match tested {
                Some(a) if a.alpha_test() && a.test_func() != TestFunction::TestAlways => {
                    (1.0, f32::from(a.threshold) / 255.0)
                }
                _ => (0.0, 0.0),
            };

            // one upload per source texture, not per shape that uses it
            let texture_key = texturing.and_then(|p| p.base_texture.as_ref()?.source_ref.index());
            let property = texturing;
            let texturing_block = in_force.texturing.index();

            // the uv rows in the uniform are one pass's, so they belong to the first pass that
            // samples anything. A shader whose passes read different slots with different
            // transforms is not expressible yet, and none of them does.
            let resolved: Vec<[Option<shaders::Source>; BOUND_SLOTS]> = shader
                .passes
                .iter()
                .map(|pass| shader_slots(&nif.blocks, &geometry.extra_data_refs, pass.slots))
                .collect();
            let bound_at = resolved
                .iter()
                .position(|slots| slots.iter().any(Option::is_some));
            let bound = bound_at.map_or([None; BOUND_SLOTS], |at| resolved[at]);
            let uv_pins = bound_at.map_or([None; BOUND_SLOTS], |at| shader.passes[at].uv_set);

            let mut mesh_passes: Vec<MeshPass> = Vec::with_capacity(shader.passes.len());
            for (index, pass) in shader.passes.iter().enumerate() {
                let state = DrawState {
                    // a pass can set its own cull, which is how an outline hull shows only its
                    // far side, and that beats the file's own stencil property
                    cull: pass
                        .state
                        .cull
                        .unwrap_or_else(|| cull_of(stencil.map(|p| &p.draw_mode))),
                    depth_write: pass
                        .state
                        .depth_write
                        .unwrap_or_else(|| zbuffer.is_none_or(|z| z.depth_write())),
                    depth: depth_of(zbuffer),
                    blend: pass.state.blend.unwrap_or(from_file),
                };
                let key = (shader.name.clone(), index);
                let module = modules
                    .entry(key.clone())
                    .or_insert_with(|| compile(device, &shader.name, pass));
                // a shader that failed to compile falls back rather than taking the viewer down
                let module = match module {
                    Ok(module) => module,
                    Err(_) => &fixed_module,
                };
                let pipeline = pipelines
                    .entry((key.clone(), state))
                    .or_insert_with(|| self.pipeline(state, module))
                    .clone();
                let unculled = DrawState {
                    cull: None,
                    ..state
                };
                let pipeline_unculled = pipelines
                    .entry((key, unculled))
                    .or_insert_with(|| self.pipeline(unculled, module))
                    .clone();
                let (texture, bindings) = self.pass_textures(
                    nif,
                    pass,
                    resolved[index],
                    property,
                    texturing_block,
                    library,
                    visit.effects.environment(),
                    &blank_cube,
                    &neutral,
                    &mut cache,
                    &mut named_textures,
                    &missing,
                );
                mesh_passes.push(MeshPass {
                    pipeline,
                    pipeline_unculled,
                    texture,
                    bindings,
                });
            }
            // the bits name the scene's lights, so a shape lit by a subset carries a subset
            let light_mask = visit
                .lights
                .iter()
                .filter_map(|r| r.index())
                .filter_map(|i| light_at.get(&i))
                .filter(|at| **at != usize::MAX)
                .fold(0u32, |mask, at| mask | (1 << at));
            let mut model_uniform = [0f32; MODEL_FLOATS as usize];
            model_uniform[..16].copy_from_slice(&drawn_at(origin, model).to_cols_array());
            model_uniform[16..20].copy_from_slice(&diffuse);
            model_uniform[20..24].copy_from_slice(&emissive);
            model_uniform[24..28].copy_from_slice(&[
                emissive_from_vertex,
                diffuse_from_vertex,
                lighting,
                replace * f32::from(texture_key.is_some()),
            ]);
            model_uniform[28..32].copy_from_slice(&[alpha_threshold, alpha_test, 0.0, 0.0]);
            model_uniform[32..64].copy_from_slice(&slot_uv_rows(
                &nif.blocks,
                property,
                bound,
                uv_pins,
                0.0,
            ));
            model_uniform[64..68].copy_from_slice(&shader_params(
                &nif.blocks,
                &geometry.extra_data_refs,
                shader,
            ));
            model_uniform[72..76].copy_from_slice(&ambient);
            model_uniform[76..80].copy_from_slice(&specular);
            model_uniform[80] = light_mask as f32;
            // the engine leaves specular off unless a property switches it on
            model_uniform[81] = f32::from(matches!(
                in_force.specular.get(&nif.blocks),
                Some(Block::NiSpecularProperty(p)) if p.is_enabled()
            ));
            model_uniform[68..72].copy_from_slice(&shader_color(
                &nif.blocks,
                &geometry.extra_data_refs,
                shader,
            ));

            let mut indices: Vec<u16> = Vec::with_capacity(triangles.len() * 3);
            let mut edges: Vec<u16> = Vec::with_capacity(triangles.len() * 6);
            for triangle in &triangles {
                let (a, b, c) = (triangle.a, triangle.b, triangle.c);
                indices.extend_from_slice(&[a, b, c]);
                edges.extend_from_slice(&[a, b, b, c, c, a]);
            }

            let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("nifty model"),
                contents: bytemuck::cast_slice(&model_uniform),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            // A skinned shape's box is already in world space and its bones, not its own
            // transform, move it, so the transform sweep has nothing to say about it. Its own
            // sweep is in the bounds above already.
            if !skinned {
                boxes.insert(visit.index, (local_min, local_max));
            }
            // only a shape whose geometry is rewritten pays to keep its attributes
            let deform_source = (reach.is_some() || skinned).then(|| attributes.clone());
            let deforms = deform_source.is_some();
            meshes.push(Mesh {
                shape_block: visit.index,
                lod: lod_of.get(&visit.index).copied(),
                sorted: sorts(blend.is_some(), alpha),
                deform_source,
                skinned,
                uv_pins,
                attributes: (
                    shader.param_names,
                    shader_params(&nif.blocks, &geometry.extra_data_refs, shader),
                ),
                passes: mesh_passes,
                center: (shape_min + shape_max) * 0.5,
                local_center: (local_min + local_max) * 0.5,
                material_block: material_index,
                texturing_block,
                bound,
                radius: ((shape_max - shape_min).length() * 0.5).max(0.001),
                data_block: geometry.data_ref.index().unwrap_or(usize::MAX),
                vertices: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("nifty vertices"),
                    contents: bytemuck::cast_slice(&attributes),
                    // a shape the frame rewrites needs the buffer to be writable, and only it
                    usage: if deforms {
                        wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                    } else {
                        wgpu::BufferUsages::VERTEX
                    },
                }),
                indices: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("nifty indices"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                }),
                count: indices.len() as u32,
                edges: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("nifty edges"),
                    contents: bytemuck::cast_slice(&edges),
                    usage: wgpu::BufferUsages::INDEX,
                }),
                edge_count: edges.len() as u32,
                bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("nifty model"),
                    layout: &self.model_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: model_buffer.as_entire_binding(),
                    }],
                }),
                model_buffer,
            });
        }

        // min/max, not a half-extent about the origin, since terrain chunks sit far off it.
        // Particle systems extend the bounds too, so a file made only of them still frames.
        let (center, radius) = if meshes.is_empty() && particles.is_empty() {
            (Vec3::ZERO, 1.0)
        } else {
            ((min + max) * 0.5, (max - min).length() * 0.5)
        };
        let animated_radius = swept_radius(nif, center, radius, &boxes);

        let samplers_for_grid = self.sampler(shaders::Sampling {
            address: (wgpu::AddressMode::Repeat, wgpu::AddressMode::Repeat),
            filter: wgpu::FilterMode::Linear,
        });
        // sized to the scene, not to how far the scene is from the origin, and placed on the
        // ground below it. The z stays at the world's own floor so height still reads truthfully.
        let (lines, spacing, half) = grid_lines(radius);
        let ground = grid_ground(center, spacing);
        let mut identity = [0f32; MODEL_FLOATS as usize];
        identity[..16]
            .copy_from_slice(&drawn_at(origin, Mat4::from_translation(ground)).to_cols_array());
        identity[32..64].copy_from_slice(&slot_uv_rows(
            &[],
            None,
            DEFAULT_SLOTS,
            [None; BOUND_SLOTS],
            0.0,
        ));
        let grid = Grid {
            half,
            center: ground,
            count: (lines.len() / VERTEX_FLOATS) as u32,
            vertices: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("nifty grid"),
                contents: bytemuck::cast_slice(&lines),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            model: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("nifty grid"),
                layout: &self.model_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("nifty grid model"),
                            contents: bytemuck::cast_slice(&identity),
                            usage: wgpu::BufferUsages::UNIFORM,
                        })
                        .as_entire_binding(),
                }],
            }),
            // the layout carries a texture group whether the shader samples it or not
            texture: self.slot_group(
                std::array::from_fn(|_| (&white, &samplers_for_grid)),
                (&blank_cube, &samplers_for_grid),
            ),
            spacing,
        };

        unhandled.sort();
        unhandled.dedup();
        partial.sort();
        partial.dedup();
        (
            Scene {
                camera: scene_camera,
                lights: scene_lights,
                light_blocks,
                ambient: scene_ambient,
                origin,
                meshes,
                particles,
                center,
                radius: radius.max(0.001),
                animated_radius: animated_radius.max(radius).max(0.001),
                lods,
                grid,
            },
            unhandled,
            partial,
        )
    }
}

/// A drawable shape's geometry, its vertex data and its triangles, whether the file stores
/// those as a triangle list or as strips.
pub(crate) fn geometry_of<'a>(
    nif: &'a Nif,
    block: &'a Block,
) -> Option<(&'a NiGeometry, &'a NiGeometryData, Vec<Triangle>)> {
    let geometry = block.geometry()?;
    let (data, triangles) = block.triangles(&nif.blocks)?;
    Some((geometry, data, triangles))
}

/// A uv transform as the two rows that reach the shader, since the third is always (0, 0, 1).
/// No transform is the identity, which matters: a map with none must keep its uvs untouched,
/// because the engine's substitute carries a v flip rather than being neutral.
/// `uv_set` rides in the first row's spare lane, since the shader has to know which of the two
/// sets in the vertex buffer this slot reads. Dark is the reason there are two.
pub fn uv_rows(transform: Option<nif::blocks::TextureTransform>, uv_set: u32) -> [f32; 8] {
    let set = if uv_set == 0 { 0.0 } else { 1.0 };
    let Some(transform) = transform else {
        return [1.0, 0.0, 0.0, set, 0.0, 1.0, 0.0, 0.0];
    };
    let m = transform.matrix();
    let (x, y) = (m.x_axis, m.y_axis);
    [x.x, y.x, m.z_axis.x, set, x.y, y.y, m.z_axis.y, 0.0]
}

/// What a shader's attributes resolve to for one shape. An attribute is bound from an extra data
/// block on the shape itself whose name matches, and falls back to the value the shader declares
/// only where the shape carries none. A shape's own float is therefore what the engine drew with,
/// so a shell asking for a fifth of its opacity has to be read rather than defaulted.
pub fn shader_params(blocks: &[Block], extra_data_refs: &[BlockRef], shader: &Shader) -> [f32; 4] {
    let mut params = shader.params;
    for (lane, attribute) in shader.param_names.iter().enumerate() {
        if attribute.is_empty() {
            continue;
        }
        let supplied = extra_data_refs
            .iter()
            .filter_map(|reference| reference.get(blocks))
            .find_map(|block| match block {
                Block::NiFloatExtraData(float) if float.name.as_bytes() == attribute.as_bytes() => {
                    Some(float.value)
                }
                _ => None,
            });
        if let Some(value) = supplied {
            params[lane] = value;
        }
    }
    params
}

/// Every drawn slot's uv rows at `time`, in `DEFAULT_SLOTS` order. Both the scene build and the
/// per frame update go through this, so an animated slot cannot be one the build forgot.
pub fn slot_uv_rows(
    blocks: &[Block],
    property: Option<&nif::blocks::NiTexturingProperty>,
    bound: [Option<shaders::Source>; BOUND_SLOTS],
    pinned: [Option<u32>; BOUND_SLOTS],
    time: f32,
) -> [f32; BOUND_SLOTS * 8] {
    let mut out = [0.0; BOUND_SLOTS * 8];
    for (position, slot) in bound.iter().enumerate() {
        // A texture the shader names itself carries no TexDesc, so it has no transform and no
        // set of its own. A technique that pins one still applies to it, since the pin comes
        // from that technique's vertex shader rather than from any map.
        let (transform, own_set) = match *slot {
            Some(shaders::Source::Slot(slot)) => (
                property.and_then(|p| nif::anim::texture_transform_at(blocks, p, slot, time)),
                property
                    .and_then(|p| p.texture(slot))
                    .map_or(0, |d| d.uv_set),
            ),
            _ => (None, 0),
        };
        // a technique that names the set beats the map, since its vertex shader is what runs
        let rows = uv_rows(transform, pinned[position].unwrap_or(own_set));
        out[position * 8..(position + 1) * 8].copy_from_slice(&rows);
    }
    out
}

/// How many cells the grid will draw either side of its centre before it gives up on the
/// spacing it wanted and takes a coarser one. Sized so a scene of a few hundred units still
/// gets a cell for every unit.
///
/// Coarsening rather than clamping is what keeps the floor covering the scene: the spacing is
/// raised until the cell count fits, so the count never has to be cut.
const GRID_MAX_CELLS: i32 = 1024;

/// The finest the grid will subdivide for a model smaller than a few cells across.
const GRID_MIN_SPACING: f32 = 1e-4;

/// The gap between grid lines for a scene reaching `reach` from its centre.
///
/// One unit per cell where the scene allows it. It coarsens only when a scene is too wide to
/// draw at that, so a cell means the same thing across files and a size can be read off the
/// floor.
fn grid_spacing(reach: f32) -> f32 {
    let reach = reach.max(1e-3);
    let mut spacing = 1.0f32;
    while reach / spacing > GRID_MAX_CELLS as f32 {
        spacing *= 10.0;
    }
    // and a model smaller than a few units across gets a finer one, or it sits inside one cell
    while reach / spacing < 4.0 && spacing > GRID_MIN_SPACING {
        spacing /= 10.0;
    }
    spacing
}

/// How bright a grid line at `index` cells from the centre is drawn.
///
/// The tiers are keyed to where the line falls in the file's own units, not to a count of
/// lines, so the one unit line is the one unit line at every spacing. Read off the index
/// instead, the bright line means a different distance in every file.
fn grid_tier(index: i32, spacing: f32) -> [f32; 4] {
    // how many lines fall inside one unit, which is 1 once a cell is a unit or wider
    let per_unit = (1.0 / spacing).round().max(1.0) as i32;
    if index % per_unit.saturating_mul(10).max(1) == 0 {
        // ten units, the coarsest reading
        [0.52, 0.55, 0.62, 1.0]
    } else if index % per_unit == 0 {
        // exactly one unit, which is the line worth finding
        [0.40, 0.43, 0.50, 1.0]
    } else {
        [0.26, 0.28, 0.32, 1.0]
    }
}

/// Where the floor sits: under the scene, but snapped so that every line falls on a whole
/// multiple of the spacing in world space.
///
/// Centring it on the scene alone leaves the lines wherever the bounding box happens to fall, so
/// a cell is the right width but no line marks a round coordinate. Snapping costs up to half a
/// cell of offset from the scene, which nothing depends on.
fn grid_ground(center: Vec3, spacing: f32) -> Vec3 {
    let snap = |v: f32| match spacing > 0.0 {
        true => (v / spacing).round() * spacing,
        false => v,
    };
    Vec3::new(snap(center.x), snap(center.y), 0.0)
}

/// A grid on the XY plane through the origin, plus the positive axes over it, as a line list.
/// NIF is Z up, so XY is the ground. Returns the vertex data and the spacing it chose.
fn grid_lines(reach: f32) -> (Vec<f32>, f32, f32) {
    let spacing = grid_spacing(reach);
    let cells = ((reach / spacing).ceil() as i32).clamp(4, GRID_MAX_CELLS);
    let half = cells as f32 * spacing;

    let mut out = Vec::new();
    let mut line = |from: Vec3, to: Vec3, color: [f32; 4]| {
        for point in [from, to] {
            out.extend_from_slice(&[point.x, point.y, point.z]);
            out.extend_from_slice(&[0.0, 0.0, 0.0]);
            out.extend_from_slice(&color);
            out.extend_from_slice(&[0.0; 6]);
        }
    };

    for i in -cells..=cells {
        let at = i as f32 * spacing;
        let color = grid_tier(i, spacing);
        if i == 0 {
            // the positive halves of these two are the axes below. Drawing both would put two
            // lines in one place, which is what fights for depth once precision drops off.
            line(Vec3::new(0.0, -half, 0.0), Vec3::ZERO, color);
            line(Vec3::new(-half, 0.0, 0.0), Vec3::ZERO, color);
            continue;
        }
        line(Vec3::new(at, -half, 0.0), Vec3::new(at, half, 0.0), color);
        line(Vec3::new(-half, at, 0.0), Vec3::new(half, at, 0.0), color);
    }

    // only the positive half of each axis is drawn, so the direction is not a guess
    line(Vec3::ZERO, Vec3::X * half, [0.88, 0.30, 0.30, 1.0]);
    line(Vec3::ZERO, Vec3::Y * half, [0.35, 0.80, 0.40, 1.0]);
    line(Vec3::ZERO, Vec3::Z * half, [0.35, 0.60, 0.95, 1.0]);

    (out, spacing, half)
}

/// Maps every block under a NiLODNode to that node and the level it belongs to. A nested LOD
/// wins over an outer one, since the walk assigns as it descends.
fn lod_ancestry(nif: &Nif) -> HashMap<usize, (usize, usize)> {
    let mut out = HashMap::new();
    let mut seen = HashSet::new();
    let mut stack: Vec<(usize, Option<(usize, usize)>)> =
        nif.roots().map(|(index, _)| (index, None)).collect();

    while let Some((index, owner)) = stack.pop() {
        if !seen.insert(index) {
            continue;
        }
        if let Some(owner) = owner {
            out.insert(index, owner);
        }
        let Some(block) = nif.blocks.get(index) else {
            continue;
        };
        let children = block.child_refs().unwrap_or_default();
        let lod = matches!(block, Block::NiLODNode(_)).then_some(index);
        for (level, child) in children.iter().enumerate() {
            let Some(child) = child.index() else { continue };
            stack.push((child, lod.map(|node| (node, level)).or(owner)));
        }
    }
    out
}

fn build_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    target_format: wgpu::TextureFormat,
    state: DrawState,
    topology: wgpu::PrimitiveTopology,
    fragment_entry: &str,
) -> wgpu::RenderPipeline {
    // the framebuffer alpha is not read back, so only the colour factors follow the file
    let blend = state.blend.map(|(src, dst)| wgpu::BlendState {
        color: wgpu::BlendComponent {
            src_factor: src,
            dst_factor: dst,
            operation: wgpu::BlendOperation::Add,
        },
        alpha: wgpu::BlendComponent::OVER,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("nifty"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: (VERTEX_FLOATS * 4) as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x3, 1 => Float32x3, 2 => Float32x4,
                    3 => Float32x2, 4 => Float32x2, 5 => Float32x2
                ],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fragment_entry),
            targets: &[Some(wgpu::ColorTargetState {
                format: target_format,
                blend,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        // Measured, not assumed: 67 of 74 closed fixture shapes have positive signed
        // volume, i.e. triangles run counter-clockwise seen from outside. slipvillage
        // agrees: it reverses to (c, b, a) because Godot treats clockwise as front.
        primitive: wgpu::PrimitiveState {
            topology,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: state.cull,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: Some(state.depth_write),
            depth_compare: Some(state.depth),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: MSAA_SAMPLES,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}

/// `floats` is what the shader's matching struct holds. Declaring it makes a buffer that has
/// fallen behind the struct fail when the bind group is built, rather than once per draw call.
/// Compiles one shader against the contract. A shader out of a user's directory can fail to
/// compile and a panic is not an option, so the error comes back for the UI to report.
fn compile(
    device: &wgpu::Device,
    name: &str,
    pass: &shaders::Pass,
) -> Result<wgpu::ShaderModule, String> {
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(name),
        source: wgpu::ShaderSource::Wgsl(pass.module_source().into()),
    });
    match pollster::block_on(scope.pop()) {
        Some(error) => Err(error.to_string()),
        None => Ok(module),
    }
}

/// A texture and sampler pair per bound slot, in binding order.
fn slot_layout_entries() -> [wgpu::BindGroupLayoutEntry; GROUP_SLOTS * 2 + 2] {
    std::array::from_fn(|i| wgpu::BindGroupLayoutEntry {
        binding: i as u32,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: if i % 2 == 0 {
            wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                // the last pair is the cube map, which a reflection vector indexes directly
                view_dimension: match i == GROUP_SLOTS * 2 {
                    true => wgpu::TextureViewDimension::Cube,
                    false => wgpu::TextureViewDimension::D2,
                },
                multisampled: false,
            }
        } else {
            wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
        },
        count: None,
    })
}

fn uniform_entry(floats: u64) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: wgpu::BufferSize::new(floats * 4),
        },
        count: None,
    }
}

pub struct PreviewCall {
    pub scene: Arc<Scene>,
    pub wireframe: bool,
    pub grid: bool,
    pub cull: bool,
    pub selected: Option<usize>,
    /// Blended shapes sort against this.
    pub eye: Vec3,
    /// The camera's own axes in world space, which is what a particle quad is built on.
    pub right: Vec3,
    pub up: Vec3,
    pub lod_mode: LodMode,
    pub lod_distance: f32,
    pub frame: Arc<Frame>,
}

impl PreviewCall {
    /// Where the shape's centre is this frame. A billboard turns and an animated node moves, so
    /// the centre the scene was built with is not where it is being drawn.
    fn center(&self, mesh: &Mesh) -> Vec3 {
        self.frame.center_of(mesh)
    }

    /// One pass's textures as of this frame, which a flip controller may have swapped.
    /// What the shape's texturing property is flipping to this frame, across every slot.
    fn flip_state(&self, mesh: &Mesh) -> FlipState {
        mesh.texturing_block
            .and_then(|block| self.frame.flip.get(&block))
            .copied()
            .unwrap_or_default()
    }

    fn texture_of<'a>(
        &'a self,
        preview: &'a Preview,
        mesh: &'a Mesh,
        at: usize,
        pass: &'a MeshPass,
    ) -> &'a wgpu::BindGroup {
        let state = self.flip_state(mesh);
        if state.is_empty() {
            return &pass.texture;
        }
        // filled in `prepare`, which is the only place with a device. A combination that somehow
        // was not prepared falls back to the shape's own maps rather than dropping the draw.
        preview
            .flipped
            .get(&(mesh.shape_block, at, state))
            .unwrap_or(&pass.texture)
    }

    /// The same question for a particle system, which carries its level like any other shape.
    fn visible_particles(&self, mesh: &ParticleMesh) -> bool {
        !self.frame.hidden.contains(&mesh.block)
            && self.scene.shows(
                mesh.lod,
                self.lod_mode,
                self.lod_distance,
                self.eye,
                &self.frame.poses,
            )
    }

    fn visible(&self, mesh: &Mesh) -> bool {
        !self.frame.hidden.contains(&mesh.shape_block)
            && self.scene.shows(
                mesh.lod,
                self.lod_mode,
                self.lod_distance,
                self.eye,
                &self.frame.poses,
            )
    }
}

/// One pass of one shape. A shape's passes draw together and in order, since an outline is only
/// an outline while the surface that hides its near side follows it immediately.
fn draw_pass(
    render_pass: &mut wgpu::RenderPass<'static>,
    mesh: &Mesh,
    pipeline: &wgpu::RenderPipeline,
    texture: &wgpu::BindGroup,
) {
    render_pass.set_pipeline(pipeline);
    render_pass.set_bind_group(1, &mesh.bind_group, &[]);
    render_pass.set_bind_group(2, texture, &[]);
    render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
    render_pass.set_index_buffer(mesh.indices.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..mesh.count, 0, 0..1);
}

impl PreviewCall {
    /// The axes a camera facing quad spans. Taken from the camera rather than computed per
    /// particle, so every quad in a frame faces the same way.
    fn quad_axes(&self) -> (Vec3, Vec3) {
        (self.right, self.up)
    }
}

impl egui_wgpu::CallbackTrait for PreviewCall {
    /// The model matrix is the first 64 bytes of the uniform, so a pose rewrites only that
    /// and leaves the material behind it alone.
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        // A flipped shape binds a group per combination of slot frames, built the first time the
        // animation reaches that combination. Only combinations actually visited are built, which
        // is what keeps this off the cross product.
        if let Some(preview) = resources.get_mut::<Preview>() {
            for mesh in self.scene.meshes.iter().filter(|m| self.visible(m)) {
                let state = self.flip_state(mesh);
                if state.is_empty() {
                    continue;
                }
                for (at, pass) in mesh.passes.iter().enumerate() {
                    let key = (mesh.shape_block, at, state);
                    if preview.flipped.contains_key(&key) {
                        continue;
                    }
                    let group = pass.bindings.group(device, &preview.texture_layout, state);
                    preview.flipped.insert(key, group);
                }
            }
        }
        for mesh in &self.scene.meshes {
            // a skinned shape's vertices arrive in world space, so its node pose is not its
            // model matrix and writing one here would move it twice
            if let Some(model) = (!mesh.skinned)
                .then(|| self.frame.poses.get(&mesh.shape_block))
                .flatten()
            {
                queue.write_buffer(
                    &mesh.model_buffer,
                    0,
                    bytemuck::cast_slice(&drawn_at(self.scene.origin, *model).to_cols_array()),
                );
            }
            if let Some(alpha) = mesh
                .material_block
                .and_then(|block| self.frame.alpha.get(&block))
            {
                queue.write_buffer(&mesh.model_buffer, ALPHA_OFFSET, bytemuck::bytes_of(alpha));
            }
            if let Some(rows) = self.frame.uv.get(&mesh.shape_block) {
                queue.write_buffer(&mesh.model_buffer, UV_OFFSET, bytemuck::cast_slice(rows));
            }
            // a deform replaces the stored vertices outright, so the whole buffer goes back
            // rather than the positions being poked one at a time
            if let (Some(moved), Some(source)) = (
                self.frame.deformed.get(&mesh.shape_block),
                mesh.deform_source.as_ref(),
            ) {
                let mut attributes = source.clone();
                for (vertex, at) in moved.positions.iter().enumerate() {
                    let Some(slot) = attributes.get_mut(vertex * VERTEX_FLOATS..) else {
                        break;
                    };
                    let Some(position) = slot.get_mut(..3) else {
                        break;
                    };
                    position.copy_from_slice(&[at.x, at.y, at.z]);
                }
                // the normal lane follows the position lane, since bending or turning a lit
                // surface changes which way it faces
                for (vertex, at) in moved.normals.iter().flat_map(|n| n.iter()).enumerate() {
                    let Some(slot) = attributes.get_mut(vertex * VERTEX_FLOATS + 3..) else {
                        break;
                    };
                    let Some(normal) = slot.get_mut(..3) else {
                        break;
                    };
                    normal.copy_from_slice(&[at.x, at.y, at.z]);
                }
                queue.write_buffer(&mesh.vertices, 0, bytemuck::cast_slice(&attributes));
            }
            if let Some(params) = self.frame.params.get(&mesh.shape_block) {
                queue.write_buffer(
                    &mesh.model_buffer,
                    PARAMS_OFFSET,
                    bytemuck::cast_slice(params),
                );
            }
        }
        // the quads are generated here rather than stored, since a particle moves every frame
        for mesh in self
            .scene
            .particles
            .iter()
            .filter(|m| self.visible_particles(m))
        {
            let Some(particles) = self.frame.particles.get(&mesh.block) else {
                continue;
            };
            // the pose the frame walked, so an animated system draws where it now is rather
            // than where the scene was built. Picking walks at the same time, and the two have
            // to agree or the ray tests empty space.
            let model = particle_space(
                self.frame
                    .poses
                    .get(&mesh.block)
                    .copied()
                    .unwrap_or(mesh.model),
                mesh.world_space,
            );
            let scale = model.x_axis.truncate().length();
            let model = drawn_at(self.scene.origin, model);
            let (right, up) = self.quad_axes();
            let mut vertices: Vec<f32> = Vec::with_capacity(particles.len() * 4 * VERTEX_FLOATS);
            for particle in particles.iter().take(mesh.capacity) {
                let centre = model.transform_point3(Vec3::from(&particle.position));
                let colour = &particle.color;
                // the radius is in the system's space, like the position it sits at, and the
                // scale comes from the same matrix as the position so the two cannot disagree
                // a grow and fade modifier scales the radius rather than replacing it
                let half = particle.drawn_radius().max(0.0) * scale;
                // A rotation modifier turns the sprite in the plane facing the camera rather
                // than about an axis of its own, so the corners rotate and the axes do not.
                let (sin, cos) = particle.rotation.sin_cos();
                // a quad facing the camera, wound so the shared corners meet the index pattern
                for (corner, uv) in [
                    ((-1.0, -1.0), (0.0, 1.0)),
                    ((1.0, -1.0), (1.0, 1.0)),
                    ((1.0, 1.0), (1.0, 0.0)),
                    ((-1.0, 1.0), (0.0, 0.0)),
                ] {
                    let (x, y) = (
                        corner.0 * cos - corner.1 * sin,
                        corner.0 * sin + corner.1 * cos,
                    );
                    let at = centre + right * (x * half) + up * (y * half);
                    vertices.extend_from_slice(&[at.x, at.y, at.z]);
                    // the normal faces the camera, so anything lighting it sees the quad flat on
                    let normal = right.cross(up);
                    vertices.extend_from_slice(&[normal.x, normal.y, normal.z]);
                    vertices.extend_from_slice(&[colour.r, colour.g, colour.b, colour.a]);
                    vertices.extend_from_slice(&[uv.0, uv.1, uv.0, uv.1, uv.0, uv.1]);
                }
            }
            if !vertices.is_empty() {
                queue.write_buffer(&mesh.vertices, 0, bytemuck::cast_slice(&vertices));
            }
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let Some(preview) = resources.get::<Preview>() else {
            return;
        };

        render_pass.set_bind_group(0, &preview.camera_bind_group, &[]);

        if self.grid {
            let grid = &self.scene.grid;
            render_pass.set_pipeline(&preview.grid);
            render_pass.set_bind_group(1, &grid.model, &[]);
            render_pass.set_bind_group(2, &grid.texture, &[]);
            render_pass.set_vertex_buffer(0, grid.vertices.slice(..));
            render_pass.draw(0..grid.count, 0..1);
        }

        if self.wireframe {
            render_pass.set_pipeline(&preview.wire);
            for mesh in self.scene.meshes.iter().filter(|m| self.visible(m)) {
                render_pass.set_bind_group(1, &mesh.bind_group, &[]);
                render_pass.set_bind_group(
                    2,
                    self.texture_of(preview, mesh, 0, &mesh.passes[0]),
                    &[],
                );
                render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
                render_pass.set_index_buffer(mesh.edges.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.edge_count, 0, 0..1);
            }
        } else {
            // Everything that is not sorted draws first, in traversal order, and the sorted
            // shapes follow back to front. That is the engine's split: it queues the sortable
            // ones and draws the rest immediately as it meets them, so an unsorted blended
            // shape lands among the opaque geometry rather than after it.
            // A particle system is blended geometry like any other, so it sorts with the rest
            // rather than after it. Drawing it last put it behind anything blended that writes
            // depth, which is why particles inside a transparent shell vanished.
            let mut sorted: Vec<Sorted> = Vec::new();
            let mut immediate: Vec<Sorted> = Vec::new();
            for mesh in self
                .scene
                .particles
                .iter()
                .filter(|m| self.visible_particles(m))
            {
                let quads = self
                    .frame
                    .particles
                    .get(&mesh.block)
                    .map_or(0, |p| p.len().min(mesh.capacity));
                if quads == 0 {
                    continue;
                }
                if mesh.sorted {
                    sorted.push(Sorted::Particles(mesh, quads));
                } else {
                    immediate.push(Sorted::Particles(mesh, quads));
                }
            }
            for mesh in self.scene.meshes.iter().filter(|m| self.visible(m)) {
                if mesh.sorted {
                    sorted.push(Sorted::Shape(mesh));
                } else {
                    immediate.push(Sorted::Shape(mesh));
                }
            }
            let centre = |item: &Sorted| match item {
                Sorted::Shape(mesh) => self.center(mesh),
                Sorted::Particles(mesh, _) => particle_space(
                    self.frame
                        .poses
                        .get(&mesh.block)
                        .copied()
                        .unwrap_or(mesh.model),
                    mesh.world_space,
                )
                .transform_point3(Vec3::ZERO),
            };
            sorted.sort_by(|a, b| {
                centre(b)
                    .distance_squared(self.eye)
                    .total_cmp(&centre(a).distance_squared(self.eye))
            });
            for item in immediate.into_iter().chain(sorted) {
                match item {
                    Sorted::Shape(mesh) => {
                        for (at, pass) in mesh.passes.iter().enumerate() {
                            let pipeline = if self.cull {
                                &pass.pipeline
                            } else {
                                &pass.pipeline_unculled
                            };
                            draw_pass(
                                render_pass,
                                mesh,
                                pipeline,
                                self.texture_of(preview, mesh, at, pass),
                            );
                        }
                    }
                    Sorted::Particles(mesh, quads) => {
                        render_pass.set_pipeline(&mesh.pipeline);
                        render_pass.set_bind_group(1, &mesh.bind_group, &[]);
                        render_pass.set_bind_group(2, &mesh.texture, &[]);
                        render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
                        render_pass
                            .set_index_buffer(mesh.indices.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..(quads * 6) as u32, 0, 0..1);
                    }
                }
            }
        }

        for mesh in self
            .scene
            .particles
            .iter()
            .filter(|m| self.visible_particles(m))
        {
            let Some(particles) = self.frame.particles.get(&mesh.block) else {
                continue;
            };
            let quads = particles.len().min(mesh.capacity);
            if quads == 0 {
                continue;
            }
            // the solid pass draws these among the blended shapes, so only the outlines are
            // left here
            if !self.wireframe {
                continue;
            }
            render_pass.set_pipeline(&preview.wire);
            render_pass.set_bind_group(1, &mesh.bind_group, &[]);
            render_pass.set_bind_group(2, &mesh.texture, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
            render_pass.set_index_buffer(mesh.edges.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(quads * 8) as u32, 0, 0..1);
        }

        // the selected shape gets its wireframe drawn over everything, so it stays findable
        let Some(selected) = self.selected else {
            return;
        };
        render_pass.set_pipeline(&preview.highlight);
        for mesh in self.scene.meshes.iter().filter(|m| self.visible(m)) {
            if mesh.shape_block != selected && mesh.data_block != selected {
                continue;
            }
            render_pass.set_bind_group(1, &mesh.bind_group, &[]);
            render_pass.set_bind_group(2, self.texture_of(preview, mesh, 0, &mesh.passes[0]), &[]);
            render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
            render_pass.set_index_buffer(mesh.edges.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..mesh.edge_count, 0, 0..1);
        }
        // a selected particle system outlines its quads the same way, so picking one shows what
        // was picked rather than leaving the selection invisible
        for mesh in self
            .scene
            .particles
            .iter()
            .filter(|m| self.visible_particles(m))
        {
            if mesh.block != selected {
                continue;
            }
            let Some(particles) = self.frame.particles.get(&mesh.block) else {
                continue;
            };
            let quads = particles.len().min(mesh.capacity);
            if quads == 0 {
                continue;
            }
            render_pass.set_bind_group(1, &mesh.bind_group, &[]);
            render_pass.set_bind_group(2, &mesh.texture, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
            render_pass.set_index_buffer(mesh.edges.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(quads * 8) as u32, 0, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{grid_lines, shader_params, sorts, Light, Shaders};
    use nif::blocks::{Block, NiAlphaProperty, NiFloatExtraData, NiObjectNET, NiString};
    use nif::common::BlockRef;
    use nif::glam::Vec3;

    fn alpha_property(flags: u16) -> NiAlphaProperty {
        NiAlphaProperty {
            base: NiObjectNET {
                name: NiString::from("alpha"),
                extra_data_refs: Vec::new(),
                controller_ref: BlockRef::None,
            },
            flags,
            threshold: 0,
        }
    }

    /// The blend flag and the no sorter hint are different bits and mean different things: one
    /// decides how the shape is drawn, the other only where in the order. Reading the hint as
    /// "opaque" would drop the blending, and ignoring it puts the shape in the wrong pass.
    #[test]
    fn a_no_sorter_shape_blends_but_leaves_the_sort() {
        let blending = alpha_property(0x0001);
        let no_sorter = alpha_property(0x0001 | 0x2000);
        assert!(blending.alpha_blend() && !blending.no_sorter());
        assert!(no_sorter.alpha_blend() && no_sorter.no_sorter());

        assert!(sorts(true, Some(&blending)));
        assert!(!sorts(true, Some(&no_sorter)));
    }

    /// An opaque shape is never in the sorted pass, whatever the hint says, and a shape with no
    /// alpha property at all takes the sort when a shader forces blending on.
    #[test]
    fn only_a_blending_shape_can_be_sorted() {
        assert!(!sorts(false, Some(&alpha_property(0x0001))));
        assert!(!sorts(false, Some(&alpha_property(0x2000))));
        assert!(!sorts(false, None));
        assert!(sorts(true, None));
    }

    fn float_extra(name: &str, value: f32) -> Block {
        Block::NiFloatExtraData(NiFloatExtraData {
            name: NiString::from(name),
            value,
        })
    }

    /// A shape's own float beats the shader's declared default, which is what the engine does and
    /// is the difference between the snowglobe drawing opaque and drawing at the fifth of an
    /// opacity its shell asks for.
    #[test]
    fn a_shape_supplies_its_own_attributes_by_name() {
        let shaders = Shaders::default();
        let shader = shaders.get("OilyFilm").expect("OilyFilm is built in");
        assert_eq!(shader.param_names[..2], ["WarpAlpha", "Exponent"]);
        assert_eq!(shader.params[..2], [1.0, 48.0]);

        let blocks = vec![
            float_extra("Exponent", 51.5),
            float_extra("Unrelated", 9.0),
            float_extra("WarpAlpha", 0.2),
        ];
        let refs = [BlockRef::Index(0), BlockRef::Index(1), BlockRef::Index(2)];
        let bound = shader_params(&blocks, &refs, shader);

        assert_eq!(bound[0], 0.2);
        assert_eq!(bound[1], 51.5);
    }

    /// A lane no attribute names, and a shape carrying nothing, both keep the declared default.
    #[test]
    fn an_attribute_a_shape_does_not_carry_keeps_its_default() {
        let shaders = Shaders::default();
        let shader = shaders.get("OilyFilm").expect("built in");
        let blocks = vec![float_extra("Exponent", 8.0)];
        let bound = shader_params(&blocks, &[BlockRef::Index(0)], shader);

        assert_eq!(bound[0], shader.params[0]);
        assert_eq!(bound[1], 8.0);
        assert_eq!(bound[2..], shader.params[2..]);
    }

    /// A rotated box reaches furthest at a corner, and which corner depends on the rotation, so
    /// the sweep has to try all eight. Taking the two extremes alone would understate every
    /// animated shape that turns.
    #[test]
    fn a_box_offers_all_eight_of_its_corners() {
        let low = Vec3::new(-1.0, -2.0, -3.0);
        let high = Vec3::new(4.0, 5.0, 6.0);
        let corners = super::box_corners(low, high);

        assert!(corners.contains(&low));
        assert!(corners.contains(&high));
        // every one is distinct, and every one is a corner of the box
        for (at, corner) in corners.iter().enumerate() {
            assert!(corners[at + 1..].iter().all(|other| other != corner));
            for axis in 0..3 {
                assert!(corner[axis] == low[axis] || corner[axis] == high[axis]);
            }
        }
        // the furthest from any point is a corner, which is the property the sweep leans on
        let from = Vec3::new(10.0, -10.0, 0.0);
        let furthest = corners
            .iter()
            .map(|c| from.distance(*c))
            .fold(0.0f32, f32::max);
        assert!(furthest >= from.distance(low));
        assert!(furthest >= from.distance(high));
    }

    /// A `TexDesc` naming its own uv set is a fixed function notion, and a technique with its
    /// own vertex shader reads whatever `TEXCOORD` its source declares instead. `AGCar2` wires
    /// the decal to the first and both the mask and the window decal to the second, and a
    /// shape's own maps may name a set that disagrees.
    #[test]
    fn a_technique_can_pin_the_uv_set_a_slot_reads() {
        let shaders = Shaders::default();
        let pass = &shaders.get("AGCar2").expect("built in").passes[0];
        assert_eq!(pass.uv_set, [Some(0), Some(1), Some(1), None]);

        // a property whose maps all name set 0, which is the case the pin exists for
        let pinned = super::slot_uv_rows(&[], None, pass.slots, pass.uv_set, 0.0);
        let plain = super::slot_uv_rows(&[], None, pass.slots, [None; super::BOUND_SLOTS], 0.0);
        // the set rides in the first row's spare lane, so that is what changes
        assert_eq!([pinned[3], pinned[11], pinned[19]], [0.0, 1.0, 1.0]);
        assert_eq!([plain[3], plain[11], plain[19]], [0.0, 0.0, 0.0]);

        // and every technique built on stages leaves it to the map, as it always did
        for name in ["VCAlphaTextureBlender", "ToonShading", "ActionGameTree"] {
            let shader = shaders.get(name).unwrap_or_else(|| panic!("{name}"));
            assert!(
                shader.passes.iter().all(|p| p.uv_set == [None; 4]),
                "{name} should leave the uv set to its maps"
            );
        }
    }

    /// A technique can bind a texture to a shader map by index rather than by naming a file, and
    /// the shape moves it with the same `<attribute>Index` extra data. Falling back to a file
    /// there would resolve to nothing, so the technique's own index has to be the default.
    #[test]
    fn an_indexed_slot_falls_back_to_the_map_the_technique_names() {
        let shaders = Shaders::default();
        let shader = shaders.get("AGCar2").expect("built in");
        let declared = shader.passes[0].slots;
        assert!(matches!(
            declared[2],
            Some(super::shaders::Source::IndexedSlot {
                index: "MaskTex0Index",
                slot: nif::blocks::TextureSlot::Shader(1),
            })
        ));

        // a shape naming no map keeps the technique's own choice
        let bare = super::shader_slots(&[], &[], declared);
        assert_eq!(
            bare[2],
            Some(super::shaders::Source::Slot(
                nif::blocks::TextureSlot::Shader(1)
            ))
        );

        // and one that names a different map is followed
        let blocks = vec![Block::NiIntegerExtraData(nif::blocks::NiIntegerExtraData {
            name: NiString::from("MaskTex0Index"),
            value: 2,
        })];
        let moved = super::shader_slots(&blocks, &[BlockRef::Index(0)], declared);
        assert_eq!(
            moved[2],
            Some(super::shaders::Source::Slot(
                nif::blocks::TextureSlot::Shader(2)
            ))
        );
        // the base slot beside it is the file's own either way
        assert_eq!(moved[0], declared[0]);
    }

    /// A colour attribute takes a whole vec4 rather than a lane, and it binds by name the same
    /// way a float does. Leaving it at the declared white painted every car body white.
    #[test]
    fn a_colour_attribute_binds_by_name_and_falls_back_to_the_declared_one() {
        let shaders = Shaders::default();
        let shader = shaders.get("ActionGameCartoonFX").expect("built in");
        assert_eq!(shader.color_name, "MaterialColor");
        assert_eq!(shader.color, [1.0; 4]);

        let blocks = vec![
            Block::NiColorExtraData(nif::blocks::NiColorExtraData {
                name: NiString::from("Unrelated"),
                data: nif::common::Color4 {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            }),
            Block::NiColorExtraData(nif::blocks::NiColorExtraData {
                name: NiString::from("MaterialColor"),
                data: nif::common::Color4 {
                    r: 0.1,
                    g: 0.3,
                    b: 0.6,
                    a: 1.0,
                },
            }),
        ];
        let refs = [BlockRef::Index(0), BlockRef::Index(1)];
        assert_eq!(
            super::shader_color(&blocks, &refs, shader),
            [0.1, 0.3, 0.6, 1.0]
        );

        // a shape carrying none keeps what the technique declares
        assert_eq!(super::shader_color(&[], &[], shader), shader.color);

        // and a shader declaring no colour is left alone whatever the shape carries
        let plain = shaders.get("ActionGameTree").expect("built in");
        assert_eq!(plain.color_name, "");
        assert_eq!(super::shader_color(&blocks, &refs, plain), plain.color);
    }

    /// A shader declares its toon ramp with a file name, and nearly every shape redirects it to
    /// one of its own maps. Reading the declared name instead resolves to nothing without a
    /// texture root, the slot stands in white, and the whole ramp flattens out.
    #[test]
    fn a_texture_attribute_reads_the_map_the_shape_names() {
        let shaders = Shaders::default();
        let shader = shaders.get("ToonShading").expect("built in");
        let declared = shader.passes[0].slots;
        assert!(matches!(
            declared[1],
            Some(super::shaders::Source::Attribute {
                index: "ToonRampIndex",
                ..
            })
        ));

        let blocks = vec![Block::NiIntegerExtraData(nif::blocks::NiIntegerExtraData {
            name: NiString::from("ToonRampIndex"),
            value: 2,
        })];
        let bound = super::shader_slots(&blocks, &[BlockRef::Index(0)], declared);
        assert_eq!(
            bound[1],
            Some(super::shaders::Source::Slot(
                nif::blocks::TextureSlot::Shader(2)
            ))
        );
        // the base slot beside it is the file's own and is left alone
        assert_eq!(bound[0], declared[0]);

        // a shape naming no map falls back to the file the shader declares
        let bare = super::shader_slots(&[], &[], declared);
        assert_eq!(bare[1], Some(super::shaders::Source::Named("ToonRamp.bmp")));
    }

    /// The outline test reduces to `N dot V <= outlineThickness`, and reducing it depends on a
    /// declared constant of a half. The vertex program's own comment lists that constant as 1,
    /// where the test can never fire and the shader has no outline at all, so the two disagree
    /// and the declared value is what runs.
    #[test]
    fn outlining_per_pixel_is_a_separate_technique_from_the_hull() {
        let shaders = Shaders::default();
        let outline = shaders.get("ToonShadingWithOutline").expect("built in");
        // one pass, unlike the cartoon pair, and it moves no vertices
        assert_eq!(outline.passes.len(), 1);
        assert!(outline.passes[0].vertex.is_none());
        assert_eq!(outline.param_names[0], "outlineThickness");
        assert_eq!(outline.params[0], 0.1);

        // it reads the same ramp attribute the rest of the family does
        assert!(matches!(
            outline.passes[0].slots[1],
            Some(super::shaders::Source::Attribute {
                index: "ToonRampIndex",
                ..
            })
        ));
        // and leaves cull and depth to the file, where the hull pins both
        assert_eq!(outline.passes[0].state.cull, None);
        let hull = &shaders.get("ActionGameCartoon").expect("built in").passes[0];
        assert!(hull.state.cull.is_some());
    }

    /// `Reflection` is the exponent the specular band raises its cosine to, so a default left in
    /// place where the file overrides it is the difference between a pinpoint and a sweep.
    #[test]
    fn the_specular_band_reads_its_exponent_from_the_shape() {
        let shaders = Shaders::default();
        let shader = shaders.get("ActionSpecularBand").expect("built in");
        assert_eq!(shader.param_names[0], "Reflection");
        assert_eq!(shader.params[0], 100.0);

        let blocks = vec![float_extra("Reflection", 10.0)];
        let bound = shader_params(&blocks, &[BlockRef::Index(0)], shader);
        assert_eq!(bound[0], 10.0);
    }

    /// The outline family draws twice, and the order and the culling are the whole technique: an
    /// expanded shell with its front faces dropped, then the surface over it. Getting either
    /// wrong turns the shell from a rim into a coat of paint over the whole object.
    #[test]
    fn the_outlining_techniques_draw_a_culled_hull_first() {
        let shaders = Shaders::default();
        for name in ["ActionGameCartoon", "JiCartoon"] {
            let shader = shaders
                .get(name)
                .unwrap_or_else(|| panic!("{name} is built in"));
            assert_eq!(shader.passes.len(), 2, "{name}");

            let hull = &shader.passes[0];
            assert_eq!(
                hull.state.cull,
                Some(Some(eframe::egui_wgpu::wgpu::Face::Front)),
                "{name} hull has to drop its near side"
            );
            assert!(
                hull.vertex.is_some(),
                "{name} hull has to move its vertices"
            );
            assert!(
                hull.slots.iter().all(Option::is_none),
                "{name} hull samples nothing"
            );

            let surface = &shader.passes[1];
            assert!(surface.vertex.is_none(), "{name} surface stays put");
            assert_eq!(
                surface.state.cull,
                Some(Some(eframe::egui_wgpu::wgpu::Face::Back)),
                "{name} surface culls the way any solid does"
            );
            // the thickness is the shape's to set, and the source's default stands in for it
            assert_eq!(shader.param_names[0], "outlineThickness");
        }
    }

    /// Every shader is one module per pass, and a pass that moves no vertex still has to get a
    /// `displace` to call, or the contract fails to compile for the six shaders that have none.
    #[test]
    fn every_pass_declares_exactly_one_displacement() {
        let shaders = Shaders::default();
        let all = shaders
            .names()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>();
        assert!(all.contains(&"ActionGameCartoon".to_string()));

        for name in all {
            let shader = shaders.get(&name).expect("listed");
            for (index, pass) in shader.passes.iter().enumerate() {
                let source = pass.module_source();
                assert_eq!(
                    source.matches("fn displace(").count(),
                    1,
                    "{name} pass {index}"
                );
                assert!(source.contains("fn vs_main("), "{name} pass {index}");
                assert!(source.contains("fn fs_main("), "{name} pass {index}");
            }
        }
    }

    /// The light replaced constants baked into the fragment shader. If the defaults drift, every
    /// file in the viewer changes appearance, so they are pinned to what those constants were.
    #[test]
    fn the_default_light_reproduces_the_shading_it_replaced() {
        let light = Light::default();

        assert_eq!(light.ambient, Vec3::splat(0.2));
        assert_eq!(light.diffuse, Vec3::splat(0.65));
        assert_eq!(light.fill, 0.3);

        // the shader negates the direction to face the light, so the stored value is the
        // negation of the old key. Getting this backwards lights the far side of everything.
        let to_light = -light.direction;
        assert!(
            to_light.abs_diff_eq(Vec3::new(0.3, 0.45, 0.85).normalize(), 1e-6),
            "direction of travel points {:?}",
            light.direction
        );
        assert!(
            to_light.z > 0.0,
            "NIF is Z up, so the key light comes from above"
        );
    }

    /// The scene is drawn moved to sit near zero, and the shader compares a light's position
    /// against a surface in that same moved space. A position uploaded in the file's own space
    /// is off by the whole origin, which is thousands of units for a model parked far out.
    /// A direction is not a position and must not move.
    #[test]
    fn a_light_position_reaches_the_shader_in_the_space_the_scene_is_drawn_in() {
        use nif::light::{Falloff, Lit};
        let origin = Vec3::new(9000.0, -4000.0, 25.0);
        let at = Vec3::new(9010.0, -3995.0, 30.0);
        let aim = Vec3::new(0.0, 0.0, -1.0);
        let lit = |falloff| Lit {
            falloff,
            position: at,
            direction: aim,
            ambient: Vec3::ZERO,
            diffuse: Vec3::ONE,
            specular: Vec3::ZERO,
            attenuation: Vec3::X,
            cos_cutoff: -1.0,
            exponent: 0.0,
        };

        let light = Light::default();
        for falloff in [Falloff::Point, Falloff::Spot] {
            let filled = super::camera_uniform(
                nif::glam::Mat4::IDENTITY,
                Vec3::ZERO,
                true,
                true,
                &light,
                &[lit(falloff)],
                origin,
            );
            let placed = Vec3::new(filled[44], filled[45], filled[46]);
            assert_eq!(placed, at - origin, "{falloff:?} was not moved with the scene");
        }

        // a directional light carries a direction in that slot, and moving it would tilt it
        let filled = super::camera_uniform(
            nif::glam::Mat4::IDENTITY,
            Vec3::ZERO,
            true,
            true,
            &light,
            &[lit(Falloff::Directional)],
            origin,
        );
        assert_eq!(Vec3::new(filled[44], filled[45], filled[46]), aim);
    }

    #[test]
    fn the_camera_uniform_carries_the_light_at_the_end() {
        let light = Light::default();
        let filled =
            super::camera_uniform(nif::glam::Mat4::IDENTITY, Vec3::ZERO, true, true, &light, &[], Vec3::ZERO);

        assert_eq!(filled.len(), super::CAMERA_FLOATS as usize);
        assert_eq!(&filled[24..40], &light.uniform());
    }

    /// Every vertex is position, colour and two uv sets, and the axes are the last three lines.
    const STRIDE: usize = super::VERTEX_FLOATS;

    /// One unit per cell wherever the scene allows it, and a power of ten either side of that
    /// when it does not. A scene of a few hundred units is the common case and it is the one
    /// that has to land on 1.
    #[test]
    fn a_cell_is_one_unit_unless_the_scene_is_too_wide_for_that() {
        for (reach, expected) in [
            (10.0, 1.0),
            (50.0, 1.0),
            (192.0, 1.0),
            (1000.0, 1.0),
            // past the cell budget it coarsens by powers of ten rather than fitting the scene
            (2000.0, 10.0),
            (20000.0, 100.0),
            // and a model smaller than a few cells across subdivides instead
            (1.0, 0.1),
            (0.05, 0.01),
        ] {
            let (_, spacing, _) = grid_lines(reach);
            assert_eq!(spacing, expected, "reach {reach}");
        }
    }

    /// The brightest lines have to land on round distances in the file's own units, or the
    /// floor cannot be read as a ruler. Keyed off the line index alone, as it was, the bright
    /// line lands on a different distance in every file.
    #[test]
    fn the_emphasised_lines_fall_on_whole_units() {
        // a cell per unit: every line is a unit line, every tenth is a ten
        let unit = super::grid_tier(1, 1.0);
        let ten = super::grid_tier(10, 1.0);
        assert_ne!(unit, ten);
        assert_eq!(super::grid_tier(3, 1.0), unit);

        // a tenth of a unit per cell: the unit line is every tenth line, not every line
        let dim = super::grid_tier(3, 0.1);
        assert_ne!(dim, unit);
        assert_eq!(super::grid_tier(10, 0.1), unit, "1.0 units in");
        assert_eq!(super::grid_tier(20, 0.1), unit, "2.0 units in");
        assert_eq!(super::grid_tier(100, 0.1), ten, "10.0 units in");

        // a hundredth, and the same distances still carry the same weight
        assert_eq!(super::grid_tier(100, 0.01), unit, "1.0 units in");
        assert_eq!(super::grid_tier(1000, 0.01), ten, "10.0 units in");
        assert_eq!(super::grid_tier(50, 0.01), dim, "half a unit in");

        // and the unit line is brighter than the ones between it
        assert!(unit[0] > dim[0] && ten[0] > unit[0]);
    }

    /// The floor still covers the scene at every size. Coarsening is what keeps the cell count
    /// inside its budget, so the budget itself never has to cut the grid short.
    #[test]
    fn the_floor_reaches_at_least_as_far_as_the_scene() {
        for reach in [0.05, 1.0, 7.5, 240.0, 6000.0] {
            let (lines, _, _) = grid_lines(reach);
            let furthest = lines
                .chunks(STRIDE)
                .map(|v| v[0].abs().max(v[1].abs()))
                .fold(0.0, f32::max);
            // a cell edge is a product of floats, so it can land an ulp short of the reach
            assert!(
                furthest >= reach * (1.0 - 1e-6),
                "reach {reach} got {furthest}"
            );
        }
    }

    /// The far plane is computed from `Grid::half`, so if that understates the geometry the
    /// floor gets clipped again, which is the bug it was added to fix.
    #[test]
    fn the_reported_extent_bounds_every_grid_vertex() {
        for reach in [0.0, 0.05, 1.0, 7.5, 240.0, 6000.0] {
            let (lines, _, half) = grid_lines(reach);
            let furthest = lines
                .chunks(STRIDE)
                .map(|v| v[0].abs().max(v[1].abs()))
                .fold(0.0, f32::max);
            assert!(
                half >= furthest,
                "reach {reach} reported {half} but a vertex sits at {furthest}"
            );
        }
    }

    /// A scene centre is an arbitrary float, so the floor has to be snapped or its lines fall
    /// on nothing in particular. The width of a cell is only half of being able to read a
    /// coordinate off it.
    #[test]
    fn every_grid_line_lands_on_a_whole_multiple_of_the_spacing() {
        for (centre, radius) in [
            (Vec3::new(123.456, -7.3, 0.0), 200.0f32),
            (Vec3::new(-1893.02, 44.87, 0.0), 900.0),
            (Vec3::new(0.4999, 0.5001, 0.0), 8.0),
            (Vec3::new(-0.03, 0.07, 0.0), 0.4),
        ] {
            let spacing = super::grid_spacing(radius);
            let ground = super::grid_ground(centre, spacing);
            for axis in [ground.x, ground.y] {
                let steps = axis / spacing;
                assert!(
                    (steps - steps.round()).abs() < 1e-3,
                    "centre {centre:?} at spacing {spacing} left a line at {axis}"
                );
            }
            // and the floor still sits under the scene rather than being dragged to the origin
            assert!((ground.x - centre.x).abs() <= spacing * 0.5 + 1e-4);
            assert!((ground.y - centre.y).abs() <= spacing * 0.5 + 1e-4);
            assert_eq!(ground.z, 0.0, "the floor stays at the world's own z");
        }
    }

    #[test]
    fn a_scene_at_the_origin_still_gets_a_grid() {
        let (lines, spacing, _) = grid_lines(0.0);
        assert!(spacing > 0.0);
        assert!(!lines.is_empty());
        assert!(lines.iter().all(|f| f.is_finite()));
    }

    #[test]
    fn only_the_positive_axes_are_drawn() {
        let (lines, _, _) = grid_lines(10.0);
        // three axis lines, two vertices each, at the end of the buffer
        let axes = &lines[lines.len() - 6 * STRIDE..];
        for (i, axis) in axes.chunks(2 * STRIDE).enumerate() {
            assert_eq!(
                &axis[..3],
                &[0.0, 0.0, 0.0],
                "axis {i} starts at the origin"
            );
            assert!(axis[STRIDE + i] > 0.0, "axis {i} runs positive");
        }
    }
}
