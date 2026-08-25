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

/// The shader's `Model` and `Camera` structs, in floats. Every buffer bound as one has to be
/// this long, the grid's included.
const MODEL_FLOATS: u64 = 72;
const CAMERA_FLOATS: u64 = 40;

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
) -> [f32; CAMERA_FLOATS as usize] {
    let mut out = [0.0; CAMERA_FLOATS as usize];
    out[..16].copy_from_slice(&view_proj.to_cols_array());
    out[16..19].copy_from_slice(&eye.to_array());
    out[20] = f32::from(colors);
    out[21] = f32::from(textures);
    out[24..40].copy_from_slice(&light.uniform());
    out
}

/// Lives in `callback_resources`, which is all `paint` can reach.
pub struct Preview {
    wire: wgpu::RenderPipeline,
    highlight: wgpu::RenderPipeline,
    grid: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,
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
    /// Every source a flip controller can swap into the base slot, by its own block. Empty
    /// unless one drives this shape.
    flip_frames: HashMap<usize, wgpu::BindGroup>,
    /// Which texture slots this shape's three bindings hold, which its shader decides.
    pub bound: [Option<shaders::Source>; BOUND_SLOTS],
    pub radius: f32,
    /// The NiLODNode this shape sits under, and which of its levels, if any.
    lod: Option<(usize, usize)>,
    /// Whether this goes in the back to front pass. A shape that blends but whose alpha property
    /// asks for no sorter is drawn where the traversal reaches it instead, among the opaque ones.
    sorted: bool,
    pipeline: wgpu::RenderPipeline,
    pipeline_unculled: wgpu::RenderPipeline,
    texture: wgpu::BindGroup,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    count: u32,
    edges: wgpu::Buffer,
    edge_count: u32,
    bind_group: wgpu::BindGroup,
    /// Rewritten when an animated pose moves the shape. The model matrix is its first 16 floats.
    model_buffer: wgpu::Buffer,
}

/// A particle system's drawing side. The geometry is generated per frame rather than stored, so
/// the buffers are sized once for the system's capacity and rewritten as the simulation moves.
pub struct ParticleMesh {
    /// The NiParticleSystem this draws, which is what the frame's particles are keyed by.
    pub block: usize,
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
}

/// Something drawn in the back to front pass, which sorts across both kinds.
enum Sorted<'a> {
    Shape(&'a Mesh),
    /// The system and how many of its quads are alive this frame.
    Particles(&'a ParticleMesh, usize),
}

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub particles: Vec<ParticleMesh>,
    pub center: Vec3,
    pub radius: f32,
    pub lods: HashMap<usize, Lod>,
    pub grid: Grid,
}

/// The ground plane and axes. Sized to the file when the scene is built, since one spacing
/// cannot serve a 2 unit gauge and a 6,000 unit parking lot.
pub struct Grid {
    vertices: wgpu::Buffer,
    count: u32,
    model: wgpu::BindGroup,
    texture: wgpu::BindGroup,
    pub spacing: f32,
    /// How far the floor reaches from the origin. The far plane has to clear it, or the grid is
    /// cut off rather than merely small when the camera closes in on something.
    pub half: f32,
}

/// A NiLODNode's switching distances, and the point they are measured from.
pub struct Lod {
    pub center: Vec3,
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
    /// The source a flip controller has swapped into the base slot, by texturing property block.
    pub flip: HashMap<usize, usize>,
    /// Where each particle system's particles are, by the system's own block. Simulated by the
    /// caller, since the state has to outlive a scene rebuild.
    pub particles: HashMap<usize, Vec<nif::psys::Particle>>,
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
    pub fn shows(&self, mesh: &Mesh, mode: LodMode, distance: f32, eye: Vec3) -> bool {
        let Some((node, level)) = mesh.lod else {
            return true;
        };
        let Some(lod) = self.lods.get(&node) else {
            return true;
        };
        match mode {
            LodMode::All => true,
            LodMode::Auto => level == lod.level_at(lod.center.distance(eye)),
            LodMode::Manual => level == lod.level_at(distance),
        }
    }

    /// The shape blocks currently drawn, which is what picking may select.
    pub fn visible_shapes(&self, mode: LodMode, distance: f32, eye: Vec3) -> HashSet<usize> {
        self.meshes
            .iter()
            .filter(|mesh| self.shows(mesh, mode, distance, eye))
            .map(|mesh| mesh.shape_block)
            .collect()
    }
}

impl Lod {
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
        let shader = compile(device, Shaders::default().fixed())
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
        slots: [(&wgpu::TextureView, &wgpu::Sampler); BOUND_SLOTS],
    ) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = slots
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
        self.render_state
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("nifty textures"),
                layout: &self.texture_layout,
                entries: &entries,
            })
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

    /// A draw per shape, each carrying its own transform, not one merged mesh.
    /// Buffers for one particle system, sized once for its capacity. The vertices are rewritten
    /// every frame from the simulation, so the contents here are only a starting size.
    fn particle_mesh(
        &self,
        nif: &Nif,
        visit: &nif::walk::Visit<'_>,
        geometry: &NiGeometry,
        library: &TextureLibrary,
        module: &wgpu::ShaderModule,
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
        uniform[32..64].copy_from_slice(&slot_uv_rows(&nif.blocks, None, DEFAULT_SLOTS, 0.0));

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
            model: Mat4::from(&visit.transform),
            capacity,
            sorted,
            reach: reach * visit.transform.scale.abs(),
            pipeline: self.pipeline(state, module),
            texture: self.slot_group(std::array::from_fn(|_| (&view, &sampler))),
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
    ) -> (Scene, Vec<String>) {
        let lod_of = lod_ancestry(nif);
        let device = &self.render_state.device;
        // one per Absent variant, since what an unread slot stands in with depends on how the
        // slot combines: white where it multiplies, black where it adds, half where it doubles
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
        let mut pipelines: HashMap<(String, DrawState), wgpu::RenderPipeline> = HashMap::new();
        let mut modules: HashMap<String, Result<wgpu::ShaderModule, String>> = HashMap::new();
        let fixed_module = compile(device, shaders.fixed())
            .expect("the built in fixed function shader has to compile");
        let mut unhandled: Vec<String> = Vec::new();
        let mut meshes = Vec::new();
        let mut particles = Vec::new();
        let mut lods: HashMap<usize, Lod> = HashMap::new();
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

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
                let mesh = self.particle_mesh(nif, &visit, &psys.base, library, &fixed_module);
                if let Some(mesh) = mesh {
                    let centre = mesh.model.transform_point3(Vec3::ZERO);
                    min = min.min(centre - Vec3::splat(mesh.reach));
                    max = max.max(centre + Vec3::splat(mesh.reach));
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

            let model = Mat4::from(&visit.transform);
            let colors = data.vertex_colors.as_ref();
            let normals = data.normals.as_ref();
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
            let blend = shader.state.blend.unwrap_or(from_file);
            let tested = alpha.filter(|_| shader.state.alpha_test != Some(false));
            // the shader implements TestGreater only. TestAlways never discards.
            let (alpha_test, alpha_threshold) = match tested {
                Some(a) if a.alpha_test() && a.test_func() != TestFunction::TestAlways => {
                    (1.0, f32::from(a.threshold) / 255.0)
                }
                _ => (0.0, 0.0),
            };
            let state = DrawState {
                cull: cull_of(stencil.map(|p| &p.draw_mode)),
                depth_write: shader
                    .state
                    .depth_write
                    .unwrap_or_else(|| zbuffer.is_none_or(|z| z.depth_write())),
                depth: depth_of(zbuffer),
                blend,
            };
            let module = modules
                .entry(shader.name.clone())
                .or_insert_with(|| compile(device, shader));
            // a shader that failed to compile falls back rather than taking the viewer down
            let module = match module {
                Ok(module) => module,
                Err(_) => &fixed_module,
            };
            let pipeline = pipelines
                .entry((shader.name.clone(), state))
                .or_insert_with(|| self.pipeline(state, module))
                .clone();
            let unculled = DrawState {
                cull: None,
                ..state
            };
            let pipeline_unculled = pipelines
                .entry((shader.name.clone(), unculled))
                .or_insert_with(|| self.pipeline(unculled, module))
                .clone();

            // one upload per source texture, not per shape that uses it
            let texture_key = texturing.and_then(|p| p.base_texture.as_ref()?.source_ref.index());
            let property = texturing;
            let texturing_block = in_force.texturing.index();

            let bound = shader.slots;
            // a slot the shape does not use has to change nothing, and what that means depends
            // on how the slot is combined
            let mut views: [wgpu::TextureView; BOUND_SLOTS] =
                std::array::from_fn(|position| neutral[shader.absent[position] as usize].clone());
            let default_sampling = shaders::Sampling {
                address: (wgpu::AddressMode::Repeat, wgpu::AddressMode::Repeat),
                filter: wgpu::FilterMode::Linear,
            };
            let mut samplers: [wgpu::Sampler; BOUND_SLOTS] = std::array::from_fn(|position| {
                self.sampler(shader.address[position].unwrap_or(default_sampling))
            });

            for (position, slot) in bound.iter().enumerate() {
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
                    shaders::Source::Slot(slot) => {
                        let Some(desc) = property.and_then(|p| p.texture(*slot)) else {
                            continue;
                        };
                        if let Some(key) = desc.source_ref.index() {
                            views[position] = cache
                                .entry(key)
                                .or_insert_with(|| {
                                    self.source_texture(nif, desc.source_ref, library)
                                        .unwrap_or_else(|| missing.clone())
                                })
                                .clone();
                        }
                        // the shader's own sampler state beats the map's clamp mode
                        samplers[position] =
                            self.sampler(shader.address[position].unwrap_or(shaders::Sampling {
                                address: address_of(&desc.clamp_mode),
                                filter: wgpu::FilterMode::Linear,
                            }));
                    }
                }
            }
            let texture = self.slot_group(std::array::from_fn(|i| (&views[i], &samplers[i])));
            // every frame a flip controller can reach, uploaded once each and shared by block
            let mut flip_frames = HashMap::new();
            let base_slot_flips =
                bound.first() == Some(&Some(shaders::Source::Slot(TextureSlot::Base)));
            for block in nif.blocks.iter().filter(|_| base_slot_flips) {
                let Block::NiFlipController(flip) = block else {
                    continue;
                };
                if flip.target_ref.index() != texturing_block
                    || TextureSlot::from_flip_index(flip.texture_slot) != Some(TextureSlot::Base)
                {
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
                    // the other two slots keep whatever the shape itself carries
                    let group = self.slot_group(std::array::from_fn(|i| {
                        // only the first binding is swapped; the rest stay as the shape's own
                        let view = if i == 0 { &view } else { &views[i] };
                        (view, &samplers[i])
                    }));
                    flip_frames.insert(index, group);
                }
            }

            let mut model_uniform = [0f32; MODEL_FLOATS as usize];
            model_uniform[..16].copy_from_slice(&model.to_cols_array());
            model_uniform[16..20].copy_from_slice(&diffuse);
            model_uniform[20..24].copy_from_slice(&emissive);
            model_uniform[24..28].copy_from_slice(&[
                emissive_from_vertex,
                diffuse_from_vertex,
                lighting,
                replace * f32::from(texture_key.is_some()),
            ]);
            model_uniform[28..32].copy_from_slice(&[alpha_threshold, alpha_test, 0.0, 0.0]);
            model_uniform[32..64].copy_from_slice(&slot_uv_rows(&nif.blocks, property, bound, 0.0));
            model_uniform[64..68].copy_from_slice(&shader_params(
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

            meshes.push(Mesh {
                shape_block: visit.index,
                lod: lod_of.get(&visit.index).copied(),
                sorted: sorts(blend.is_some(), alpha),
                pipeline,
                pipeline_unculled,
                center: (shape_min + shape_max) * 0.5,
                local_center: (local_min + local_max) * 0.5,
                material_block: material_index,
                texturing_block,
                flip_frames,
                bound,
                radius: ((shape_max - shape_min).length() * 0.5).max(0.001),
                data_block: geometry.data_ref.index().unwrap_or(usize::MAX),
                texture,
                vertices: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("nifty vertices"),
                    contents: bytemuck::cast_slice(&attributes),
                    usage: wgpu::BufferUsages::VERTEX,
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

        let samplers_for_grid = self.sampler(shaders::Sampling {
            address: (wgpu::AddressMode::Repeat, wgpu::AddressMode::Repeat),
            filter: wgpu::FilterMode::Linear,
        });
        // the floor has to reach the geometry as well as the origin, which a chunk sitting
        // far out is nowhere near
        let (lines, spacing, half) = grid_lines(center.length() + radius);
        let mut identity = [0f32; MODEL_FLOATS as usize];
        identity[..16].copy_from_slice(&Mat4::IDENTITY.to_cols_array());
        identity[32..64].copy_from_slice(&slot_uv_rows(&[], None, DEFAULT_SLOTS, 0.0));
        let grid = Grid {
            half,
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
            texture: self.slot_group(std::array::from_fn(|_| (&white, &samplers_for_grid))),
            spacing,
        };

        unhandled.sort();
        unhandled.dedup();
        (
            Scene {
                meshes,
                particles,
                center,
                radius: radius.max(0.001),
                lods,
                grid,
            },
            unhandled,
        )
    }
}

/// A drawable shape's geometry, its vertex data and its triangles, whether the file stores
/// those as a triangle list or as strips.
pub(crate) fn geometry_of<'a>(
    nif: &'a Nif,
    block: &'a Block,
) -> Option<(&'a NiGeometry, &'a NiGeometryData, Vec<Triangle>)> {
    let (geometry, data) = match block {
        Block::NiTriShape(shape) => (&shape.base, shape.data_ref.get(&nif.blocks)?),
        Block::NiTriStrips(strips) => (&strips.base, strips.data_ref.get(&nif.blocks)?),
        _ => return None,
    };
    match data {
        Block::NiTriShapeData(data) => Some((geometry, &data.base.base, data.triangles.clone()?)),
        Block::NiTriShapeDynamicData(data) => {
            Some((geometry, &data.base.base.base, data.base.triangles.clone()?))
        }
        Block::NiTriStripsData(data) => {
            Some((geometry, &data.base.base, data.triangles().collect()))
        }
        _ => None,
    }
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
    time: f32,
) -> [f32; BOUND_SLOTS * 8] {
    let mut out = [0.0; BOUND_SLOTS * 8];
    for (position, slot) in bound.iter().enumerate() {
        // a texture the shader names itself carries no TexDesc, so it has no transform either
        let Some(shaders::Source::Slot(slot)) = *slot else {
            out[position * 8..position * 8 + 8].copy_from_slice(&uv_rows(None, 0));
            continue;
        };
        let desc = property.and_then(|p| p.texture(slot));
        let transform =
            property.and_then(|p| nif::anim::texture_transform_at(blocks, p, slot, time));
        let rows = uv_rows(transform, desc.map_or(0, |d| d.uv_set));
        out[position * 8..(position + 1) * 8].copy_from_slice(&rows);
    }
    out
}

/// A grid on the XY plane through the origin, plus the positive axes over it, as a line list.
/// NIF is Z up, so XY is the ground. Returns the vertex data and the spacing it chose.
///
/// `reach` is how far the scene gets from the origin, and the spacing is the power of ten that
/// puts roughly ten cells between the two, so the numbers on it stay round.
fn grid_lines(reach: f32) -> (Vec<f32>, f32, f32) {
    let spacing = 10f32.powf((reach.max(1e-3) / 10.0).log10().round());
    let cells = ((reach / spacing).ceil() as i32).clamp(4, 40);
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
        // every tenth line is brighter, so the scale reads without counting cells
        let color = if i % 10 == 0 {
            [0.46, 0.48, 0.54, 1.0]
        } else {
            [0.32, 0.34, 0.38, 1.0]
        };
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
fn compile(device: &wgpu::Device, shader: &Shader) -> Result<wgpu::ShaderModule, String> {
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(&shader.name),
        source: wgpu::ShaderSource::Wgsl(shader.module_source().into()),
    });
    match pollster::block_on(scope.pop()) {
        Some(error) => Err(error.to_string()),
        None => Ok(module),
    }
}

/// A texture and sampler pair per bound slot, in binding order.
fn slot_layout_entries() -> [wgpu::BindGroupLayoutEntry; BOUND_SLOTS * 2] {
    std::array::from_fn(|i| wgpu::BindGroupLayoutEntry {
        binding: i as u32,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: if i % 2 == 0 {
            wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
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
        match self.frame.poses.get(&mesh.shape_block) {
            Some(model) => model.transform_point3(mesh.local_center),
            None => mesh.center,
        }
    }

    /// The base texture as of this frame, which a flip controller may have swapped.
    fn texture_of<'a>(&'a self, mesh: &'a Mesh) -> &'a wgpu::BindGroup {
        mesh.texturing_block
            .and_then(|block| self.frame.flip.get(&block))
            .and_then(|source| mesh.flip_frames.get(source))
            .unwrap_or(&mesh.texture)
    }

    fn visible(&self, mesh: &Mesh) -> bool {
        !self.frame.hidden.contains(&mesh.shape_block)
            && self
                .scene
                .shows(mesh, self.lod_mode, self.lod_distance, self.eye)
    }
}

fn draw_mesh(
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
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        _resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        for mesh in &self.scene.meshes {
            if let Some(model) = self.frame.poses.get(&mesh.shape_block) {
                queue.write_buffer(
                    &mesh.model_buffer,
                    0,
                    bytemuck::cast_slice(&model.to_cols_array()),
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
        }
        // the quads are generated here rather than stored, since a particle moves every frame
        for mesh in &self.scene.particles {
            let Some(particles) = self.frame.particles.get(&mesh.block) else {
                continue;
            };
            // the pose the frame walked, so an animated system draws where it now is rather
            // than where the scene was built. Picking walks at the same time, and the two have
            // to agree or the ray tests empty space.
            let model = self
                .frame
                .poses
                .get(&mesh.block)
                .copied()
                .unwrap_or(mesh.model);
            let scale = model.x_axis.truncate().length();
            let (right, up) = self.quad_axes();
            let mut vertices: Vec<f32> = Vec::with_capacity(particles.len() * 4 * VERTEX_FLOATS);
            for particle in particles.iter().take(mesh.capacity) {
                let centre = model.transform_point3(Vec3::from(&particle.position));
                let colour = &particle.color;
                // the radius is in the system's space, like the position it sits at, and the
                // scale comes from the same matrix as the position so the two cannot disagree
                let half = particle.radius.max(0.0) * scale;
                // a quad facing the camera, wound so the shared corners meet the index pattern
                for (corner, uv) in [
                    ((-1.0, -1.0), (0.0, 1.0)),
                    ((1.0, -1.0), (1.0, 1.0)),
                    ((1.0, 1.0), (1.0, 0.0)),
                    ((-1.0, 1.0), (0.0, 0.0)),
                ] {
                    let at = centre + right * (corner.0 * half) + up * (corner.1 * half);
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
                render_pass.set_bind_group(2, self.texture_of(mesh), &[]);
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
            for mesh in &self.scene.particles {
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
                Sorted::Particles(mesh, _) => self
                    .frame
                    .poses
                    .get(&mesh.block)
                    .copied()
                    .unwrap_or(mesh.model)
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
                        let pipeline = if self.cull {
                            &mesh.pipeline
                        } else {
                            &mesh.pipeline_unculled
                        };
                        draw_mesh(render_pass, mesh, pipeline, self.texture_of(mesh));
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

        for mesh in &self.scene.particles {
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
            render_pass.set_bind_group(2, self.texture_of(mesh), &[]);
            render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
            render_pass.set_index_buffer(mesh.edges.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..mesh.edge_count, 0, 0..1);
        }
        // a selected particle system outlines its quads the same way, so picking one shows what
        // was picked rather than leaving the selection invisible
        for mesh in &self.scene.particles {
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

    #[test]
    fn the_camera_uniform_carries_the_light_at_the_end() {
        let light = Light::default();
        let filled =
            super::camera_uniform(nif::glam::Mat4::IDENTITY, Vec3::ZERO, true, true, &light);

        assert_eq!(filled.len(), super::CAMERA_FLOATS as usize);
        assert_eq!(&filled[24..40], &light.uniform());
    }

    /// Every vertex is position, colour and two uv sets, and the axes are the last three lines.
    const STRIDE: usize = super::VERTEX_FLOATS;

    #[test]
    fn spacing_is_a_round_number_about_a_tenth_of_the_reach() {
        for (reach, expected) in [
            (1.0, 0.1),
            (10.0, 1.0),
            (50.0, 10.0),
            (6000.0, 1000.0),
            (0.05, 0.01),
        ] {
            let (_, spacing, _) = grid_lines(reach);
            assert_eq!(spacing, expected, "reach {reach}");
        }
    }

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
