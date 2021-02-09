use binread::io::cursor::Cursor;
use nif::{blocks::Block, header::EndianType, Nif};

#[test]
fn nif_a() -> anyhow::Result<()> {
    let nif_buffer = include_bytes!("a.nif");
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif: Nif = Nif::parse(&mut nif_cursor)?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[1], Block::NiZBufferProperty { .. }));
    assert!(matches!(nif.blocks[2], Block::NiVertexColorProperty { .. }));
    assert!(matches!(nif.blocks[3], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[4], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[5], Block::NiZBufferProperty { .. }));
    assert!(matches!(nif.blocks[6], Block::NiTexturingProperty { .. }));
    assert!(matches!(nif.blocks[7], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[8], Block::NiAlphaProperty { .. }));
    assert!(matches!(nif.blocks[9], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[10], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_b() -> anyhow::Result<()> {
    let nif_buffer = include_bytes!("b.nif");
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif: Nif = Nif::parse(&mut nif_cursor)?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[1], Block::NiZBufferProperty { .. }));
    assert!(matches!(nif.blocks[2], Block::NiVertexColorProperty { .. }));
    assert!(matches!(nif.blocks[3], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[4], Block::NiIntegerExtraData { .. }));
    assert!(matches!(nif.blocks[5], Block::NiTexturingProperty { .. }));
    assert!(matches!(nif.blocks[6], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[7], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[8], Block::NiAlphaProperty { .. }));
    assert!(matches!(nif.blocks[9], Block::NiSpecularProperty { .. }));
    assert!(matches!(nif.blocks[10], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[11], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[12], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[13], Block::NiTexturingProperty { .. }));
    assert!(matches!(nif.blocks[14], Block::NiAlphaProperty { .. }));
    assert!(matches!(nif.blocks[15], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[16], Block::NiTriShapeData { .. }));

    Ok(())
}
