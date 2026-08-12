
use super::NiTriBasedGeomData;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiTriStripsData {
    pub base: NiTriBasedGeomData,

    #[br(temp)]
    #[bw(calc = strip_lengths.len() as u16)]
    num_strips: u16,
    #[br(count = num_strips)]
    pub strip_lengths: Vec<u16>,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub has_points: bool,
    #[br(if(has_points), count = strip_lengths.iter().map(|l| *l as usize).sum::<usize>())]
    pub points: Option<Vec<u16>>,
}

impl std::ops::Deref for NiTriStripsData {
    type Target = NiTriBasedGeomData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
