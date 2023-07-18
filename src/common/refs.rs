use binrw::BinRead;

use crate::blocks::Block;

#[derive(Debug, PartialEq, BinRead, Clone, Copy, Hash)]
pub struct BlockRef(pub i32);

impl BlockRef {
    pub fn get<'b>(&self, blocks: &'b [Block]) -> Option<&'b Block> {
        if self.0 < 0 {
            None
        } else {
            blocks.get(self.0 as usize)
        }
    }
}
