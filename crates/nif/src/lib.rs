use binrw::{
    io::{Read, Seek, Write},
    BinRead, BinReaderExt, BinWrite, BinWriterExt,
};
pub use error::NifError;

/// Evaluating animation and orienting billboards is `glam` work, so both live behind the
/// feature rather than shipping a second implementation of the same maths.
#[cfg(feature = "glam")]
pub mod anim;
#[cfg(feature = "glam")]
pub mod billboard;
pub mod blocks;
pub mod common;
pub mod error;
pub mod header;
pub mod walk;

#[cfg(feature = "glam")]
pub mod psys;

#[cfg(feature = "glam")]
pub mod skin;

#[cfg(feature = "facet")]
pub mod reflect;

#[cfg(feature = "glam")]
pub use glam;

mod parse_utils;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct Nif {
    #[bw(args(block_type_names(blocks), block_type_index(blocks)))]
    pub header: header::Header,
    #[br(args(
        header.block_types().iter().map(|b| b.to_string_lossy().into_owned()).collect(),
        header.block_type_index().to_vec(),
    ))]
    #[br(parse_with = parse_utils::parse_blocks)]
    pub blocks: Vec<blocks::Block>,
    pub footer: Footer,
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
pub struct Footer {
    #[br(temp)]
    #[bw(calc = root_refs.len() as u32)]
    num_roots: u32,
    #[br(count = num_roots)]
    pub root_refs: Vec<common::BlockRef>,
}

fn block_type_names(blocks: &[blocks::Block]) -> Vec<blocks::NiString> {
    let mut names: Vec<blocks::NiString> = Vec::new();
    for block in blocks {
        let name = blocks::NiString::from(block.name());
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

fn block_type_index(blocks: &[blocks::Block]) -> Vec<u16> {
    let names = block_type_names(blocks);
    blocks
        .iter()
        .map(|block| {
            let name = blocks::NiString::from(block.name());
            names.iter().position(|n| *n == name).unwrap_or(0) as u16
        })
        .collect()
}

impl Nif {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, NifError> {
        Ok(reader.read_le()?)
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<(), NifError> {
        writer.write_le(self)?;
        Ok(())
    }

    pub fn roots(&self) -> impl Iterator<Item = (usize, &blocks::Block)> {
        self.footer
            .root_refs
            .iter()
            .filter_map(|r| r.index())
            .filter_map(|i| self.blocks.get(i).map(|b| (i, b)))
    }

    pub fn walk(&self) -> walk::Walk<'_> {
        walk::Walk::from_roots(&self.blocks, self.roots().map(|(i, _)| i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn load(n: u32) -> Nif {
        let bytes = std::fs::read(format!("tests/{}.nif", n)).expect("read nif");
        Nif::parse(&mut Cursor::new(bytes)).expect("parse nif")
    }

    #[test]
    fn every_fixture_has_resolvable_roots() {
        for n in 1..=26 {
            let nif = load(n);
            assert!(!nif.footer.root_refs.is_empty(), "file {} has no roots", n);
            assert_eq!(
                nif.roots().count(),
                nif.footer.root_refs.len(),
                "file {} has a root ref pointing outside blocks",
                n
            );
        }
    }

    #[test]
    fn walk_starts_from_the_declared_roots() {
        for n in 1..=26 {
            let nif = load(n);
            let first = nif.walk().next().expect("at least one visit");
            let (root_index, _) = nif.roots().next().expect("at least one root");
            assert_eq!(first.index, root_index, "file {}", n);
            assert_eq!(first.depth, 0);
        }
    }

    #[test]
    fn writing_rebuilds_the_header_type_table() {
        let mut nif = load(1);
        nif.blocks.pop().expect("a block to drop");
        nif.blocks.pop().expect("a block to drop");

        let mut bytes = Vec::new();
        nif.write(&mut Cursor::new(&mut bytes)).expect("write nif");
        let reparsed = Nif::parse(&mut Cursor::new(bytes)).expect("reparse nif");

        assert_eq!(reparsed.blocks.len(), nif.blocks.len());
        assert_eq!(reparsed.header.block_type_index().len(), nif.blocks.len());
        for (before, after) in nif.blocks.iter().zip(&reparsed.blocks) {
            assert_eq!(before.name(), after.name());
        }
        for (i, block) in reparsed.blocks.iter().enumerate() {
            let declared = reparsed.header.block_type_name(i).expect("type name");
            assert_eq!(declared.to_string_lossy(), block.name());
        }
    }

    #[test]
    fn roots_skip_negative_refs() {
        let mut nif = load(1);
        nif.footer.root_refs.push(common::BlockRef::None);
        assert_eq!(nif.roots().count(), nif.footer.root_refs.len() - 1);
    }
}
