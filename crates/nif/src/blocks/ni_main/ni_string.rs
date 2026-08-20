use crate::parse_utils;
use binrw::{BinRead, BinWrite};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, PartialEq, Clone, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiString {
    #[br(parse_with = parse_utils::parse_int_prefixed_bytes)]
    #[bw(write_with = parse_utils::write_int_prefixed_bytes)]
    pub value: Vec<u8>,
}

impl NiString {
    pub fn as_bytes(&self) -> &[u8] {
        &self.value
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.value)
    }
}

impl fmt::Display for NiString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl From<&str> for NiString {
    fn from(value: &str) -> Self {
        Self {
            value: value.as_bytes().to_vec(),
        }
    }
}
