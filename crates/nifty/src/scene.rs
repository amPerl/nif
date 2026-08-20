use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use eframe::egui;
use eframe::egui_wgpu::{self, wgpu};
use nif::glam::{Mat4, Vec3};
use nif::{
    blocks::{
        AlphaFunction, ApplyMode, Block, LightMode, NiGeometry, NiGeometryData, StencilDrawMode,
        TestFunction, VertMode,
    },
    common::{NiTransform, Triangle},
    Nif,
};
use wgpu::util::DeviceExt as _;

use crate::library::{self, TextureLibrary};
use crate::texture::decode_texture;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

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
    sampler: wgpu::Sampler,
    shader: wgpu::ShaderModule,
    pipeline_layout: wgpu::PipelineLayout,
    pub camera_buffer: wgpu::Buffer,
}

pub struct Mesh {
    pub shape_block: usize,
    pub data_block: usize,
    pub center: Vec3,
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
}

/// A NiLODNode's switching distances, and the point they are measured from.
pub struct Lod {
    pub center: Vec3,
    pub ranges: Vec<(f32, f32)>,
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
    depth_test: bool,
    blend: Option<(wgpu::BlendFactor, wgpu::BlendFactor)>,
}

impl DrawState {
    fn opaque() -> Self {
        Self {
            cull: Some(wgpu::Face::Back),
            depth_write: true,
            depth_test: true,
            blend: None,
        }
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
            entries: &[uniform_entry()],
        });
        let model_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("nifty model"),
            entries: &[uniform_entry()],
        });
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("nifty texture"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("nifty sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("nifty camera"),
            size: 96,
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
                depth_test: false,
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
            sampler,
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

    fn upload_texture(&self, width: u32, height: u32, rgba: &[u8]) -> wgpu::BindGroup {
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

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("nifty texture"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        })
    }

    /// Resolves a shape's base texture, embedded or from the library on disk.
    fn shape_texture(
        &self,
        nif: &Nif,
        shape_properties: &[nif::common::BlockRef],
        library: &TextureLibrary,
    ) -> Option<wgpu::BindGroup> {
        let texturing = shape_properties
            .iter()
            .find_map(|r| match r.get(&nif.blocks) {
                Some(Block::NiTexturingProperty(p)) => Some(p),
                _ => None,
            })?;
        let source = texturing
            .base_texture
            .as_ref()?
            .source_ref
            .get(&nif.blocks)?;
        let Block::NiSourceTexture(source) = source else {
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
        // shapes whose texture could not be loaded get a checker rather than white
        let missing = {
            let (width, height, rgba) = library::placeholder();
            self.upload_texture(width, height, &rgba)
        };
        let mut cache: HashMap<usize, wgpu::BindGroup> = HashMap::new();
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
                    let local = Vec3::new(data.center.x, data.center.y, data.center.z);
                    lods.insert(
                        visit.index,
                        Lod {
                            center: model_matrix(&visit.transform).transform_point3(local),
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

            let model = model_matrix(&visit.transform);
            let colors = data.vertex_colors.as_ref();
            let uvs = data.uv_sets.first().map(|set| &set.uvs);
            let mut attributes: Vec<f32> = Vec::with_capacity(vertices.len() * 9);
            let mut shape_min = Vec3::splat(f32::MAX);
            let mut shape_max = Vec3::splat(f32::MIN);
            for (i, v) in vertices.iter().enumerate() {
                let world = model.transform_point3(Vec3::new(v.x, v.y, v.z));
                min = min.min(world);
                max = max.max(world);
                shape_min = shape_min.min(world);
                shape_max = shape_max.max(world);

                let color = colors
                    .and_then(|c| c.get(i))
                    .map(|c| [c.r, c.g, c.b, c.a])
                    .unwrap_or([1.0, 1.0, 1.0, 1.0]);
                let uv = uvs
                    .and_then(|set| set.get(i))
                    .map(|t| [t.u, t.v])
                    .unwrap_or([0.0, 0.0]);
                attributes.extend_from_slice(&[v.x, v.y, v.z]);
                attributes.extend_from_slice(&color);
                attributes.extend_from_slice(&uv);
            }

            // NiMaterialProperty is a D3DMATERIAL9 verbatim. There is no ambient term: the
            // engine multiplies material ambient by the global ambient, which is black unless
            // the scene carries an NiAmbientLight.
            let material = geometry
                .property_refs
                .iter()
                .find_map(|r| match r.get(&nif.blocks) {
                    Some(Block::NiMaterialProperty(m)) => Some(m),
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
                depth_test: zbuffer.is_none_or(|z| z.depth_test()),
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
            let texture = match texture_key {
                Some(key) => cache
                    .entry(key)
                    .or_insert_with(|| {
                        self.shape_texture(nif, &geometry.property_refs, library)
                            .unwrap_or_else(|| missing.clone())
                    })
                    .clone(),
                None => white.clone(),
            };

            let mut model_uniform = [0f32; 32];
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
                usage: wgpu::BufferUsages::UNIFORM,
            });

            meshes.push(Mesh {
                shape_block: visit.index,
                lod: lod_of.get(&visit.index).copied(),
                blended: blend.is_some(),
                pipeline,
                pipeline_unculled,
                center: (shape_min + shape_max) * 0.5,
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
            });
        }

        // min/max, not a half-extent about the origin, since terrain chunks sit far off it.
        let (center, radius) = if meshes.is_empty() {
            (Vec3::ZERO, 1.0)
        } else {
            ((min + max) * 0.5, (max - min).length() * 0.5)
        };

        // the floor has to reach the geometry as well as the origin, which a chunk sitting
        // far out is nowhere near
        let (lines, spacing) = grid_lines(center.length() + radius);
        let mut identity = [0f32; 32];
        identity[..16].copy_from_slice(&Mat4::IDENTITY.to_cols_array());
        let grid = Grid {
            count: (lines.len() / 9) as u32,
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
            texture: white,
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
fn grid_lines(reach: f32) -> (Vec<f32>, f32) {
    let spacing = 10f32.powf((reach.max(1e-3) / 10.0).log10().round());
    let cells = ((reach / spacing).ceil() as i32).clamp(4, 40);
    let half = cells as f32 * spacing;

    let mut out = Vec::new();
    let mut line = |from: Vec3, to: Vec3, color: [f32; 4]| {
        for point in [from, to] {
            out.extend_from_slice(&[point.x, point.y, point.z]);
            out.extend_from_slice(&color);
            out.extend_from_slice(&[0.0, 0.0]);
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

    (out, spacing)
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
                array_stride: 36,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32x2],
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
            depth_compare: Some(if state.depth_test {
                wgpu::CompareFunction::Less
            } else {
                wgpu::CompareFunction::Always
            }),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}

fn uniform_entry() -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub(crate) fn model_matrix(transform: &NiTransform) -> Mat4 {
    Mat4::from_translation(transform.translation.into())
        * Mat4::from_mat3(transform.rotation.into())
        * Mat4::from_scale(Vec3::splat(transform.scale))
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
}

impl PreviewCall {
    fn visible(&self, mesh: &Mesh) -> bool {
        self.scene
            .shows(mesh, self.lod_mode, self.lod_distance, self.eye)
    }
}

fn draw_mesh(
    render_pass: &mut wgpu::RenderPass<'static>,
    mesh: &Mesh,
    pipeline: &wgpu::RenderPipeline,
) {
    render_pass.set_pipeline(pipeline);
    render_pass.set_bind_group(1, &mesh.bind_group, &[]);
    render_pass.set_bind_group(2, &mesh.texture, &[]);
    render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
    render_pass.set_index_buffer(mesh.indices.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..mesh.count, 0, 0..1);
}

impl egui_wgpu::CallbackTrait for PreviewCall {
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
                render_pass.set_bind_group(2, &mesh.texture, &[]);
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
                draw_mesh(render_pass, mesh, pipeline);
            }
            blended.sort_by(|a, b| {
                b.center
                    .distance_squared(self.eye)
                    .total_cmp(&a.center.distance_squared(self.eye))
            });
            for mesh in blended {
                let pipeline = if self.cull {
                    &mesh.pipeline
                } else {
                    &mesh.pipeline_unculled
                };
                draw_mesh(render_pass, mesh, pipeline);
            }
        }

        // the selected shape gets its wireframe drawn over everything, so it stays findable
        let Some(selected) = self.selected else {
            return;
        };
        render_pass.set_pipeline(&preview.highlight);
        for mesh in &self.scene.meshes {
            if mesh.shape_block != selected && mesh.data_block != selected {
                continue;
            }
            render_pass.set_bind_group(1, &mesh.bind_group, &[]);
            render_pass.set_bind_group(2, &mesh.texture, &[]);
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
};
struct Model {
    model: mat4x4<f32>,
    diffuse: vec4<f32>,
    emissive: vec4<f32>,
    // emissive from vertex colour, diffuse from vertex colour, lighting enabled, apply replace
    sources: vec4<f32>,
    // alpha test threshold, alpha test enabled
    alpha: vec4<f32>,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> model: Model;
@group(2) @binding(0) var base_texture: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
) -> VertexOut {
    let world = model.model * vec4<f32>(position, 1.0);
    var out: VertexOut;
    out.clip = camera.view_proj * world;
    out.world = world.xyz;
    out.color = color;
    out.uv = uv;
    return out;
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

    // NIF is Z-up, so a key light from above means +Z
    let key = normalize(vec3<f32>(0.3, 0.45, 0.85));
    let lambert = clamp(dot(normal, key), 0.0, 1.0);
    let fill = 0.3 * clamp(dot(normal, to_eye), 0.0, 1.0);

    let shade = 0.2 + 0.65 * lambert + fill;

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

    let texel = textureSample(base_texture, base_sampler, in.uv);
    let replace = model.sources.w * camera.flags.x;
    let shaded = mix(lit * texel.rgb, texel.rgb, replace);

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
    use super::grid_lines;

    /// Every vertex is position, colour and uv, and the axes are the last three lines.
    const STRIDE: usize = 9;

    #[test]
    fn spacing_is_a_round_number_about_a_tenth_of_the_reach() {
        for (reach, expected) in [
            (1.0, 0.1),
            (10.0, 1.0),
            (50.0, 10.0),
            (6000.0, 1000.0),
            (0.05, 0.01),
        ] {
            let (_, spacing) = grid_lines(reach);
            assert_eq!(spacing, expected, "reach {reach}");
        }
    }

    #[test]
    fn the_floor_reaches_at_least_as_far_as_the_scene() {
        for reach in [0.05, 1.0, 7.5, 240.0, 6000.0] {
            let (lines, _) = grid_lines(reach);
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

    #[test]
    fn a_scene_at_the_origin_still_gets_a_grid() {
        let (lines, spacing) = grid_lines(0.0);
        assert!(spacing > 0.0);
        assert!(!lines.is_empty());
        assert!(lines.iter().all(|f| f.is_finite()));
    }

    #[test]
    fn only_the_positive_axes_are_drawn() {
        let (lines, _) = grid_lines(10.0);
        // three axis lines, two vertices each, at the end of the buffer
        let axes = &lines[lines.len() - 6 * STRIDE..];
        for (i, axis) in axes.chunks(2 * STRIDE).enumerate() {
            assert_eq!(&axis[..3], &[0.0, 0.0, 0.0], "axis {i} starts at the origin");
            assert!(axis[STRIDE + i] > 0.0, "axis {i} runs positive");
        }
    }
}
