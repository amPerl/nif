use binrw::{BinRead, BinWrite};

use crate::common::{BlockRef, Color4, TexCoord, Vector3};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiGeometryData {
    pub group_id: i32,
    #[br(temp)]
    #[bw(calc = vertices.as_ref().map(|v| v.len())
        .or_else(|| normals.as_ref().map(|v| v.len()))
        .or_else(|| vertex_colors.as_ref().map(|v| v.len()))
        .or_else(|| uv_sets.first().map(|s| s.uvs.len()))
        .unwrap_or(0) as u16)]
    num_vertices: u16,
    pub keep_flags: u8,
    pub compress_flags: u8,

    #[br(temp)]
    #[bw(calc = u8::from(vertices.is_some()))]
    has_vertices: u8,
    #[br(if(has_vertices != 0), count = num_vertices)]
    pub vertices: Option<Vec<Vector3>>,

    #[bw(map = |x: &u16| (*x & !0x003F) | (uv_sets.len() as u16 & 0x003F))]
    pub data_flags: u16,

    #[br(temp)]
    #[bw(calc = u8::from(normals.is_some()))]
    has_normals: u8,
    #[br(if(has_normals != 0), count = num_vertices)]
    pub normals: Option<Vec<Vector3>>,
    #[br(if(has_normals != 0 && (data_flags & 0xF000) > 0), count = num_vertices)]
    pub tangents: Option<Vec<Vector3>>,
    #[br(if(has_normals != 0 && (data_flags & 0xF000) > 0), count = num_vertices)]
    pub binormals: Option<Vec<Vector3>>,

    pub center: Vector3,
    pub radius: f32,

    #[br(temp)]
    #[bw(calc = u8::from(vertex_colors.is_some()))]
    has_vertex_colors: u8,
    #[br(if(has_vertex_colors != 0), count = num_vertices)]
    pub vertex_colors: Option<Vec<Color4>>,

    #[br(args { count: (data_flags & 0x003F) as _, inner: (num_vertices,) })]
    pub uv_sets: Vec<UvSet>,

    pub consistency_flags: u16,
    pub additional_data_ref: BlockRef,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(num_vertices: u16))]
pub struct UvSet {
    #[br(count = num_vertices)]
    pub uvs: Vec<TexCoord>,
}

impl NiGeometryData {
    pub fn vertex_count(&self) -> usize {
        self.vertices
            .as_ref()
            .map(|v| v.len())
            .or_else(|| self.normals.as_ref().map(|v| v.len()))
            .or_else(|| self.vertex_colors.as_ref().map(|v| v.len()))
            .or_else(|| self.uv_sets.first().map(|s| s.uvs.len()))
            .unwrap_or(0)
    }
    pub fn data_flags(&self) -> u16 {
        self.data_flags
    }
    pub fn uv_set_count(&self) -> u16 {
        self.data_flags() & 0x003F
    }
    pub fn havok_material(&self) -> u16 {
        (self.data_flags() & 0x0FC0) >> 6
    }
    pub fn nbt_method(&self) -> u16 {
        (self.data_flags() & 0xF000) >> 12
    }
}

#[cfg(test)]
mod tests {
    use crate::{blocks::Block, Nif};
    use std::io::Cursor;

    #[test]
    fn uv_set_count_agrees_with_parsed_uv_sets() {
        for n in 1..=26 {
            let bytes = std::fs::read(format!("tests/{}.nif", n)).unwrap();
            let nif = Nif::parse(&mut Cursor::new(bytes)).unwrap();
            for block in &nif.blocks {
                if let Block::NiTriShapeData(d) = block {
                    assert_eq!(d.uv_set_count() as usize, d.uv_sets.len(), "file {}", n);
                    assert_eq!(d.havok_material(), 0, "file {}", n);
                }
            }
        }
    }
}
