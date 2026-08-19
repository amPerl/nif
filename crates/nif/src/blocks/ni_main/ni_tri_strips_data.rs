
use super::NiTriBasedGeomData;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiTriStripsData {
    #[bw(args(base.num_triangles))]
    pub base: NiTriBasedGeomData,

    #[br(temp)]
    #[bw(calc = strip_lengths.len() as u16)]
    num_strips: u16,
    #[br(count = num_strips)]
    pub strip_lengths: Vec<u16>,
    #[br(temp)]
    #[bw(calc = u8::from(points.is_some()))]
    has_points: u8,
    #[br(if(has_points != 0), count = strip_lengths.iter().map(|l| *l as usize).sum::<usize>())]
    pub points: Option<Vec<u16>>,
}

impl std::ops::Deref for NiTriStripsData {
    type Target = NiTriBasedGeomData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
