use crate::parse_utils;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiString {
    #[br(parse_with = parse_utils::parse_int_prefixed_string)]
    pub value: String,
}
