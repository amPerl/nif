use anyhow::Context;

use crate::{
    blocks::{Block, NiNode, NiTriShape, NiTriShapeData},
    Nif,
};

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<glam::Vec3>,
    pub normals: Vec<glam::Vec3>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn add_nif(&mut self, nif: &Nif, lod_distance: f32) -> anyhow::Result<()> {
        if let Some(Block::NiNode(ni_node)) = nif.blocks.first() {
            self.visit_ni_node(nif, ni_node, None, lod_distance)?;
        }

        Ok(())
    }

    fn visit_ni_node(
        &mut self,
        nif: &Nif,
        ni_node: &NiNode,
        parent_transform: Option<glam::Mat4>,
        lod_distance: f32,
    ) -> anyhow::Result<()> {
        let mut transform = ni_node.transform();

        if let Some(parent_transform) = parent_transform {
            transform = parent_transform * transform;
        }

        for child_ref in ni_node.child_refs.iter().filter(|r| r.0 >= 0) {
            match child_ref
                .get(&nif.blocks)
                .with_context(|| format!("Invalid child ref: {:?}", child_ref))?
            {
                Block::NiNode(ni_node) => {
                    self.visit_ni_node(nif, ni_node, Some(transform), lod_distance)?;
                }
                Block::NiLODNode(ni_lod_node) => {
                    let lod_transform = ni_lod_node.transform();
                    let transform = transform * lod_transform;

                    // if we have lod ranges, find the one that applies
                    if let Some(Block::NiRangeLODData(range_data)) =
                        ni_lod_node.lod_level_data_ref.get(&nif.blocks)
                    {
                        // we only need to select the index, default to whatever is the first
                        let (range_index, _range_data) = range_data
                            .lod_levels
                            .iter()
                            .enumerate()
                            .find(|(_i, range)| {
                                lod_distance >= range.near && lod_distance < range.far
                            })
                            .unwrap_or_else(|| (0, &range_data.lod_levels[0]));
                        let lod_child_ref =
                            ni_lod_node.child_refs.get(range_index).with_context(|| {
                                format!("invalid NiLODNode child index: {:?}", range_index)
                            })?;
                        let lod_child = lod_child_ref
                            .get(&nif.blocks)
                            .with_context(|| format!("Invalid child ref: {:?}", lod_child_ref))?;
                        match lod_child {
                            Block::NiNode(ni_node) => {
                                self.visit_ni_node(nif, ni_node, Some(transform), lod_distance)?;
                            }
                            Block::NiTriShape(ni_tri_shape) => {
                                self.visit_ni_tri_shape(nif, ni_tri_shape, transform)?;
                            }
                            _ => {}
                        }
                    } else {
                        self.visit_ni_node(nif, ni_lod_node, Some(transform), lod_distance)?;
                    }
                }
                Block::NiTriShape(ni_tri_shape) => {
                    self.visit_ni_tri_shape(nif, ni_tri_shape, transform)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn visit_ni_tri_shape(
        &mut self,
        nif: &Nif,
        ni_tri_shape: &NiTriShape,
        parent_transform: glam::Mat4,
    ) -> anyhow::Result<()> {
        let transform = ni_tri_shape.transform();
        let transform = parent_transform * transform;

        let ni_tri_shape_data = ni_tri_shape
            .data_ref
            .get(&nif.blocks)
            .with_context(|| format!("Invalid data ref: {:?}", ni_tri_shape.data_ref))?;

        if let Block::NiTriShapeData(tri_shape_data) = ni_tri_shape_data {
            self.visit_ni_tri_shape_data(tri_shape_data, transform)?;
        } else {
            anyhow::bail!("ni_tri_shape data ref was not NiTriShapeData")
        }

        Ok(())
    }

    fn visit_ni_tri_shape_data(
        &mut self,
        ni_tri_shape_data: &NiTriShapeData,
        parent_transform: glam::Mat4,
    ) -> anyhow::Result<()> {
        if !ni_tri_shape_data.has_vertices
            || !ni_tri_shape_data.has_triangles
            || !ni_tri_shape_data.has_normals
        {
            return Ok(());
        }

        let normal_transform = parent_transform.inverse().transpose();

        let index_offset = self.vertices.len() as u32;

        let vertices: Vec<glam::Vec3> = ni_tri_shape_data
            .vertices
            .as_ref()
            .unwrap()
            .iter()
            .map::<glam::Vec3, _>(|v| v.into())
            .map(|v| parent_transform.transform_point3(v))
            .collect();

        let normals: Vec<glam::Vec3> = ni_tri_shape_data
            .normals
            .as_ref()
            .unwrap()
            .iter()
            .map::<glam::Vec3, _>(|v| v.into())
            .map(|v| normal_transform.transform_vector3(v))
            .collect();

        let indices = ni_tri_shape_data
            .triangles
            .as_ref()
            .unwrap()
            .iter()
            .flat_map(|t| {
                [
                    t.a as u32 + index_offset,
                    t.b as u32 + index_offset,
                    t.c as u32 + index_offset,
                ]
            });

        self.vertices.extend(vertices);
        self.normals.extend(normals);
        self.indices.extend(indices);

        Ok(())
    }
}
