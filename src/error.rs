use thiserror::Error;

#[derive(Error, Debug)]
pub enum NifError {
    #[error("nif feature \"{0}\" is not implemented")]
    NotImplemented(&'static str),
    #[error("encountered unknown block {0} of type \"{1}\"")]
    UnknownBlock(usize, String),
    #[error("an invalid block type index was specified")]
    InvalidBlockTypeIndex,
    #[error("value is invalid")]
    InvalidValueError,
    #[error("invalid string")]
    StringParseError,
    #[error("block {index} ({block_type}) at offset {offset}, previous block started at {previous_offset}: {detail}")]
    BlockParse {
        index: usize,
        block_type: String,
        offset: u64,
        previous_offset: u64,
        detail: String,
    },
    #[error("binrw error: {0}")]
    BinReadError(#[from] binrw::error::Error),
}
