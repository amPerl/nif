use binrw::{BinRead, BinWrite};

use crate::{blocks::NiString, common::Vector3};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiMorphData {
    #[br(temp)]
    #[bw(calc = morphs.len() as u32)]
    num_morphs: u32,
    #[bw(map = |x: &u32| morphs.first().map_or(*x, |m| m.vectors.len() as u32))]
    pub num_vertices: u32,
    pub relative_targets: u8,
    #[br(args { count: num_morphs as _, inner: (num_vertices,) })]
    pub morphs: Vec<Morph>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(num_vertices: u32))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Morph {
    pub frame_name: NiString,
    pub legacy_weight: f32,
    #[br(count = num_vertices)]
    pub vectors: Vec<Vector3>,
}
