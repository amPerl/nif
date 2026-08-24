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
    common::Triangle,
    Nif,
};
use wgpu::util::DeviceExt as _;

use crate::library::{self, TextureLibrary};
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
const MODEL_FLOATS: u64 = 56;
const CAMERA_FLOATS: u64 = 40;

/// Position, colour and two uv sets. The second exists because the dark slot reads uv set 1.
const VERTEX_FLOATS: usize = 3 + 4 + 2 + 2;

/// The slots the renderer draws, in the order their uv transforms sit in the model uniform.
const DRAWN_SLOTS: [TextureSlot; 3] = [TextureSlot::Base, TextureSlot::Dark, TextureSlot::Glow];

/// The one light the viewer invents, since a NIF does not carry the scene's lighting. Only 55 of
/// the 18,569 files whose shapes name a custom shader contain a light block at all, so a faithful
/// reading of the file would leave almost everything black.
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
    samplers: [wgpu::Sampler; 4],
    shader: wgpu::ShaderModule,
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
    texturing_block: Option<usize>,
    /// Every source a flip controller can swap into the base slot, by its own block. Empty
    /// unless one drives this shape.
    flip_frames: HashMap<usize, wgpu::BindGroup>,
    pub radius: f32,
    /// The NiLODNode this shape sits under, and which of its levels, if any.
    lod: Option<(usize, usize)>,
    /// Blended shapes draw after the opaque ones, back to front.
    blended: bool,
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

pub struct Scene {
    pub meshes: Vec<Mesh>,
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
    /// Every drawn slot's uv transform where a controller drives one, by texturing property
    /// block. All three slots ride together, since one controller per member means several can
    /// target one property at once.
    pub uv: HashMap<usize, [f32; 24]>,
    /// The source a flip controller has swapped into the base slot, by texturing property block.
    pub flip: HashMap<usize, usize>,
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
        // one per address mode combination, since TexClampMode is per map and glow maps are
        // clamped about as often as they wrap
        let samplers = std::array::from_fn(|i| {
            let mode = |wraps| {
                if wraps {
                    wgpu::AddressMode::Repeat
                } else {
                    wgpu::AddressMode::ClampToEdge
                }
            };
            device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("nifty sampler"),
                address_mode_u: mode(i & 1 != 0),
                address_mode_v: mode(i & 2 != 0),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("nifty"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
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
            shader,
            pipeline_layout: layout,
            camera_buffer,
        }
    }

    fn pipeline(&self, state: DrawState) -> wgpu::RenderPipeline {
        build_pipeline(
            &self.render_state.device,
            &self.pipeline_layout,
            &self.shader,
            self.render_state.target_format,
            state,
            wgpu::PrimitiveTopology::TriangleList,
            "fs_main",
        )
    }

    fn sampler(&self, wraps_u: bool, wraps_v: bool) -> wgpu::Sampler {
        self.samplers[usize::from(wraps_u) | usize::from(wraps_v) << 1].clone()
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
    fn slot_group(&self, slots: [(&wgpu::TextureView, &wgpu::Sampler); 3]) -> wgpu::BindGroup {
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
    pub fn build_scene(&self, nif: &Nif, library: &TextureLibrary) -> Scene {
        let lod_of = lod_ancestry(nif);
        let device = &self.render_state.device;
        let white = self.upload_texture(1, 1, &[255, 255, 255, 255]);
        let black = self.upload_texture(1, 1, &[0, 0, 0, 255]);
        // shapes whose texture could not be loaded get a checker rather than white
        let missing = {
            let (width, height, rgba) = library::placeholder();
            self.upload_texture(width, height, &rgba)
        };
        let mut cache: HashMap<usize, wgpu::TextureView> = HashMap::new();
        let mut pipelines: HashMap<DrawState, wgpu::RenderPipeline> = HashMap::new();
        let mut meshes = Vec::new();
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
            // the dark slot reads uv set 1 in all but 10 of the corpus's 1,871 dark maps, so
            // two sets go up and each slot picks the one its own TexDesc names
            let uvs = data.uv_sets.first().map(|set| &set.uvs);
            let uvs1 = data.uv_sets.get(1).map(|set| &set.uvs).or(uvs);
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
                attributes.extend_from_slice(&[v.x, v.y, v.z]);
                attributes.extend_from_slice(&color);
                attributes.extend_from_slice(&uv);
                attributes.extend_from_slice(&uv1);
            }

            // NiMaterialProperty is a D3DMATERIAL9 verbatim. There is no ambient term: the
            // engine multiplies material ambient by the global ambient, which is black unless
            // the scene carries an NiAmbientLight.
            // the block index as well as the material, since that is what a controller targets
            let material_index = geometry.property_refs.iter().find(|r| {
                matches!(r.get(&nif.blocks), Some(Block::NiMaterialProperty(_)))
            });
            let material_index = material_index.and_then(|r| r.index());
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
            let vertex_color =
                geometry
                    .property_refs
                    .iter()
                    .find_map(|r| match r.get(&nif.blocks) {
                        Some(Block::NiVertexColorProperty(p)) => Some(p),
                        _ => None,
                    });
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
            let replace = match texturing_of(nif, &geometry.property_refs) {
                Some(p) => f32::from(p.apply_mode == ApplyMode::Replace),
                None => 0.0,
            };

            let stencil = geometry
                .property_refs
                .iter()
                .find_map(|r| match r.get(&nif.blocks) {
                    Some(Block::NiStencilProperty(p)) => Some(p),
                    _ => None,
                });
            let zbuffer = geometry
                .property_refs
                .iter()
                .find_map(|r| match r.get(&nif.blocks) {
                    Some(Block::NiZBufferProperty(p)) => Some(p),
                    _ => None,
                });
            let alpha = geometry
                .property_refs
                .iter()
                .find_map(|r| match r.get(&nif.blocks) {
                    Some(Block::NiAlphaProperty(p)) => Some(p),
                    _ => None,
                });

            let blend = alpha.filter(|a| a.alpha_blend()).map(|a| {
                (
                    blend_factor(&a.source_blend_mode(), false),
                    blend_factor(&a.destination_blend_mode(), true),
                )
            });
            // the shader implements TestGreater only. TestAlways never discards.
            let (alpha_test, alpha_threshold) = match alpha {
                Some(a) if a.alpha_test() && a.test_func() != TestFunction::TestAlways => {
                    (1.0, f32::from(a.threshold) / 255.0)
                }
                _ => (0.0, 0.0),
            };
            let state = DrawState {
                cull: cull_of(stencil.map(|p| &p.draw_mode)),
                depth_write: zbuffer.is_none_or(|z| z.depth_write()),
                depth: depth_of(zbuffer),
                blend,
            };
            let pipeline = pipelines
                .entry(state)
                .or_insert_with(|| self.pipeline(state))
                .clone();
            let unculled = DrawState {
                cull: None,
                ..state
            };
            let pipeline_unculled = pipelines
                .entry(unculled)
                .or_insert_with(|| self.pipeline(unculled))
                .clone();

            // one upload per source texture, not per shape that uses it
            let texture_key = texturing_key(nif, &geometry.property_refs);
            let property = texturing_of(nif, &geometry.property_refs);
            let texturing_block = texturing_index(nif, &geometry.property_refs);

            // a slot the shape does not use has to change nothing: dark multiplies so it
            // defaults to white, glow adds so it defaults to black
            let mut views = [white.clone(), white.clone(), black.clone()];
            let mut samplers = [
                self.sampler(true, true),
                self.sampler(true, true),
                self.sampler(true, true),
            ];

            for (position, slot) in DRAWN_SLOTS.iter().enumerate() {
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
                samplers[position] =
                    self.sampler(desc.clamp_mode.wraps_u(), desc.clamp_mode.wraps_v());
            }
            let texture = self.slot_group([
                (&views[0], &samplers[0]),
                (&views[1], &samplers[1]),
                (&views[2], &samplers[2]),
            ]);
            // every frame a flip controller can reach, uploaded once each and shared by block
            let mut flip_frames = HashMap::new();
            for block in &nif.blocks {
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
                    let group = self.slot_group([
                        (&view, &samplers[0]),
                        (&views[1], &samplers[1]),
                        (&views[2], &samplers[2]),
                    ]);
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
            model_uniform[32..56].copy_from_slice(&slot_uv_rows(&nif.blocks, property, 0.0));

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
                blended: blend.is_some(),
                pipeline,
                pipeline_unculled,
                center: (shape_min + shape_max) * 0.5,
                local_center: (local_min + local_max) * 0.5,
                material_block: material_index,
                texturing_block,
                flip_frames,
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
        let (center, radius) = if meshes.is_empty() {
            (Vec3::ZERO, 1.0)
        } else {
            ((min + max) * 0.5, (max - min).length() * 0.5)
        };

        let samplers_for_grid = self.sampler(true, true);
        // the floor has to reach the geometry as well as the origin, which a chunk sitting
        // far out is nowhere near
        let (lines, spacing, half) = grid_lines(center.length() + radius);
        let mut identity = [0f32; MODEL_FLOATS as usize];
        identity[..16].copy_from_slice(&Mat4::IDENTITY.to_cols_array());
        identity[32..56].copy_from_slice(&slot_uv_rows(&[], None, 0.0));
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
            texture: self.slot_group([
                (&white, &samplers_for_grid),
                (&white, &samplers_for_grid),
                (&black, &samplers_for_grid),
            ]),
            spacing,
        };

        Scene {
            meshes,
            center,
            radius: radius.max(0.001),
            lods,
            grid,
        }
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

/// Every drawn slot's uv rows at `time`, in `DRAWN_SLOTS` order. Both the scene build and the
/// per frame update go through this, so an animated slot cannot be one the build forgot.
pub fn slot_uv_rows(
    blocks: &[Block],
    property: Option<&nif::blocks::NiTexturingProperty>,
    time: f32,
) -> [f32; 24] {
    let mut out = [0.0; 24];
    for (position, slot) in DRAWN_SLOTS.iter().enumerate() {
        let desc = property.and_then(|p| p.texture(*slot));
        let transform =
            property.and_then(|p| nif::anim::texture_transform_at(blocks, p, *slot, time));
        let rows = uv_rows(transform, desc.map_or(0, |d| d.uv_set));
        out[position * 8..(position + 1) * 8].copy_from_slice(&rows);
    }
    out
}

fn texturing_index(nif: &Nif, properties: &[nif::common::BlockRef]) -> Option<usize> {
    properties.iter().find_map(|r| {
        r.index()
            .filter(|i| matches!(nif.blocks.get(*i), Some(Block::NiTexturingProperty(_))))
    })
}

fn texturing_of<'a>(
    nif: &'a Nif,
    properties: &[nif::common::BlockRef],
) -> Option<&'a nif::blocks::NiTexturingProperty> {
    properties.iter().find_map(|r| match r.get(&nif.blocks) {
        Some(Block::NiTexturingProperty(p)) => Some(p),
        _ => None,
    })
}

fn texturing_key(nif: &Nif, properties: &[nif::common::BlockRef]) -> Option<usize> {
    texturing_of(nif, properties)?
        .base_texture
        .as_ref()?
        .source_ref
        .index()
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
            out.extend_from_slice(&color);
            out.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
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
                    0 => Float32x3, 1 => Float32x4, 2 => Float32x2, 3 => Float32x2
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
/// A texture and sampler pair per drawn slot, in `DRAWN_SLOTS` order.
fn slot_layout_entries() -> [wgpu::BindGroupLayoutEntry; 6] {
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
            if let Some(rows) = mesh
                .texturing_block
                .and_then(|block| self.frame.uv.get(&block))
            {
                queue.write_buffer(&mesh.model_buffer, UV_OFFSET, bytemuck::cast_slice(rows));
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
            // opaque first, then blended back to front
            let mut blended: Vec<&Mesh> = Vec::new();
            for mesh in self.scene.meshes.iter().filter(|m| self.visible(m)) {
                if mesh.blended {
                    blended.push(mesh);
                    continue;
                }
                let pipeline = if self.cull {
                    &mesh.pipeline
                } else {
                    &mesh.pipeline_unculled
                };
                draw_mesh(render_pass, mesh, pipeline, self.texture_of(mesh));
            }
            blended.sort_by(|a, b| {
                self.center(b)
                    .distance_squared(self.eye)
                    .total_cmp(&self.center(a).distance_squared(self.eye))
            });
            for mesh in blended {
                let pipeline = if self.cull {
                    &mesh.pipeline
                } else {
                    &mesh.pipeline_unculled
                };
                draw_mesh(render_pass, mesh, pipeline, self.texture_of(mesh));
            }
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
    }
}

const SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    flags: vec4<f32>,
    // the direction the light travels, with the rim strength in w
    light_dir: vec4<f32>,
    light_ambient: vec4<f32>,
    light_diffuse: vec4<f32>,
    light_specular: vec4<f32>,
};
struct Model {
    model: mat4x4<f32>,
    diffuse: vec4<f32>,
    emissive: vec4<f32>,
    // emissive from vertex colour, diffuse from vertex colour, lighting enabled, apply replace
    sources: vec4<f32>,
    // alpha test threshold, alpha test enabled
    alpha: vec4<f32>,
    // two rows per drawn slot, base then dark then glow. The third row is always (0, 0, 1),
    // and row0.w names which of the two uv sets the slot reads.
    uv: array<vec4<f32>, 6>,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> model: Model;
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var dark_texture: texture_2d<f32>;
@group(2) @binding(3) var dark_sampler: sampler;
@group(2) @binding(4) var glow_texture: texture_2d<f32>;
@group(2) @binding(5) var glow_sampler: sampler;

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) uv1: vec2<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) uv1: vec2<f32>,
) -> VertexOut {
    let world = model.model * vec4<f32>(position, 1.0);
    var out: VertexOut;
    out.clip = camera.view_proj * world;
    out.world = world.xyz;
    out.color = color;
    out.uv = uv;
    out.uv1 = uv1;
    return out;
}

/// The uv a slot samples at, after its own set choice and its own transform.
fn slot_uv(slot: u32, in: VertexOut) -> vec2<f32> {
    let row0 = model.uv[slot * 2u];
    let row1 = model.uv[slot * 2u + 1u];
    var source = in.uv;
    if (row0.w > 0.5) {
        source = in.uv1;
    }
    let uvw = vec3<f32>(source, 1.0);
    return vec2<f32>(dot(row0.xyz, uvw), dot(row1.xyz, uvw));
}

@fragment
fn fs_line(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}

@fragment
fn fs_wire(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(0.35, 0.95, 0.55, 1.0);
}

@fragment
fn fs_highlight(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.65, 0.15, 1.0);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // the sign of a derivative normal depends on framebuffer handedness, so orient it
    // toward the eye rather than trusting it, which holds with culling off too
    var normal = normalize(cross(dpdx(in.world), dpdy(in.world)));
    let to_eye = normalize(camera.eye.xyz - in.world);
    if (dot(normal, to_eye) < 0.0) {
        normal = -normal;
    }

    // the light travels away from its source, so the direction back to it is negated
    let to_light = -camera.light_dir.xyz;
    let lambert = clamp(dot(normal, to_light), 0.0, 1.0);
    let fill = camera.light_dir.w * clamp(dot(normal, to_eye), 0.0, 1.0);

    let shade = camera.light_ambient.rgb + camera.light_diffuse.rgb * lambert + vec3<f32>(fill);

    // the texture modulates the lit colour, so emissive is inside the multiply, not over it:
    // emissive 1,1,1 is a full brightness texture, not white. shade replaces the light sum.
    let vertex = in.color.rgb;
    let emissive_src = mix(model.emissive.rgb, vertex, model.sources.x);
    let diffuse_src = mix(model.diffuse.rgb, vertex, model.sources.y);
    let material_lit = clamp(
        emissive_src + diffuse_src * shade * model.sources.z,
        vec3<f32>(0.0),
        vec3<f32>(1.0)
    );

    let plain = vec3<f32>(0.78, 0.80, 0.84) * shade;
    let lit = mix(plain, material_lit, camera.flags.x);

    let texel = textureSample(base_texture, base_sampler, slot_uv(0u, in));
    // the dark map accumulates before the base map, which then modulates onto it, so it
    // multiplies. An absent slot is white and changes nothing.
    let dark = textureSample(dark_texture, dark_sampler, slot_uv(1u, in)).rgb;
    // the glow map is added in a ONE,ONE pass after everything, so it is unlit. Absent is black.
    let glow = textureSample(glow_texture, glow_sampler, slot_uv(2u, in)).rgb;

    let replace = model.sources.w * camera.flags.x;
    let base = texel.rgb * dark;
    let shaded = mix(lit * base, base, replace) + glow;

    // alpha follows the same source as diffuse, and the texture modulates it like the colour
    let alpha_src = mix(model.diffuse.a, in.color.a, model.sources.y);
    let alpha = mix(alpha_src, alpha_src * texel.a, camera.flags.y);
    if (model.alpha.y > 0.5 && alpha <= model.alpha.x) {
        discard;
    }
    return vec4<f32>(mix(lit, shaded, camera.flags.y), alpha);
}
"#;

#[cfg(test)]
mod tests {
    use super::{grid_lines, Light};
    use nif::glam::Vec3;

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
