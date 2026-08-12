use binrw::{
    io::{Read, Seek, Write},
    BinRead, BinReaderExt, BinWrite, BinWriterExt,
};
pub use error::NifError;

pub mod blocks;
pub mod common;
pub mod error;
pub mod header;
pub mod walk;

#[cfg(feature = "glam")]
pub use glam;

mod parse_utils;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct Nif {
    pub header: header::Header,
    #[br(args(
        header.block_types.iter().map(|b| b.to_string_lossy().into_owned()).collect(),
        header.block_type_index.clone(),
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

impl Nif {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, NifError> {
        Ok(reader.read_le()?)
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<(), NifError> {
        writer.write_le(self)?;
        Ok(())
    }

    pub fn roots(&self) -> impl Iterator<Item = (usize, &blocks::Block)> {
        self.footer.root_refs.iter().filter_map(|r| {
            usize::try_from(r.0)
                .ok()
                .and_then(|i| self.blocks.get(i).map(|b| (i, b)))
        })
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
    fn roots_skip_negative_refs() {
        let mut nif = load(1);
        nif.footer.root_refs.push(common::BlockRef(-1));
        assert_eq!(nif.roots().count(), nif.footer.root_refs.len() - 1);
    }
}
