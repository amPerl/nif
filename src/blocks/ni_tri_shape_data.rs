use crate::error::NifError;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
#[br(assert(vector_flags == 1, NifError::NotImplemented("VectorFlags")))]
pub struct NiTriShapeData {
    pub group_id: u32,
    pub num_vertices: u16,
    pub keep_flags: u8,
    pub compress_flags: u8,

    #[br(map = |x: u8| x > 0)]
    pub has_vertices: bool,
    #[br(if(has_vertices))]
    #[br(count=num_vertices)]
    pub vertices: Option<Vec<Vector3>>,
    pub vector_flags: u16,

    #[br(map = |x: u8| x > 0)]
    pub has_normals: bool,
    #[br(if(has_normals))]
    #[br(count=num_vertices)]
    pub normals: Option<Vec<Vector3>>,
    pub center: Vector3,
    pub radius: f32,

    #[br(map = |x: u8| x > 0)]
    pub has_vertex_colors: bool,
    #[br(if(has_vertex_colors))]
    #[br(count=num_vertices)]
    pub vertex_colors: Option<Vec<Color4>>,

    #[br(count=num_vertices)]
    pub uv_sets: Vec<Vector2>,

    pub consistency_flags: u16,

    #[br(assert(additional_data_ref >= -1, NifError::InvalidValueError))]
    pub additional_data_ref: i32,

    pub num_triangles: u16,
    pub num_triangle_points: u32,
    #[br(map = |x: u8| x > 0)]
    pub has_triangles: bool,
    #[br(if(has_triangles))]
    #[br(count=num_triangles)]
    pub triangles: Option<Vec<Triangle>>,

    pub num_match_groups: u16,
    #[br(count=num_match_groups)]
    pub match_groups: Vec<MatchGroup>,
}
#[derive(Debug, PartialEq, BinRead)]
pub struct MatchGroup {
    pub num_vertices: u16,
    #[br(count=num_vertices)]
    pub vertex_indices: Vec<u16>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Triangle {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

impl NiTriShapeData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
