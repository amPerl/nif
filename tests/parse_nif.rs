use binread::io::cursor::Cursor;
use nif::{blocks::Block, error::NifError, header::EndianType, Nif};

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

#[test]
fn nif_c() -> anyhow::Result<()> {
    let nif_buffer = include_bytes!("c.nif");
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    match &nif_result {
        Err(NifError::BinReadError(binread::Error::Custom { pos, err })) => {
            match err.downcast_ref::<NifError>() {
                Some(inner_nif_error) => {
                    println!("NifError: at {}: {:?}", pos, inner_nif_error);
                }
                None => (),
            }
        }
        _ => (),
    };
    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[1], Block::NiZBufferProperty { .. }));
    assert!(matches!(nif.blocks[2], Block::NiVertexColorProperty { .. }));
    assert!(matches!(nif.blocks[3], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[4], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[5], Block::NiTexturingProperty { .. }));
    assert!(matches!(nif.blocks[6], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[7], Block::NiAlphaProperty { .. }));
    assert!(matches!(nif.blocks[8], Block::NiSpecularProperty { .. }));
    assert!(matches!(nif.blocks[9], Block::NiVertexColorProperty { .. }));
    assert!(matches!(nif.blocks[10], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[11], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[12], Block::NiLODNode { .. }));
    assert!(matches!(nif.blocks[13], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[14], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[15], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[16], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[17], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[18], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[19], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[20], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[21], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[22], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[23], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[24], Block::NiRangeLODData { .. }));
    assert!(matches!(nif.blocks[25], Block::NiBillboardNode { .. }));
    assert!(matches!(nif.blocks[26], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[27], Block::NiZBufferProperty { .. }));
    assert!(matches!(nif.blocks[28], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[29], Block::NiTexturingProperty { .. }));
    assert!(matches!(nif.blocks[30], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[31], Block::NiAlphaProperty { .. }));
    assert!(matches!(nif.blocks[32], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[33], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[34], Block::NiBillboardNode { .. }));
    assert!(matches!(nif.blocks[35], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[36], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[37], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[38], Block::NiBillboardNode { .. }));
    assert!(matches!(nif.blocks[39], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[40], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[41], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_d() -> anyhow::Result<()> {
    let nif_buffer = include_bytes!("d.nif");
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    match &nif_result {
        Err(NifError::BinReadError(binread::Error::Custom { pos, err })) => {
            match err.downcast_ref::<NifError>() {
                Some(inner_nif_error) => {
                    println!("NifError: at {}: {:?}", pos, inner_nif_error);
                }
                None => (),
            }
        }
        _ => (),
    };
    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[1], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[2], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[3], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_e() -> anyhow::Result<()> {
    let nif_buffer = include_bytes!("e.nif");
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    match &nif_result {
        Err(NifError::BinReadError(binread::Error::Custom { pos, err })) => {
            match err.downcast_ref::<NifError>() {
                Some(inner_nif_error) => {
                    println!("NifError: at {}: {:?}", pos, inner_nif_error);
                }
                None => (),
            }
        }
        _ => (),
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[1], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[2], Block::NiIntegerExtraData { .. }));
    assert!(matches!(nif.blocks[3], Block::NiIntegerExtraData { .. }));
    assert!(matches!(nif.blocks[4], Block::NiIntegerExtraData { .. }));
    assert!(matches!(nif.blocks[5], Block::NiTexturingProperty { .. }));
    assert!(matches!(nif.blocks[6], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[7], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[8], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[9], Block::NiAlphaProperty { .. }));
    assert!(matches!(nif.blocks[10], Block::NiVertexColorProperty { .. }));
    assert!(matches!(nif.blocks[11], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[12], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_f() -> anyhow::Result<()> {
    let nif_buffer = include_bytes!("f.nif");
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    match &nif_result {
        Err(NifError::BinReadError(binread::Error::Custom { pos, err })) => {
            match err.downcast_ref::<NifError>() {
                Some(inner_nif_error) => {
                    println!("NifError: at {}: {:?}", pos, inner_nif_error);
                }
                None => (),
            }
        }
        _ => (),
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[1], Block::NiZBufferProperty { .. }));
    assert!(matches!(nif.blocks[2], Block::NiVertexColorProperty { .. }));
    assert!(matches!(nif.blocks[3], Block::NiCollisionData { .. }));
    assert!(matches!(nif.blocks[4], Block::NiLODNode { .. }));
    assert!(matches!(nif.blocks[5], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[6], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[7], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[8], Block::NiTexturingProperty { .. }));
    assert!(matches!(nif.blocks[9], Block::NiSourceTexture { .. }));
    assert!(matches!(nif.blocks[10], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[11], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[12], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[13], Block::NiStringExtraData { .. }));
    assert!(matches!(nif.blocks[14], Block::NiTriShapeData { .. }));
    assert!(matches!(nif.blocks[15], Block::NiRangeLODData { .. }));

    Ok(())
}
