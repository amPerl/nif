use super::NiBoolInterpolator;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBoolTimelineInterpolator {
    pub base: NiBoolInterpolator,
}

impl std::ops::Deref for NiBoolTimelineInterpolator {
    type Target = NiBoolInterpolator;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
