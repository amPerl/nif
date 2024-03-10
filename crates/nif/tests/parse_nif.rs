use nif::{blocks::Block, error::NifError, header::EndianType, Nif};
use std::io::Cursor;

#[test]
fn nif_1() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/1.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif: Nif = Nif::parse(&mut nif_cursor)?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[10], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_2() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/2.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif: Nif = Nif::parse(&mut nif_cursor)?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[16], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_3() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/3.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };
    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[41], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_4() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/4.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };
    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[3], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_5() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/5.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[12], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_6() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/6.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[15], Block::NiRangeLODData { .. }));

    Ok(())
}

#[test]
fn nif_7() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/7.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(
        nif.blocks[135],
        Block::NiTransformData(nif::blocks::NiTransformData { .. })
    ));

    Ok(())
}

#[test]
fn nif_8() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/8.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[58], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_9() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/9.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[76], Block::NiRangeLODData { .. }));

    Ok(())
}

#[test]
fn nif_10() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/10.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[30], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_11() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/11.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[69], Block::NiIntegerExtraData { .. }));

    Ok(())
}

#[test]
fn nif_12() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/12.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(
        nif.blocks[101],
        Block::NiPSysBoundUpdateModifier { .. }
    ));

    Ok(())
}

#[test]
fn nif_13() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/13.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[21], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_14() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/14.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[38], Block::NiPixelData { .. }));

    Ok(())
}

#[test]
fn nif_15() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/15.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[28], Block::NiPixelData { .. }));

    Ok(())
}

#[test]
fn nif_16() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/16.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[133], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_17() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/17.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[39], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_18() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/18.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[45], Block::NiPixelData { .. }));

    Ok(())
}

#[test]
fn nif_19() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/19.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[309], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_20() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/20.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(
        nif.blocks[89],
        Block::NiPSysBoundUpdateModifier { .. }
    ));

    Ok(())
}

#[test]
fn nif_21() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/21.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[113], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_22() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/22.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[111], Block::NiTriShapeData { .. }));

    Ok(())
}

#[test]
fn nif_23() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/23.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[147], Block::NiNode { .. }));

    Ok(())
}

#[test]
fn nif_24() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/24.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[93], Block::NiRangeLODData { .. }));

    Ok(())
}

#[test]
fn nif_25() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/25.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif_result = Nif::parse(&mut nif_cursor);
    if let Err(NifError::BinReadError(binrw::Error::Custom { pos, err })) = &nif_result {
        if let Some(inner_nif_error) = err.downcast_ref::<NifError>() {
            println!("NifError: at {}: {:?}", pos, inner_nif_error);
        }
    };

    let nif: Nif = nif_result?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[49], Block::NiRangeLODData { .. }));

    Ok(())
}

#[test]
fn nif_26() -> anyhow::Result<()> {
    let nif_buffer = std::fs::read("tests/26.nif")?;
    let mut nif_cursor = Cursor::new(nif_buffer);

    let nif: Nif = Nif::parse(&mut nif_cursor)?;

    assert_eq!(0x14000004, nif.header.version);
    assert_eq!(EndianType::LittleEndian, nif.header.endian_type);

    assert!(matches!(nif.blocks[0], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[1], Block::NiZBufferProperty { .. }));
    assert!(matches!(nif.blocks[2], Block::NiVertexColorProperty { .. }));
    assert!(matches!(nif.blocks[3], Block::NiNode { .. }));
    assert!(matches!(nif.blocks[4], Block::NiTriShape { .. }));
    assert!(matches!(nif.blocks[5], Block::NiMaterialProperty { .. }));
    assert!(matches!(nif.blocks[6], Block::NiTriShapeData { .. }));

    Ok(())
}
