use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiMaterialColorController {
    pub base: NiSingleInterpController,
    pub target_color: MaterialColor,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum MaterialColor {
    #[brw(magic = 0u16)]
    Ambient,
    #[brw(magic = 1u16)]
    Diffuse,
    #[brw(magic = 2u16)]
    Specular,
    #[brw(magic = 3u16)]
    SelfIllum,
    Unknown(u16),
}
