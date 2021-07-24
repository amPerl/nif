use std::io::SeekFrom;

use binread::{
    io::{Read, Seek},
    BinRead, BinResult, ReadOptions,
};

use crate::error::NifError;

use super::blocks::{Block, *};

pub fn parse_version<R: Read + Seek>(
    reader: &mut R,
    options: &ReadOptions,
    a: (),
) -> BinResult<u32> {
    let version_str = parse_lf_terminated_string(reader, options, a)?;
    let version_split: Vec<u32> = version_str
        .split('.')
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect::<Vec<u32>>();
    let version: u32 =
        version_split[0] << 24 | version_split[1] << 16 | version_split[2] << 8 | version_split[3];
    Ok(version)
}

pub fn parse_lf_terminated_string<R: Read + Seek>(
    reader: &mut R,
    _options: &ReadOptions,
    _: (),
) -> BinResult<String> {
    Ok(String::from_utf8_lossy(
        reader
            .bytes()
            .filter_map(Result::ok)
            .take_while(|&b| b != b'\n')
            .collect::<Vec<u8>>()
            .as_slice(),
    )
    .to_string())
}

pub fn parse_int_prefixed_string<R: Read + Seek>(
    reader: &mut R,
    options: &ReadOptions,
    _: (),
) -> BinResult<String> {
    let count = u32::read_options(reader, options, ())?;

    Ok(String::from_utf8_lossy(
        reader
            .bytes()
            .take(count as usize)
            .filter_map(Result::ok)
            .collect::<Vec<u8>>()
            .as_slice(),
    )
    .to_string())
}

pub fn parse_blocks<R: Read + Seek>(
    reader: &mut R,
    options: &ReadOptions,
    args: (Vec<String>, Vec<u16>),
) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::new();

    for block_type_index in args.1 {
        match args.0.get(block_type_index as usize) {
            Some(block_type) => {
                // println!(
                //     "Reading block {} at {}",
                //     block_type,
                //     reader.seek(SeekFrom::Current(0))?
                // );

                let block = match block_type.as_ref() {
                    "NiObjectNET" => {
                        Block::NiObjectNET(NiObjectNET::read_options(reader, options, ())?)
                    }
                    "NiAvObject" => {
                        Block::NiAvObject(NiAvObject::read_options(reader, options, ())?)
                    }
                    "NiNode" => Block::NiNode(NiNode::read_options(reader, options, ())?),
                    "NiZBufferProperty" => Block::NiZBufferProperty(
                        NiZBufferProperty::read_options(reader, options, ())?,
                    ),
                    "NiVertexColorProperty" => Block::NiVertexColorProperty(
                        NiVertexColorProperty::read_options(reader, options, ())?,
                    ),
                    "NiTriShape" => {
                        Block::NiTriShape(NiTriShape::read_options(reader, options, ())?)
                    }
                    "NiStringExtraData" => Block::NiStringExtraData(
                        NiStringExtraData::read_options(reader, options, ())?,
                    ),
                    "NiTexturingProperty" => Block::NiTexturingProperty(
                        NiTexturingProperty::read_options(reader, options, ())?,
                    ),
                    "NiSourceTexture" => {
                        Block::NiSourceTexture(NiSourceTexture::read_options(reader, options, ())?)
                    }
                    "NiAlphaProperty" => {
                        Block::NiAlphaProperty(NiAlphaProperty::read_options(reader, options, ())?)
                    }
                    "NiMaterialProperty" => Block::NiMaterialProperty(
                        NiMaterialProperty::read_options(reader, options, ())?,
                    ),
                    "NiTriShapeData" => {
                        Block::NiTriShapeData(NiTriShapeData::read_options(reader, options, ())?)
                    }
                    "NiIntegerExtraData" => Block::NiIntegerExtraData(
                        NiIntegerExtraData::read_options(reader, options, ())?,
                    ),
                    "NiSpecularProperty" => Block::NiSpecularProperty(
                        NiSpecularProperty::read_options(reader, options, ())?,
                    ),
                    "NiSwitchNode" => {
                        Block::NiSwitchNode(NiSwitchNode::read_options(reader, options, ())?)
                    }
                    "NiLODNode" => Block::NiLODNode(NiLODNode::read_options(reader, options, ())?),
                    "NiRangeLODData" => {
                        Block::NiRangeLODData(NiRangeLODData::read_options(reader, options, ())?)
                    }
                    "NiBillboardNode" => {
                        Block::NiBillboardNode(NiBillboardNode::read_options(reader, options, ())?)
                    }
                    "NiCollisionData" => {
                        Block::NiCollisionData(NiCollisionData::read_options(reader, options, ())?)
                    }
                    "NiStencilProperty" => Block::NiStencilProperty(
                        NiStencilProperty::read_options(reader, options, ())?,
                    ),
                    _ => {
                        return Err(binread::Error::Custom {
                            pos: reader.seek(SeekFrom::Current(0))?,
                            err: Box::new(NifError::UnknownBlock),
                        });
                    }
                };
                blocks.push(block);
            }
            None => {
                return Err(binread::Error::Custom {
                    pos: reader.seek(SeekFrom::Current(0))?,
                    err: Box::new(NifError::InvalidBlockTypeIndex),
                });
            }
        }
    }
    // println!("Finished reading at {}", reader.seek(SeekFrom::Current(0))?);

    Ok(blocks)
}
