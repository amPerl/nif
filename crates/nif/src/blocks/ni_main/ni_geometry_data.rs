use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiString;
use crate::common::{BlockRef, Color4, TexCoord, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiGeometryData {
    pub name: NiString,
    pub num_vertices: u16,
    pub keep_flags: u8,
    pub compress_flags: u8,

    #[br(map = |x: u8| x > 0)]
    pub has_vertices: bool,
    #[br(if(has_vertices), count = num_vertices)]
    pub vertices: Option<Vec<Vector3>>,

    pub num_uv_sets: u8,
    pub tspace_flag: u8,

    #[br(map = |x: u8| x > 0)]
    pub has_normals: bool,
    #[br(if(has_normals), count = num_vertices)]
    pub normals: Option<Vec<Vector3>>,
    #[br(if(has_normals && (tspace_flag & 240 ) > 0), count = num_vertices)]
    pub tangents: Option<Vec<Vector3>>,
    #[br(if(has_normals && (tspace_flag & 240 ) > 0), count = num_vertices)]
    pub binormals: Option<Vec<Vector3>>,

    pub center: Vector3,
    pub radius: f32,

    #[br(map = |x: u8| x > 0)]
    pub has_vertex_colors: bool,
    #[br(if(has_vertex_colors), count = num_vertices)]
    pub vertex_colors: Option<Vec<Color4>>,

    #[br(args { count: (num_uv_sets & 63) as _, inner: (num_vertices,) })]
    pub uv_sets: Vec<UvSet>,

    pub consistency_flags: u16,
    pub additional_data_ref: BlockRef,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(import(num_vertices: u16))]
pub struct UvSet {
    #[br(count = num_vertices)]
    pub uvs: Vec<TexCoord>,
}

impl NiGeometryData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
