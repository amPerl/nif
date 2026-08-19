use binrw::{BinRead, BinWrite};

use crate::blocks::Block;

/// A link to another block. `-1` is the null every file in the corpus uses; any other
/// value is an index, and is written back exactly as it was read.
#[derive(Debug, PartialEq, Eq, BinRead, BinWrite, Clone, Copy, Hash, Default)]
pub enum BlockRef {
    #[brw(magic = -1i32)]
    #[default]
    None,
    Index(u32),
}

impl BlockRef {
    pub fn index(&self) -> Option<usize> {
        match self {
            BlockRef::None => None,
            BlockRef::Index(index) => usize::try_from(*index).ok(),
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, BlockRef::None)
    }

    pub fn get<'b>(&self, blocks: &'b [Block]) -> Option<&'b Block> {
        blocks.get(self.index()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::io::Cursor;

    fn round_trip(bytes: [u8; 4]) -> (BlockRef, [u8; 4]) {
        let parsed = BlockRef::read_le(&mut Cursor::new(bytes)).expect("read");
        let mut written = Vec::new();
        parsed
            .write_le(&mut Cursor::new(&mut written))
            .expect("write");
        (parsed, written.try_into().expect("four bytes"))
    }

    #[test]
    fn minus_one_is_none() {
        let (parsed, written) = round_trip((-1i32).to_le_bytes());
        assert_eq!(parsed, BlockRef::None);
        assert_eq!(written, (-1i32).to_le_bytes());
    }

    #[test]
    fn non_negative_is_an_index() {
        let (parsed, written) = round_trip(7i32.to_le_bytes());
        assert_eq!(parsed, BlockRef::Index(7));
        assert_eq!(parsed.index(), Some(7));
        assert_eq!(written, 7i32.to_le_bytes());
    }

    #[test]
    fn other_negatives_survive_the_round_trip() {
        let (parsed, written) = round_trip((-9i32).to_le_bytes());
        assert!(!parsed.is_none());
        assert_eq!(written, (-9i32).to_le_bytes());
    }
}
