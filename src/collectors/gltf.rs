use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub use gltf_json as json;
use gltf_json::scene::UnitQuaternion;
use gltf_json::validation::Checked::Valid;

use crate::{
    blocks::*,
    common::{BlockRef, Color4, TexCoord, Vector3},
    Nif,
};

#[derive(Default, Debug)]
pub struct Gltf {
    root: json::Root,
    buffers: Vec<Vec<u8>>,
}

impl Gltf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_to_files(&self, gltf_path: PathBuf) -> anyhow::Result<()> {
        let out_gltf = BufWriter::new(File::create(&gltf_path)?);

        let mut gltf = self.root.clone();

        for (buffer_idx, buffer_vec) in self.buffers.iter().enumerate() {
            if buffer_vec.is_empty() {
                continue;
            }

            let gltf_filename = gltf_path.file_name().unwrap().to_str().unwrap();

            let relative_buffer_path = format!("{}.Buffer{}.bin", gltf_filename, buffer_idx);
            let buffer_path = gltf_path.with_file_name(&relative_buffer_path);

            gltf.buffers.push(json::Buffer {
                byte_length: buffer_vec.len() as u32,
                name: Some(format!("{}_Buffer{}", gltf_filename, buffer_idx)),
                uri: Some(relative_buffer_path),
                extensions: Default::default(),
                extras: Default::default(),
            });

            let mut buffer_writer = BufWriter::new(File::create(&buffer_path)?);
            buffer_writer.write_all(buffer_vec)?;
        }

        json::serialize::to_writer_pretty(out_gltf, &gltf)?;
        Ok(())
    }

    pub fn clone_node(
        &mut self,
        node_index: json::Index<json::Node>,
        position: Option<[f32; 3]>,
        rotation: Option<[f32; 9]>,
        scale: Option<f32>,
    ) -> json::Index<json::Node> {
        let existing_node = self
            .root
            .nodes
            .get(node_index.value())
            .expect("attempted to clone non-existing node");

        let mut new_node = existing_node.clone();
        new_node.name = new_node.name.map(|name| format!("{}_Clone", name));
        new_node.children = new_node.children.map(|children| {
            children
                .iter()
                .map(|child_index| self.clone_node(*child_index, None, None, None))
                .collect()
        });
        if let Some(position) = position {
            new_node.translation = Some(position);
        }
        if let Some(rotation) = rotation {
            let rotation_quat =
                glam::Quat::from_mat3(&glam::Mat3::from_cols_array(&rotation).transpose());

            let mut rotation_quat_slice = [0f32; 4];
            rotation_quat.write_to_slice(&mut rotation_quat_slice);

            new_node.rotation = Some(UnitQuaternion(rotation_quat_slice));
        }
        if let Some(scale) = scale {
            new_node.scale = Some([scale, scale, scale]);
        }

        self.root.nodes.push(new_node);
        json::Index::new(self.root.nodes.len() as u32 - 1)
    }

    pub fn get_or_create_scene(
        &mut self,
        name: &str,
        nodes: Option<Vec<json::Index<json::Node>>>,
    ) -> json::Index<json::Scene> {
        let scene_index = json::Index::new(
            self.root
                .scenes
                .iter()
                .enumerate()
                .find(|(_, s)| s.name.as_ref().unwrap() == name)
                .map(|(i, _)| i)
                .unwrap_or_else(|| {
                    self.root.scenes.push(json::Scene {
                        extensions: None,
                        extras: Default::default(),
                        name: Some(name.into()),
                        nodes: nodes.unwrap_or_default(),
                    });
                    self.root.scenes.len() - 1
                }) as u32,
        );

        if self.root.scene.is_none() {
            self.root.scene = Some(scene_index);
        }

        scene_index
    }

    fn get_or_create_image(&mut self, uri: &str) -> json::Index<json::Image> {
        json::Index::new(
            self.root
                .images
                .iter()
                .enumerate()
                .find(|(_, image)| image.uri.as_ref().unwrap() == uri)
                .map(|(i, _)| i)
                .unwrap_or_else(|| {
                    self.root.images.push(json::Image {
                        buffer_view: None,
                        mime_type: None,
                        name: Some(uri.into()),
                        uri: Some(uri.into()),
                        extensions: Default::default(),
                        extras: Default::default(),
                    });
                    self.root.images.len() - 1
                }) as u32,
        )
    }

    fn get_or_create_texture(
        &mut self,
        image_index: json::Index<json::Image>,
    ) -> json::Index<json::Texture> {
        json::Index::new(
            self.root
                .textures
                .iter()
                .enumerate()
                .find(|(_, texture)| texture.source.value() == image_index.value())
                .map(|(i, _)| i)
                .unwrap_or_else(|| {
                    self.root.textures.push(json::Texture {
                        name: None,
                        sampler: None,
                        source: image_index,
                        extensions: Default::default(),
                        extras: Default::default(),
                    });
                    self.root.textures.len() - 1
                }) as u32,
        )
    }

    #[allow(clippy::float_cmp)]
    fn get_or_create_material(
        &mut self,
        material_property: &NiMaterialProperty,
        base_color_texture: &Option<json::texture::Info>,
        alpha_property: Option<&NiAlphaProperty>,
        name: &str,
    ) -> json::Index<json::Material> {
        let (expected_alpha_cutoff, expected_alpha_mode) = match alpha_property {
            Some(_) => (
                Some(json::material::AlphaCutoff(0.5)),
                Valid(json::material::AlphaMode::Mask),
            ),
            None => (None, Valid(json::material::AlphaMode::Opaque)),
        };

        let expected_base_color_factor_inner = [
            material_property.color_diffuse.r,
            material_property.color_diffuse.g,
            material_property.color_diffuse.b,
            material_property.alpha,
        ];

        let expected_roughness_factor_inner = (100.0 - material_property.glossiness) / 100.0;

        let expected_emissive_factor_inner = [
            material_property.color_emissive.r,
            material_property.color_emissive.g,
            material_property.color_emissive.b,
        ];

        json::Index::new(
            self.root
                .materials
                .iter()
                .enumerate()
                .find(|(_, material)| {
                    let alpha_match = match (&material.alpha_cutoff, &material.alpha_mode) {
                        (None, Valid(alpha_mode)) => {
                            if let Valid(expected_mode) = &expected_alpha_mode {
                                expected_alpha_cutoff.is_none() && expected_mode == alpha_mode
                            } else {
                                false
                            }
                        }
                        (Some(alpha_cutoff), Valid(alpha_mode)) => {
                            if let Some(expected_cutoff) = &expected_alpha_cutoff {
                                if let Valid(expected_mode) = &expected_alpha_mode {
                                    expected_cutoff.0 == alpha_cutoff.0
                                        && expected_mode == alpha_mode
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    let base_color_match = material.pbr_metallic_roughness.base_color_factor.0
                        == expected_base_color_factor_inner;

                    let roughness_factor_match = material.pbr_metallic_roughness.roughness_factor.0
                        == expected_roughness_factor_inner;

                    let emissive_factor_match =
                        material.emissive_factor.0 == expected_emissive_factor_inner;

                    let base_color_texture_match = match (
                        &material.pbr_metallic_roughness.base_color_texture,
                        base_color_texture,
                    ) {
                        (Some(mat_tex), Some(expected_tex)) => {
                            mat_tex.index.value() == expected_tex.index.value()
                        }
                        (None, None) => true,
                        _ => false,
                    };

                    alpha_match
                        && base_color_match
                        && base_color_texture_match
                        && roughness_factor_match
                        && emissive_factor_match
                })
                .map(|(i, _material)| i)
                .unwrap_or_else(|| {
                    self.root.materials.push(json::Material {
                        alpha_cutoff: expected_alpha_cutoff,
                        alpha_mode: expected_alpha_mode,
                        double_sided: false,
                        name: Some(format!("{}_Material", name)),
                        pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                            base_color_factor: json::material::PbrBaseColorFactor(
                                expected_base_color_factor_inner,
                            ),
                            base_color_texture: base_color_texture.clone(),
                            metallic_factor: json::material::StrengthFactor(0.0),
                            roughness_factor: json::material::StrengthFactor(
                                expected_roughness_factor_inner,
                            ),
                            metallic_roughness_texture: None,
                            extensions: None,
                            extras: Default::default(),
                        },
                        normal_texture: None,
                        occlusion_texture: None,
                        emissive_texture: None,
                        emissive_factor: json::material::EmissiveFactor(
                            expected_emissive_factor_inner,
                        ),
                        extensions: None,
                        extras: Default::default(),
                    });
                    self.root.materials.len() - 1
                }) as u32,
        )
    }

    pub fn visit_nif(
        &mut self,
        nif: &Nif,
        scene_name: Option<&str>,
        name: &str,
    ) -> json::Index<json::Node> {
        let root_node = nif
            .blocks
            .first()
            .expect("nif should have at least a single node");

        match root_node {
            Block::NiNode(ni_node) => {
                if self.buffers.is_empty() {
                    self.buffers.push(Vec::new());
                }

                let root_node_index = self.visit_ni_node(nif, ni_node, name, None);

                if let Some(scene_name) = scene_name {
                    let scene_json_index =
                        self.get_or_create_scene(scene_name, Some(vec![root_node_index]));

                    let existing_scene = self
                        .root
                        .scenes
                        .get_mut(scene_json_index.value())
                        .expect("wat");

                    if !existing_scene
                        .nodes
                        .iter()
                        .any(|n| n.value() == root_node_index.value())
                    {
                        existing_scene.nodes.push(root_node_index);
                    }
                }

                root_node_index
            }
            _ => panic!("root element should be NiNode"),
        }
    }

    pub fn visit_ni_lod_node(
        &mut self,
        nif: &Nif,
        ni_lod_node: &NiLODNode,
        name: &str,
    ) -> Option<json::Index<json::Node>> {
        let lod_range_data_block = ni_lod_node
            .lod_level_data_ref
            .get(&nif.blocks)
            .expect("invalid lod_level_data_ref");

        let mut closest_lod: Option<(usize, f32)> = None;

        match lod_range_data_block {
            Block::NiRangeLODData(ni_range_lod_data) => {
                for (i, lod_range) in ni_range_lod_data.lod_levels.iter().enumerate() {
                    if let Some((_, near)) = closest_lod.as_ref() {
                        if &lod_range.near > near {
                            continue;
                        }
                    }
                    closest_lod = Some((i, lod_range.near));
                }
            }
            _ => panic!("lod_level_data_ref should point to NiRangeLODData"),
        }

        let closest_lod_ref = ni_lod_node
            .base
            .base
            .child_refs
            .get(closest_lod.expect("no closest lod").0)
            .expect("closest lod index was invalid child index");

        Some(self.visit_ni_node(nif, &ni_lod_node.base.base, name, Some(*closest_lod_ref)))
    }

    pub fn visit_ni_node(
        &mut self,
        nif: &Nif,
        ni_node: &NiNode,
        name: &str,
        single_child_ref: Option<BlockRef>,
    ) -> json::Index<json::Node> {
        let av_object = &ni_node.base;

        let rotation_quat = glam::Quat::from_mat3(&(&av_object.rotation).into());

        let mut rotation_quat_slice = [0f32; 4];
        rotation_quat.write_to_slice(&mut rotation_quat_slice);

        let rotation_quat_json = UnitQuaternion(rotation_quat_slice);

        let mut node_children = Vec::new();

        for &child_ref in ni_node.child_refs.iter() {
            if child_ref.0 < 0 {
                continue;
            }
            if let Some(single_child_ref) = single_child_ref {
                if child_ref != single_child_ref {
                    continue;
                }
            }

            let child = child_ref
                .get(&nif.blocks)
                .unwrap_or_else(|| panic!("invalid child ref: {}", child_ref.0));

            match child {
                Block::NiNode(ni_node) => {
                    node_children.push(self.visit_ni_node(
                        nif,
                        ni_node,
                        &format!("{}_NiNode{}", name, child_ref.0),
                        None,
                    ));
                }
                Block::NiLODNode(ni_lod_node) => {
                    if let Some(child_index) = self.visit_ni_lod_node(
                        nif,
                        ni_lod_node,
                        &format!("{}_NiLODNode{}", name, child_ref.0),
                    ) {
                        node_children.push(child_index);
                    }
                }
                Block::NiTriShape(ni_tri_shape) => {
                    if let Some(node) = self.visit_tri_shape(
                        nif,
                        ni_tri_shape,
                        &format!("{}_NiTriShape{}", name, child_ref.0),
                    ) {
                        node_children.push(node);
                    }
                }
                _ => {}
            }
        }

        let node = json::Node {
            camera: None,
            children: if !node_children.is_empty() {
                Some(node_children)
            } else {
                None
            },
            extensions: Default::default(),
            extras: Default::default(),
            matrix: None,
            mesh: None,
            name: Some(name.into()),
            rotation: Some(rotation_quat_json),
            scale: Some([av_object.scale, av_object.scale, av_object.scale]),
            translation: Some([
                av_object.translation.x,
                av_object.translation.y,
                av_object.translation.z,
            ]),
            skin: None,
            weights: None,
        };

        self.root.nodes.push(node);
        json::Index::new(self.root.nodes.len() as u32 - 1)
    }

    pub fn visit_tri_shape(
        &mut self,
        nif: &Nif,
        tri_shape: &NiTriShape,
        name: &str,
    ) -> Option<json::Index<json::Node>> {
        let av_object = &tri_shape.base.base;

        let rotation_quat = glam::Quat::from_mat3(&(&av_object.rotation).into());

        let mut rotation_quat_slice = [0f32; 4];
        rotation_quat.write_to_slice(&mut rotation_quat_slice);

        let rotation_quat_json = UnitQuaternion(rotation_quat_slice);

        let mut material_property: Option<&NiMaterialProperty> = None;
        let mut texturing_property: Option<&NiTexturingProperty> = None;
        let mut alpha_property: Option<&NiAlphaProperty> = None;

        for &property_ref in av_object.property_refs.iter() {
            let property = property_ref.get(&nif.blocks).expect("invalid property ref");

            match property {
                Block::NiMaterialProperty(ni_material_property) => {
                    material_property = Some(ni_material_property);
                }
                Block::NiTexturingProperty(ni_texturing_property) => {
                    texturing_property = Some(ni_texturing_property);
                }
                Block::NiAlphaProperty(ni_alpha_property) => {
                    alpha_property = Some(ni_alpha_property);
                }
                _ => {}
            };
        }

        let mut material_index = None;

        if let Some(material_property) = material_property {
            let mut base_color_texture = None;
            if let Some(texturing_property) = texturing_property {
                if let Some(base_texture_desc) = &texturing_property.base_texture {
                    let diffuse_source_ref = base_texture_desc.source_ref;
                    let diffuse_source = diffuse_source_ref
                        .get(&nif.blocks)
                        .expect("invalid property ref");

                    if let Block::NiSourceTexture(source_texture) = diffuse_source {
                        if source_texture.use_external > 0 {
                            let file_name = source_texture
                                .file_name
                                .value
                                .replace(".tga", ".dds")
                                .replace(".TGA", ".dds")
                                .replace(".bmp", ".dds")
                                .replace(".BMP", ".dds");

                            let image_index = self.get_or_create_image(&file_name);
                            let texture_index = self.get_or_create_texture(image_index);

                            base_color_texture = Some(json::texture::Info {
                                index: texture_index,
                                tex_coord: 0,
                                extensions: Default::default(),
                                extras: Default::default(),
                            });
                        }
                    }
                }
            }

            material_index = Some(self.get_or_create_material(
                material_property,
                &base_color_texture,
                alpha_property,
                name,
            ));
        }

        let data = tri_shape
            .data_ref
            .get(&nif.blocks)
            .expect("invalid property ref");

        let mesh_index = match data {
            Block::NiTriShapeData(tri_shape_data) => {
                let tri_geom_data = &tri_shape_data.base;
                let geom_data = &tri_geom_data.base;

                let expected_vertices_length = geom_data
                    .vertices
                    .as_ref()
                    .map(|v| v.len() * 12)
                    .unwrap_or(0);

                let expected_normals_length = geom_data
                    .normals
                    .as_ref()
                    .map(|v| v.len() * 12)
                    .unwrap_or(0);

                let expected_triangles_length = tri_shape_data
                    .triangles
                    .as_ref()
                    .map(|v| v.len() * 6)
                    .unwrap_or(0);

                let expected_colors_length = geom_data
                    .vertex_colors
                    .as_ref()
                    .map(|v| v.len() * 16)
                    .unwrap_or(0);

                let expected_uvs_length: usize = geom_data
                    .uv_sets
                    .first()
                    .map(|uv_set| uv_set.uvs.len() * 8)
                    .unwrap_or(0);

                let buffer_capacity = expected_vertices_length
                    + expected_normals_length
                    + expected_colors_length
                    + expected_uvs_length
                    + expected_triangles_length;

                let buffer_index = self.buffers.len() - 1;
                let buffer_vec: &mut Vec<u8> = self
                    .buffers
                    .get_mut(buffer_index)
                    .expect("failed to get latest buffer");

                let mut buffer_offset = buffer_vec.len();
                buffer_vec.reserve(buffer_capacity);

                if buffer_offset % 4 != 0 {
                    let extra_alloc = 4 - (buffer_offset % 4);
                    buffer_vec.reserve(extra_alloc);
                    buffer_vec
                        .write_all(&vec![0u8; extra_alloc])
                        .expect("failed to align buffer");
                    buffer_offset += extra_alloc;
                }

                let mut min_vertex = Vector3 {
                    x: f32::MAX,
                    y: f32::MAX,
                    z: f32::MAX,
                };
                let mut max_vertex = Vector3 {
                    x: f32::MIN,
                    y: f32::MIN,
                    z: f32::MIN,
                };
                if let Some(vertices) = &geom_data.vertices {
                    for vertex in vertices.iter() {
                        buffer_vec
                            .write_all(&vertex.x.to_le_bytes())
                            .expect("failed to write vertices to buffer");
                        buffer_vec
                            .write_all(&vertex.y.to_le_bytes())
                            .expect("failed to write vertices to buffer");
                        buffer_vec
                            .write_all(&vertex.z.to_le_bytes())
                            .expect("failed to write vertices to buffer");
                        if vertex.x < min_vertex.x {
                            min_vertex.x = vertex.x
                        };
                        if vertex.x > max_vertex.x {
                            max_vertex.x = vertex.x
                        };
                        if vertex.y < min_vertex.y {
                            min_vertex.y = vertex.y
                        };
                        if vertex.y > max_vertex.y {
                            max_vertex.y = vertex.y
                        };
                        if vertex.z < min_vertex.z {
                            min_vertex.z = vertex.z
                        };
                        if vertex.z > max_vertex.z {
                            max_vertex.z = vertex.z
                        };
                    }
                }

                let mut min_normal = Vector3 {
                    x: f32::MAX,
                    y: f32::MAX,
                    z: f32::MAX,
                };
                let mut max_normal = Vector3 {
                    x: f32::MIN,
                    y: f32::MIN,
                    z: f32::MIN,
                };
                if let Some(normals) = &geom_data.normals {
                    for normal in normals.iter() {
                        buffer_vec
                            .write_all(&normal.x.to_le_bytes())
                            .expect("failed to write normals to buffer");
                        buffer_vec
                            .write_all(&normal.y.to_le_bytes())
                            .expect("failed to write normals to buffer");
                        buffer_vec
                            .write_all(&normal.z.to_le_bytes())
                            .expect("failed to write normals to buffer");
                        if normal.x < min_normal.x {
                            min_normal.x = normal.x
                        };
                        if normal.x > max_normal.x {
                            max_normal.x = normal.x
                        };
                        if normal.y < min_normal.y {
                            min_normal.y = normal.y
                        };
                        if normal.y > max_normal.y {
                            max_normal.y = normal.y
                        };
                        if normal.z < min_normal.z {
                            min_normal.z = normal.z
                        };
                        if normal.z > max_normal.z {
                            max_normal.z = normal.z
                        };
                    }
                }

                let mut min_color = Color4 {
                    r: f32::MAX,
                    g: f32::MAX,
                    b: f32::MAX,
                    a: f32::MAX,
                };
                let mut max_color = Color4 {
                    r: f32::MIN,
                    g: f32::MIN,
                    b: f32::MIN,
                    a: f32::MIN,
                };
                if let Some(colors) = &geom_data.vertex_colors {
                    for color in colors.iter() {
                        buffer_vec
                            .write_all(&color.r.to_le_bytes())
                            .expect("failed to write colors to buffer");
                        buffer_vec
                            .write_all(&color.g.to_le_bytes())
                            .expect("failed to write colors to buffer");
                        buffer_vec
                            .write_all(&color.b.to_le_bytes())
                            .expect("failed to write colors to buffer");
                        buffer_vec
                            .write_all(&1.0_f32.to_le_bytes())
                            .expect("failed to write colors to buffer");
                        // buffer_vec
                        //     .write_all(&color.a.to_le_bytes())
                        //     .expect("failed to write colors to buffer");
                        if color.r < min_color.r {
                            min_color.r = color.r
                        };
                        if color.r > max_color.r {
                            max_color.r = color.r
                        };
                        if color.g < min_color.g {
                            min_color.g = color.g
                        };
                        if color.g > max_color.g {
                            max_color.g = color.g
                        };
                        if color.b < min_color.b {
                            min_color.b = color.b
                        };
                        if color.b > max_color.b {
                            max_color.b = color.b
                        };
                        if color.a < min_color.a {
                            min_color.a = color.a
                        };
                        if color.a > max_color.a {
                            max_color.a = color.a
                        };
                    }
                }

                let mut min_uv = TexCoord {
                    u: f32::MAX,
                    v: f32::MAX,
                };
                let mut max_uv = TexCoord {
                    u: f32::MIN,
                    v: f32::MIN,
                };
                if let Some(uv_set) = geom_data.uv_sets.first() {
                    for uv in uv_set.uvs.iter() {
                        buffer_vec
                            .write_all(&uv.u.to_le_bytes())
                            .expect("failed to write uvs to buffer");
                        buffer_vec
                            .write_all(&uv.v.to_le_bytes())
                            .expect("failed to write uvs to buffer");
                        if uv.u < min_uv.u {
                            min_uv.u = uv.u
                        };
                        if uv.u > max_uv.u {
                            max_uv.u = uv.u
                        };
                        if uv.v < min_uv.v {
                            min_uv.v = uv.v
                        };
                        if uv.v > max_uv.v {
                            max_uv.v = uv.v
                        };
                    }
                }

                if let Some(triangles) = &tri_shape_data.triangles {
                    for triangle in triangles.iter() {
                        buffer_vec
                            .write_all(&triangle.a.to_le_bytes())
                            .expect("failed to write triangles to buffer");
                        buffer_vec
                            .write_all(&triangle.b.to_le_bytes())
                            .expect("failed to write triangles to buffer");
                        buffer_vec
                            .write_all(&triangle.c.to_le_bytes())
                            .expect("failed to write triangles to buffer");
                    }
                }

                let buffer_index = json::Index::new(self.buffers.len() as u32 - 1);

                let mut offset_so_far = buffer_offset as u32;
                let positions_accessor_index = match &geom_data.vertices {
                    Some(vertices) => {
                        self.root.buffer_views.push(json::buffer::View {
                            buffer: buffer_index,
                            byte_length: expected_vertices_length as u32,
                            byte_offset: Some(offset_so_far),
                            byte_stride: Some(12),
                            name: Some(format!("{}_Buffer_Vertices", name)),
                            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
                            extensions: Default::default(),
                            extras: Default::default(),
                        });
                        offset_so_far += expected_vertices_length as u32;
                        let buffer_view_index =
                            json::Index::new(self.root.buffer_views.len() as u32 - 1);
                        self.root.accessors.push(json::Accessor {
                            buffer_view: Some(buffer_view_index),
                            byte_offset: 0,
                            count: vertices.len() as u32,
                            component_type: Valid(json::accessor::GenericComponentType(
                                json::accessor::ComponentType::F32,
                            )),
                            extensions: None,
                            extras: Default::default(),
                            type_: Valid(json::accessor::Type::Vec3),
                            min: Some(gltf::json::Value::from(vec![
                                min_vertex.x,
                                min_vertex.y,
                                min_vertex.z,
                            ])),
                            max: Some(gltf::json::Value::from(vec![
                                max_vertex.x,
                                max_vertex.y,
                                max_vertex.z,
                            ])),
                            name: None,
                            normalized: false,
                            sparse: None,
                        });
                        Some(json::Index::new(self.root.accessors.len() as u32 - 1))
                    }
                    None => None,
                };

                let normals_accessor_index = match &geom_data.normals {
                    Some(normals) => {
                        self.root.buffer_views.push(json::buffer::View {
                            buffer: buffer_index,
                            byte_length: expected_normals_length as u32,
                            byte_offset: Some(offset_so_far),
                            byte_stride: Some(12),
                            name: Some(format!("{}_Buffer_Normals", name)),
                            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
                            extensions: Default::default(),
                            extras: Default::default(),
                        });
                        offset_so_far += expected_normals_length as u32;
                        let buffer_view_index =
                            json::Index::new(self.root.buffer_views.len() as u32 - 1);
                        self.root.accessors.push(json::Accessor {
                            buffer_view: Some(buffer_view_index),
                            byte_offset: 0,
                            count: normals.len() as u32,
                            component_type: Valid(json::accessor::GenericComponentType(
                                json::accessor::ComponentType::F32,
                            )),
                            extensions: None,
                            extras: Default::default(),
                            type_: Valid(json::accessor::Type::Vec3),
                            min: Some(gltf::json::Value::from(vec![
                                min_normal.x,
                                min_normal.y,
                                min_normal.z,
                            ])),
                            max: Some(gltf::json::Value::from(vec![
                                max_normal.x,
                                max_normal.y,
                                max_normal.z,
                            ])),
                            name: None,
                            normalized: false,
                            sparse: None,
                        });
                        Some(json::Index::new(self.root.accessors.len() as u32 - 1))
                    }
                    None => None,
                };

                let colors_accessor_index = match &geom_data.vertex_colors {
                    Some(colors) => {
                        self.root.buffer_views.push(json::buffer::View {
                            buffer: buffer_index,
                            byte_length: expected_colors_length as u32,
                            byte_offset: Some(offset_so_far),
                            byte_stride: Some(16),
                            name: Some(format!("{}_Buffer_Colors", name)),
                            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
                            extensions: Default::default(),
                            extras: Default::default(),
                        });
                        offset_so_far += expected_colors_length as u32;
                        let buffer_view_index =
                            json::Index::new(self.root.buffer_views.len() as u32 - 1);
                        self.root.accessors.push(json::Accessor {
                            buffer_view: Some(buffer_view_index),
                            byte_offset: 0,
                            count: colors.len() as u32,
                            component_type: Valid(json::accessor::GenericComponentType(
                                json::accessor::ComponentType::F32,
                            )),
                            extensions: None,
                            extras: Default::default(),
                            type_: Valid(json::accessor::Type::Vec4),
                            min: Some(gltf::json::Value::from(vec![
                                min_color.r,
                                min_color.g,
                                min_color.b,
                                1.0,
                                // min_color.a,
                            ])),
                            max: Some(gltf::json::Value::from(vec![
                                max_color.r,
                                max_color.g,
                                max_color.b,
                                1.0,
                                // max_color.a,
                            ])),
                            name: None,
                            normalized: false,
                            sparse: None,
                        });
                        Some(json::Index::new(self.root.accessors.len() as u32 - 1))
                    }
                    None => None,
                };

                let uvs_accessor_index = match geom_data.uv_sets.first() {
                    Some(uv_set) => {
                        self.root.buffer_views.push(json::buffer::View {
                            buffer: buffer_index,
                            byte_length: expected_uvs_length as u32,
                            byte_offset: Some(offset_so_far),
                            byte_stride: Some(8),
                            name: Some(format!("{}_Buffer_UVs", name)),
                            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
                            extensions: Default::default(),
                            extras: Default::default(),
                        });
                        offset_so_far += expected_uvs_length as u32;
                        let buffer_view_index =
                            json::Index::new(self.root.buffer_views.len() as u32 - 1);
                        self.root.accessors.push(json::Accessor {
                            buffer_view: Some(buffer_view_index),
                            byte_offset: 0,
                            count: uv_set.uvs.len() as u32,
                            component_type: Valid(json::accessor::GenericComponentType(
                                json::accessor::ComponentType::F32,
                            )),
                            extensions: None,
                            extras: Default::default(),
                            type_: Valid(json::accessor::Type::Vec2),
                            min: Some(gltf::json::Value::from(vec![min_uv.u, min_uv.v])),
                            max: Some(gltf::json::Value::from(vec![max_uv.u, max_uv.v])),
                            name: None,
                            normalized: false,
                            sparse: None,
                        });
                        Some(json::Index::new(self.root.accessors.len() as u32 - 1))
                    }
                    None => None,
                };

                let triangles_accessor_index = match &tri_shape_data.triangles {
                    Some(triangles) => {
                        self.root.buffer_views.push(json::buffer::View {
                            buffer: buffer_index,
                            byte_length: expected_triangles_length as u32,
                            byte_offset: Some(offset_so_far),
                            byte_stride: None, //Some(6),
                            name: Some(format!("{}_Buffer_Triangles", name)),
                            target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
                            extensions: Default::default(),
                            extras: Default::default(),
                        });
                        let buffer_view_index =
                            json::Index::new(self.root.buffer_views.len() as u32 - 1);
                        self.root.accessors.push(json::Accessor {
                            buffer_view: Some(buffer_view_index),
                            byte_offset: 0,
                            count: triangles.len() as u32 * 3,
                            component_type: Valid(json::accessor::GenericComponentType(
                                json::accessor::ComponentType::U16,
                            )),
                            extensions: None,
                            extras: Default::default(),
                            type_: Valid(json::accessor::Type::Scalar),
                            min: Some(gltf::json::Value::from(vec![triangles
                                .iter()
                                .map(|t| t.a.min(t.b.min(t.c)))
                                .min()
                                .unwrap()])),
                            max: Some(gltf::json::Value::from(vec![triangles
                                .iter()
                                .map(|t| t.a.max(t.b.max(t.c)))
                                .max()
                                .unwrap()])),
                            name: None,
                            normalized: false,
                            sparse: None,
                        });
                        Some(json::Index::new(self.root.accessors.len() as u32 - 1))
                    }
                    None => None,
                };

                let primitive = json::mesh::Primitive {
                    attributes: {
                        let mut map = BTreeMap::new();
                        if let Some(positions_accessor_index) = positions_accessor_index {
                            map.insert(
                                Valid(json::mesh::Semantic::Positions),
                                positions_accessor_index,
                            );
                        }
                        if let Some(normals_accessor_index) = normals_accessor_index {
                            map.insert(
                                Valid(json::mesh::Semantic::Normals),
                                normals_accessor_index,
                            );
                        }
                        if let Some(colors_accessor_index) = colors_accessor_index {
                            map.insert(
                                Valid(json::mesh::Semantic::Colors(0)),
                                colors_accessor_index,
                            );
                        }
                        if let Some(uvs_accessor_index) = uvs_accessor_index {
                            map.insert(
                                Valid(json::mesh::Semantic::TexCoords(0)),
                                uvs_accessor_index,
                            );
                        }
                        map
                    },
                    extensions: None,
                    extras: Default::default(),
                    indices: triangles_accessor_index,
                    material: material_index,
                    mode: Valid(json::mesh::Mode::Triangles),
                    targets: None,
                };

                self.root.meshes.push(json::Mesh {
                    extensions: None,
                    extras: Default::default(),
                    name: Some(format!(
                        "{}_NiTriShapeData{}",
                        name, tri_shape.base.data_ref.0
                    )),
                    primitives: vec![primitive],
                    weights: None,
                });
                Some(json::Index::new(self.root.meshes.len() as u32 - 1))
            }
            _ => None,
        };

        let node = json::Node {
            camera: None,
            children: None,
            extensions: Default::default(),
            extras: Default::default(),
            matrix: None,
            mesh: mesh_index,
            name: Some(name.into()),
            rotation: Some(rotation_quat_json),
            scale: Some([av_object.scale, av_object.scale, av_object.scale]),
            translation: Some([
                av_object.translation.x,
                av_object.translation.y,
                av_object.translation.z,
            ]),
            skin: None,
            weights: None,
        };

        self.root.nodes.push(node);
        Some(json::Index::new(self.root.nodes.len() as u32 - 1))
    }
}
