use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{
    blocks::*,
    common::{Color3, Triangle},
    Nif,
};

#[derive(Default, Debug)]
pub struct ObjMaterial {
    pub name: String,
    pub ambient_color: Color3,
    pub diffuse_color: Color3,
    pub specular_color: Color3,
    pub alpha: f32,
    pub specular_exponent: f32, // glossiness
    pub diffuse_texture_map: Option<String>,
}

#[derive(Default, Debug)]
pub struct ObjMesh {
    pub name: String,
    pub vertices: Vec<glam::Vec3>,
    pub normals: Option<Vec<glam::Vec3>>,
    pub uvs: Option<Vec<glam::Vec2>>,
    pub triangles: Option<Vec<Triangle>>,
    pub material_name: Option<String>,
}

#[derive(Default, Debug)]
pub struct Obj {
    pub materials: HashMap<String, ObjMaterial>,
    pub meshes: Vec<ObjMesh>,
}

impl Obj {
    pub fn write_to_files(&self, obj_path: PathBuf, mtl_path: PathBuf) -> anyhow::Result<()> {
        let mut out_obj = BufWriter::new(File::create(obj_path)?);

        if !self.materials.is_empty() {
            writeln!(
                &mut out_obj,
                "mtllib {}",
                mtl_path
                    .file_name()
                    .expect("invalid mtl filename")
                    .to_str()
                    .expect("invalid mtl filename")
            )?;

            let mut out_mtl = BufWriter::new(File::create(mtl_path)?);

            for material in self.materials.values() {
                writeln!(&mut out_mtl, "newmtl {}", &material.name)?;

                let col_str = |color: &Color3| format!("{} {} {}", color.r, color.g, color.b);

                writeln!(&mut out_mtl, "Ka {}", col_str(&material.ambient_color))?;
                writeln!(&mut out_mtl, "Kd {}", col_str(&material.diffuse_color))?;
                writeln!(&mut out_mtl, "Ks {}", col_str(&material.specular_color))?;

                writeln!(&mut out_mtl, "d {}", &material.alpha)?;
                writeln!(&mut out_mtl, "Ns {}", &material.specular_exponent)?;

                if let Some(diffuse_texture_map) = &material.diffuse_texture_map {
                    writeln!(&mut out_mtl, "map_Kd {}", diffuse_texture_map)?;
                }
            }
        }

        let mut vert_offset = 1;
        let mut uv_offset = 1;
        let mut normal_offset = 1;

        for mesh in self.meshes.iter() {
            writeln!(&mut out_obj, "g {}", mesh.name)?;

            if let Some(material_name) = &mesh.material_name {
                writeln!(&mut out_obj, "usemtl {}", material_name)?;
            }

            for vertex in mesh.vertices.iter() {
                writeln!(&mut out_obj, "v {} {} {}", vertex.x, vertex.y, vertex.z)?;
            }

            if let Some(uvs) = &mesh.uvs {
                for uv in uvs {
                    writeln!(&mut out_obj, "vt {} {}", uv.x, 1.0 - uv.y)?;
                }
            }

            if let Some(normals) = &mesh.normals {
                for normal in normals {
                    writeln!(&mut out_obj, "vn {} {} {}", normal.x, normal.y, normal.z)?;
                }
            }

            if let Some(triangles) = &mesh.triangles {
                for triangle in triangles {
                    match (&mesh.uvs, &mesh.normals) {
                        (None, None) => {
                            writeln!(
                                &mut out_obj,
                                "f {0} {1} {2}",
                                triangle.a as usize + vert_offset,
                                triangle.b as usize + vert_offset,
                                triangle.c as usize + vert_offset
                            )?;
                        }
                        (None, Some(_)) => {
                            writeln!(
                                &mut out_obj,
                                "f {0}//{3} {1}//{4} {2}//{5}",
                                triangle.a as usize + vert_offset,
                                triangle.b as usize + vert_offset,
                                triangle.c as usize + vert_offset,
                                triangle.a as usize + normal_offset,
                                triangle.b as usize + normal_offset,
                                triangle.c as usize + normal_offset
                            )?;
                        }
                        (Some(_), None) => {
                            writeln!(
                                &mut out_obj,
                                "f {0}/{3} {1}/{4} {2}/{5}",
                                triangle.a as usize + vert_offset,
                                triangle.b as usize + vert_offset,
                                triangle.c as usize + vert_offset,
                                triangle.a as usize + uv_offset,
                                triangle.b as usize + uv_offset,
                                triangle.c as usize + uv_offset
                            )?;
                        }
                        (Some(_), Some(_)) => {
                            writeln!(
                                &mut out_obj,
                                "f {0}/{3}/{6} {1}/{4}/{7} {2}/{5}/{8}",
                                triangle.a as usize + vert_offset,
                                triangle.b as usize + vert_offset,
                                triangle.c as usize + vert_offset,
                                triangle.a as usize + uv_offset,
                                triangle.b as usize + uv_offset,
                                triangle.c as usize + uv_offset,
                                triangle.a as usize + normal_offset,
                                triangle.b as usize + normal_offset,
                                triangle.c as usize + normal_offset
                            )?;
                        }
                    };
                }
            }

            vert_offset += mesh.vertices.len();

            if let Some(uvs) = &mesh.uvs {
                uv_offset += uvs.len();
            }

            if let Some(normals) = &mesh.normals {
                normal_offset += normals.len();
            }
        }

        Ok(())
    }

    pub fn visit_nif(&mut self, nif: &Nif, label: Option<String>) {
        let root_node = nif
            .blocks
            .first()
            .expect("nif should have at least a single node");

        match root_node {
            Block::NiNode(ni_node) => {
                self.visit_ni_node(
                    nif,
                    ni_node,
                    None,
                    label.unwrap_or_else(|| "Untitled".into()),
                );
            }
            _ => {
                panic!("root element should be NiNode");
            }
        }
    }

    pub fn visit_ni_node(
        &mut self,
        nif: &Nif,
        ni_node: &NiNode,
        parent_transform: Option<glam::Mat4>,
        label: String,
    ) {
        let mut transform = ni_node.transform();

        if let Some(parent_transform) = parent_transform {
            transform = parent_transform * transform;
        }

        for &child_ref in ni_node.child_refs.iter() {
            let child = child_ref.get(&nif.blocks).expect("invalid child ref");

            match child {
                Block::NiNode(ni_node) => {
                    self.visit_ni_node(
                        nif,
                        ni_node,
                        Some(transform),
                        format!("{}_NiNode{}", label, child_ref.0),
                    );
                }
                Block::NiLODNode(ni_lod_node) => {
                    self.visit_ni_node(
                        nif,
                        &ni_lod_node.base.base,
                        Some(transform),
                        format!("{}_NiLODNode{}", label, child_ref.0),
                    );
                }
                Block::NiTriShape(ni_tri_shape) => {
                    self.visit_tri_shape(
                        nif,
                        ni_tri_shape,
                        transform,
                        format!("{}_NiTriShape{}", label, child_ref.0),
                    );
                }
                _ => {}
            }
        }
    }

    pub fn visit_tri_shape(
        &mut self,
        nif: &Nif,
        tri_shape: &NiTriShape,
        parent_transform: glam::Mat4,
        label: String,
    ) {
        let transform = tri_shape.transform();
        let transform = parent_transform * transform;

        let data = tri_shape
            .data_ref
            .get(&nif.blocks)
            .expect("invalid property ref");

        let obj_shape_data = match data {
            Block::NiTriShapeData(tri_shape_data) => self.visit_tri_shape_data(
                tri_shape_data,
                transform,
                format!("{}_NiTriShapeData{}", label, tri_shape.base.data_ref.0),
            ),
            _ => None,
        };

        if obj_shape_data.is_none() {
            return;
        }

        let mut mesh = obj_shape_data.unwrap();

        let mut diffuse_texture_map = None;
        let mut material = None;

        for &property_ref in tri_shape.property_refs.iter() {
            let property = property_ref.get(&nif.blocks).expect("invalid property ref");

            match property {
                Block::NiMaterialProperty(material_property) => {
                    material = Some(ObjMaterial {
                        name: format!("{}_NiMaterialProperty{}", label, property_ref.0),
                        ambient_color: material_property.color_ambient,
                        diffuse_color: material_property.color_diffuse,
                        specular_color: material_property.color_specular,
                        specular_exponent: material_property.glossiness,
                        alpha: material_property.alpha,
                        diffuse_texture_map: None,
                    });
                }
                Block::NiTexturingProperty(texturing_property) => {
                    if let Some(texture_path) =
                        self.visit_texturing_property(nif, texturing_property)
                    {
                        diffuse_texture_map = Some(texture_path);
                    }
                }
                _ => {}
            };
        }

        if let Some(mut material) = material {
            mesh.material_name = Some(material.name.clone());

            if let Some(diffuse_texture_map) = diffuse_texture_map {
                material.diffuse_texture_map = Some(diffuse_texture_map);
            }

            self.materials.insert(material.name.clone(), material);
        }

        self.meshes.push(mesh);
    }

    pub fn visit_tri_shape_data(
        &mut self,
        tri_shape_data: &NiTriShapeData,
        parent_transform: glam::Mat4,
        label: String,
    ) -> Option<ObjMesh> {
        let geometry_data = &tri_shape_data.base.base;
        if !geometry_data.has_vertices {
            return None;
        }

        Some(ObjMesh {
            name: label,
            vertices: geometry_data
                .vertices
                .as_ref()
                .unwrap()
                .iter()
                .map::<glam::Vec3, _>(|v| v.into())
                .map(|v| parent_transform.transform_point3(v))
                .collect(),
            normals: geometry_data
                .normals
                .as_ref()
                .map(|normals| normals.iter().map(|v| v.into()).collect()),
            triangles: tri_shape_data.triangles.clone(),
            uvs: geometry_data
                .uv_sets
                .first()
                .map(|uv_set| uv_set.uvs.iter().map(|v| v.into()).collect()),
            material_name: None,
        })
    }

    pub fn visit_texturing_property(
        &mut self,
        nif: &Nif,
        texturing_property: &NiTexturingProperty,
    ) -> Option<String> {
        if !texturing_property.has_base_texture || texturing_property.base_texture.is_none() {
            return None;
        }

        let diffuse_tex_desc = texturing_property.base_texture.as_ref().unwrap();

        let diffuse_source = diffuse_tex_desc
            .source_ref
            .get(&nif.blocks)
            .expect("invalid property ref");

        match diffuse_source {
            Block::NiSourceTexture(source_texture) => self.visit_source_texture(source_texture),
            _ => None,
        }
    }

    pub fn visit_source_texture(&mut self, source_texture: &NiSourceTexture) -> Option<String> {
        if source_texture.use_external == 0 {
            return None;
        }

        Some(
            source_texture
                .file_name
                .value
                .replace(".tga", ".dds")
                .replace(".TGA", ".dds")
                .replace(".bmp", ".dds")
                .replace(".BMP", ".dds"),
        )
    }
}
