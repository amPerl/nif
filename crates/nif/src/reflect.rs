//! Finds the `BlockRef` values inside a block. Requires the `facet` feature.
//!
//! Descends structs, the active variant of an enum, lists, options and pointers.

use facet::Facet;
use facet_reflect::Peek;

use crate::blocks::Block;
use crate::common::BlockRef;

/// A reference and the field names leading to it.
#[derive(Debug, PartialEq)]
pub struct FoundRef {
    pub path: Vec<String>,
    pub reference: BlockRef,
}

/// Every `BlockRef` the block holds, in field order.
///
/// Not generic, so `Facet` stays out of the public API.
pub fn refs(block: &Block) -> Vec<FoundRef> {
    walk_value(block)
}

fn walk_value<'a, T: Facet<'a>>(value: &'a T) -> Vec<FoundRef> {
    let mut found = Vec::new();
    let mut path: Vec<String> = Vec::new();
    walk(Peek::new(value), &mut path, &mut found);
    found
}

fn walk(peek: Peek<'_, '_>, path: &mut Vec<String>, found: &mut Vec<FoundRef>) {
    if let Ok(reference) = peek.get::<BlockRef>() {
        found.push(FoundRef {
            path: path.clone(),
            reference: *reference,
        });
        return;
    }

    if let Ok(option) = peek.into_option() {
        if let Some(inner) = option.value() {
            walk(inner, path, found);
        }
        return;
    }
    if let Ok(list) = peek.into_list_like() {
        for item in list.iter() {
            walk(item, path, found);
        }
        return;
    }
    if let Ok(pointer) = peek.into_pointer() {
        if let Some(inner) = pointer.borrow_inner() {
            walk(inner, path, found);
        }
        return;
    }
    if let Ok(structure) = peek.into_struct() {
        for (index, field) in structure.ty().fields.iter().enumerate() {
            if let Ok(value) = structure.field(index) {
                path.push(field.name.to_string());
                walk(value, path, found);
                path.pop();
            }
        }
        return;
    }
    if let Ok(variant) = peek.into_enum() {
        if let Ok(active) = variant.active_variant() {
            for (index, field) in active.data.fields.iter().enumerate() {
                if let Ok(Some(value)) = variant.field(index) {
                    path.push(field.name.to_string());
                    walk(value, path, found);
                    path.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{NiObjectNET, NiSourceTexture, NiString};

    fn object_net(controller: BlockRef, extra: Vec<BlockRef>) -> NiObjectNET {
        NiObjectNET {
            name: NiString { value: Vec::new() },
            extra_data_refs: extra,
            controller_ref: controller,
        }
    }

    #[test]
    fn finds_refs_behind_a_base_struct_and_a_vec() {
        let base = object_net(
            BlockRef::Index(7),
            vec![BlockRef::Index(1), BlockRef::Index(2)],
        );
        let found = walk_value(&base);
        let names: Vec<_> = found
            .iter()
            .map(|f| (f.path.clone(), f.reference))
            .collect();
        assert_eq!(
            names,
            vec![
                (vec!["extra_data_refs".to_string()], BlockRef::Index(1)),
                (vec!["extra_data_refs".to_string()], BlockRef::Index(2)),
                (vec!["controller_ref".to_string()], BlockRef::Index(7)),
            ]
        );
    }

    #[test]
    fn finds_refs_nested_two_levels_down() {
        let texture = NiSourceTexture {
            base: object_net(BlockRef::Index(3), Vec::new()),
            use_external: false,
            file_name: NiString { value: Vec::new() },
            unknown_link_ref: None,
            pixel_data_ref: BlockRef::Index(9),
            pixel_layout: crate::blocks::PixelLayout::Default,
            mipmap_format: crate::blocks::MipMapFormat::Default,
            alpha_format: crate::blocks::AlphaFormat::None,
            is_static: 1,
            direct_render: false,
        };
        let found = walk_value(&texture);
        assert_eq!(
            found,
            vec![
                FoundRef {
                    path: vec!["base".into(), "controller_ref".into()],
                    reference: BlockRef::Index(3),
                },
                FoundRef {
                    path: vec!["pixel_data_ref".into()],
                    reference: BlockRef::Index(9),
                },
            ]
        );
    }

    #[test]
    fn finds_every_ref_in_a_fixture() {
        use crate::Nif;

        let bytes = include_bytes!("../tests/1.nif");
        let nif = Nif::parse(&mut binrw::io::Cursor::new(&bytes[..])).expect("parses");

        let mut total = 0;
        let mut named = std::collections::BTreeSet::new();
        for block in &nif.blocks {
            for found in refs(block) {
                total += 1;
                if let Some(last) = found.path.last() {
                    named.insert(last.clone());
                }
            }
        }
        assert!(total > 0, "no refs found");

        for expected in ["controller_ref", "data_ref", "source_ref"] {
            assert!(
                named.contains(expected),
                "{expected} missing, found {named:?}"
            );
        }

        let has_geometry = nif
            .blocks
            .iter()
            .any(|b| matches!(b, Block::NiTriShape(_) | Block::NiTriStrips(_)));
        assert!(has_geometry);
    }

    #[test]
    fn a_null_ref_is_still_reported() {
        let base = object_net(BlockRef::None, Vec::new());
        let found = walk_value(&base);
        assert_eq!(found.len(), 1);
        assert!(found[0].reference.is_none());
    }
}
