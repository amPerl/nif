use super::ni_string::NiString;

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiIntegersExtraData {
    pub name: NiString,
    #[br(temp)]
    #[bw(calc = data.len() as u32)]
    num_integers: u32,
    #[br(count = num_integers)]
    pub data: Vec<u32>,
}
