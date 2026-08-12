use super::ni_string::NiString;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct NiFloatsExtraData {
    pub name: NiString,
    #[br(temp)]
    #[bw(calc = data.len() as u32)]
    num_floats: u32,
    #[br(count = num_floats)]
    pub data: Vec<f32>,
}
