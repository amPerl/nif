use super::NiBoolInterpolator;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiBoolTimelineInterpolator {
    pub base: NiBoolInterpolator,
}

impl std::ops::Deref for NiBoolTimelineInterpolator {
    type Target = NiBoolInterpolator;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
