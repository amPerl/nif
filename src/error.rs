use thiserror::Error;

#[derive(Error, Debug)]
pub enum NifError {
    #[error("nif feature \"{0}\" is not implemented")]
    NotImplemented(&'static str),
    #[error("encountered unknown block")]
    UnknownBlock(usize, String),
    #[error("an invalid block type index was specified")]
    InvalidBlockTypeIndex,
    #[error("value is invalid")]
    InvalidValueError,
    #[error("invalid string")]
    StringParseError,
    #[error("binrw error")]
    BinReadError(#[from] binrw::error::Error),
}
