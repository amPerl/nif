use thiserror::Error;

#[derive(Error, Debug)]
pub enum NifError<'a> {
    #[error("nif feature \"{0}\" is not implemented")]
    NotImplemented(&'a str),
    #[error("encountered unknown block")]
    UnknownBlock,
    #[error("an invalid block type index was specified")]
    InvalidBlockTypeIndex,
    #[error("value is invalid")]
    InvalidValueError,
}
