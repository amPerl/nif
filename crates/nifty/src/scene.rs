use std::collections::HashMap;
use std::sync::Arc;

use eframe::egui;
use eframe::egui_wgpu::{self, wgpu};
use nif::glam::{Mat4, Vec3};
use nif::{
    blocks::{ApplyMode, Block, LightMode, NiGeometry, NiGeometryData, VertMode},
    common::{NiTransform, Triangle},
    Nif,
};
use wgpu::util::DeviceExt as _;

use crate::texture::decode_texture;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// Lives in `callback_resources`, which is all `paint` can reach.
pub struct Preview {
    solid: wgpu::RenderPipeline,
    solid_two_sided: wgpu::RenderPipeline,
    wire: wgpu::RenderPipeline,
    highlight: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,
}

/// Lives on the app, for building meshes when a file loads.
pub struct Gfx {
    pub render_state: egui_wgpu::RenderState,
    model_layout: wgpu::BindGroupLayout,
    texture_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pub camera_buffer: wgpu::Buffer,
}

pub struct Mesh {
    pub shape_block: usize,
    pub data_block: usize,
    pub center: Vec3,
    pub radius: f32,
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
        let build_with = |cull: Option<wgpu::Face>,
                          topology: wgpu::PrimitiveTopology,
                          fragment_entry: &str,
                          depth_compare: wgpu::CompareFunction,
                          depth_write: bool| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("nifty"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 36,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32x2],
                    }],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(fragment_entry),
                    targets: &[Some(render_state.target_format.into())],
                    compilation_options: Default::default(),
                }),
                // Measured, not assumed: 67 of 74 closed fixture shapes have positive signed
                // volume, i.e. triangles run counter-clockwise seen from outside. slipvillage
                // agrees: it reverses to (c, b, a) because Godot treats clockwise as front.
                primitive: wgpu::PrimitiveState {
                    topology,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: cull,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: Some(depth_write),
                    depth_compare: Some(depth_compare),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let build = |cull, topology, fragment_entry| {
            build_with(
                cull,
                topology,
                fragment_entry,
                wgpu::CompareFunction::Less,
                true,
            )
        };

        let solid = build(
            Some(wgpu::Face::Back),
            wgpu::PrimitiveTopology::TriangleList,
            "fs_main",
        );
        let solid_two_sided = build(None, wgpu::PrimitiveTopology::TriangleList, "fs_main");
        let highlight = build_with(
            None,
            wgpu::PrimitiveTopology::LineList,
            "fs_highlight",
            wgpu::CompareFunction::Always,
            false,
        );
        // line list rather than PolygonMode::Line, which needs a device feature
        let wire = build(None, wgpu::PrimitiveTopology::LineList, "fs_wire");

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(Preview {
                solid,
                solid_two_sided,
                wire,
                highlight,
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
            camera_buffer,
        }
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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
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

    /// Resolves a shape's base texture, if it is embedded rather than a file reference.
    fn shape_texture(
        &self,
        nif: &Nif,
        shape_properties: &[nif::common::BlockRef],
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
        if source.use_external {
            return None;
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
    pub fn build_scene(&self, nif: &Nif) -> Scene {
        let device = &self.render_state.device;
        let white = self.upload_texture(1, 1, &[255, 255, 255, 255]);
        let mut cache: HashMap<usize, wgpu::BindGroup> = HashMap::new();
        let mut meshes = Vec::new();
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        for visit in nif.walk() {
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

            // NiMaterialProperty is a D3DMATERIAL9 verbatim. There is no ambient term here:
            // the engine multiplies material ambient by the global ambient, which is black
            // unless the scene carries an NiAmbientLight, and 31 of 31,434 corpus files do.
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

            // NiVertexColorProperty does not tint: it re-routes which source feeds each D3D
            // material channel. LightMode::Emissive uploads no lights at all, and paired with
            // SourceEmissive it disables lighting outright so the vertex colour passes through.
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

            // one upload per source texture, not per shape that uses it
            let texture_key = texturing_key(nif, &geometry.property_refs);
            let texture = match texture_key {
                Some(key) => cache
                    .entry(key)
                    .or_insert_with(|| {
                        self.shape_texture(nif, &geometry.property_refs)
                            .unwrap_or_else(|| self.upload_texture(1, 1, &[255, 255, 255, 255]))
                    })
                    .clone(),
                None => white.clone(),
            };

            let mut model_uniform = [0f32; 28];
            model_uniform[..16].copy_from_slice(&model.to_cols_array());
            model_uniform[16..20].copy_from_slice(&diffuse);
            model_uniform[20..24].copy_from_slice(&emissive);
            model_uniform[24..28].copy_from_slice(&[
                emissive_from_vertex,
                diffuse_from_vertex,
                lighting,
                replace * f32::from(texture_key.is_some()),
            ]);

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

        Scene {
            meshes,
            center,
            radius: radius.max(0.001),
        }
    }
}

/// A drawable shape's geometry, its vertex data and its triangles, whether the file stores
/// those as a triangle list or as strips.
fn geometry_of<'a>(
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

fn model_matrix(transform: &NiTransform) -> Mat4 {
    Mat4::from_translation(transform.translation.into())
        * Mat4::from_mat3(transform.rotation.into())
        * Mat4::from_scale(Vec3::splat(transform.scale))
}

pub struct PreviewCall {
    pub scene: Arc<Scene>,
    pub wireframe: bool,
    pub cull: bool,
    pub selected: Option<usize>,
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

        let pipeline = match (self.wireframe, self.cull) {
            (true, _) => &preview.wire,
            (false, true) => &preview.solid,
            (false, false) => &preview.solid_two_sided,
        };

        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &preview.camera_bind_group, &[]);
        for mesh in &self.scene.meshes {
            render_pass.set_bind_group(1, &mesh.bind_group, &[]);
            render_pass.set_bind_group(2, &mesh.texture, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertices.slice(..));
            if self.wireframe {
                render_pass.set_index_buffer(mesh.edges.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.edge_count, 0, 0..1);
            } else {
                render_pass.set_index_buffer(mesh.indices.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.count, 0, 0..1);
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
    // emissive 1,1,1 is a full-brightness texture, not white. shade stands in for the lights.
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
    return vec4<f32>(mix(lit, shaded, camera.flags.y), 1.0);
}
"#;
