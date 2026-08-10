use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiMaterialColorController {
    pub base: NiSingleInterpController,
    pub target_color: MaterialColor,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum MaterialColor {
    #[br(magic = 0u16)]
    Ambient,
    #[br(magic = 1u16)]
    Diffuse,
    #[br(magic = 2u16)]
    Specular,
    #[br(magic = 3u16)]
    SelfIllum,
}
