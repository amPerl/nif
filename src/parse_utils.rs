use super::blocks::{Block, *};
use super::common;
use crate::error::NifError;
use binrw::{io::Read, BinRead, BinWrite, BinResult};

#[binrw::parser(reader, endian)]
pub fn parse_keys<T>(
    num_keys: u32,
    key_type: Option<common::KeyType>,
) -> BinResult<Vec<common::Key<T>>>
where
    T: BinRead + BinWrite,
    T: for<'a> BinRead<Args<'a> = ()>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    if num_keys == 0 {
        return Ok(Vec::new());
    }
    let Some(key_type) = key_type else {
        return Err(binrw::Error::Custom {
            pos: reader.stream_position()?,
            err: Box::new(NifError::InvalidValueError),
        });
    };

    let mut keys = Vec::new();
    for _ in 0..num_keys {
        let key = common::Key::read_options(reader, endian, (key_type,))?;
        keys.push(key);
    }

    Ok(keys)
}

#[binrw::parser(reader, endian)]
pub fn parse_quat_keys(
    num_keys: u32,
    key_type: Option<common::KeyType>,
) -> BinResult<Vec<common::QuatKey>> {
    if num_keys == 0 {
        return Ok(Vec::new());
    }
    let Some(key_type) = key_type else {
        return Err(binrw::Error::Custom {
            pos: reader.stream_position()?,
            err: Box::new(NifError::InvalidValueError),
        });
    };
    if key_type == common::KeyType::XyzRotation {
        return Ok(Vec::new());
    }

    let mut keys = Vec::new();
    for _ in 0..num_keys {
        let key = common::QuatKey::read_options(reader, endian, (key_type,))?;
        keys.push(key);
    }

    Ok(keys)
}

#[binrw::parser(reader, endian)]
pub fn parse_version() -> BinResult<u32> {
    let version_str = parse_lf_terminated_string(reader, endian, ())?;
    let version_split: Vec<u32> = version_str
        .split('.')
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect::<Vec<u32>>();
    let [major, minor, patch, build] = version_split[..] else {
        return Err(binrw::Error::Custom {
            pos: reader.stream_position()?,
            err: Box::new(NifError::StringParseError),
        });
    };
    Ok((major & 0xFF) << 24 | (minor & 0xFF) << 16 | (patch & 0xFF) << 8 | (build & 0xFF))
}

#[binrw::writer(writer)]
pub fn write_version(version: &u32) -> BinResult<()> {
    let text = format!(
        "{}.{}.{}.{}\n",
        version >> 24 & 0xFF,
        version >> 16 & 0xFF,
        version >> 8 & 0xFF,
        version & 0xFF
    );
    writer.write_all(text.as_bytes())?;
    Ok(())
}

const MAX_LF_TERMINATED_STRING_LEN: usize = 64;

#[binrw::parser(reader)]
pub fn parse_lf_terminated_string() -> BinResult<String> {
    let pos = reader.stream_position()?;
    let mut bytes = Vec::new();
    let mut byte = [0u8; 1];

    loop {
        reader.read_exact(&mut byte)?;
        if byte[0] == b'\n' {
            return Ok(String::from_utf8_lossy(&bytes).into_owned());
        }
        if bytes.len() == MAX_LF_TERMINATED_STRING_LEN {
            return Err(binrw::Error::Custom {
                pos,
                err: Box::new(NifError::StringParseError),
            });
        }
        bytes.push(byte[0]);
    }
}

#[binrw::parser(reader, endian)]
pub fn parse_int_prefixed_bytes() -> BinResult<Vec<u8>> {
    let pos = reader.stream_position()?;
    let count = u32::read_options(reader, endian, ())?;

    let mut bytes = Vec::new();
    let read = reader.take(count.into()).read_to_end(&mut bytes)?;

    if read as u64 != u64::from(count) {
        return Err(binrw::Error::Custom {
            pos,
            err: Box::new(NifError::StringParseError),
        });
    }

    Ok(bytes)
}

#[binrw::writer(writer, endian)]
#[allow(clippy::ptr_arg)]
pub fn write_int_prefixed_bytes(value: &Vec<u8>) -> BinResult<()> {
    let Ok(count) = u32::try_from(value.len()) else {
        return Err(binrw::Error::Custom {
            pos: writer.stream_position()?,
            err: Box::new(NifError::StringParseError),
        });
    };

    count.write_options(writer, endian, ())?;
    writer.write_all(value)?;
    Ok(())
}

const MAX_PREALLOCATED_BLOCKS: usize = 8192;

#[binrw::parser(reader, endian)]
pub fn parse_blocks(strings: Vec<String>, block_type_indices: Vec<u16>) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::with_capacity(block_type_indices.len().min(MAX_PREALLOCATED_BLOCKS));
    let mut previous_offset = 0u64;

    for block_type_index in block_type_indices {
        match strings.get(block_type_index as usize) {
            Some(block_type) => {
                let offset = reader.stream_position()?;
                let index = blocks.len();
                let parsed =
                    (|| -> BinResult<Block> {
                        Ok(match block_type.as_ref() {
                            "NiObjectNET" => {
                                Block::NiObjectNET(NiObjectNET::read_options(reader, endian, ())?)
                            }
                            "NiAvObject" => {
                                Block::NiAvObject(NiAvObject::read_options(reader, endian, ())?)
                            }
                            "NiNode" => Block::NiNode(NiNode::read_options(reader, endian, ())?),
                            "NiZBufferProperty" => Block::NiZBufferProperty(
                                NiZBufferProperty::read_options(reader, endian, ())?,
                            ),
                            "NiVertexColorProperty" => Block::NiVertexColorProperty(
                                NiVertexColorProperty::read_options(reader, endian, ())?,
                            ),
                            "NiTriShape" => {
                                Block::NiTriShape(NiTriShape::read_options(reader, endian, ())?)
                            }
                            "NiStringExtraData" => Block::NiStringExtraData(
                                NiStringExtraData::read_options(reader, endian, ())?,
                            ),
                            "NiTexturingProperty" => Block::NiTexturingProperty(
                                NiTexturingProperty::read_options(reader, endian, ())?,
                            ),
                            "NiSourceTexture" => Block::NiSourceTexture(
                                NiSourceTexture::read_options(reader, endian, ())?,
                            ),
                            "NiAlphaProperty" => Block::NiAlphaProperty(
                                NiAlphaProperty::read_options(reader, endian, ())?,
                            ),
                            "NiMaterialProperty" => Block::NiMaterialProperty(
                                NiMaterialProperty::read_options(reader, endian, ())?,
                            ),
                            "NiTriShapeData" => Block::NiTriShapeData(
                                NiTriShapeData::read_options(reader, endian, ())?,
                            ),
                            "NiIntegerExtraData" => Block::NiIntegerExtraData(
                                NiIntegerExtraData::read_options(reader, endian, ())?,
                            ),
                            "NiSpecularProperty" => Block::NiSpecularProperty(
                                NiSpecularProperty::read_options(reader, endian, ())?,
                            ),
                            "NiSwitchNode" => {
                                Block::NiSwitchNode(NiSwitchNode::read_options(reader, endian, ())?)
                            }
                            "NiLODNode" => {
                                Block::NiLODNode(NiLODNode::read_options(reader, endian, ())?)
                            }
                            "NiRangeLODData" => Block::NiRangeLODData(
                                NiRangeLODData::read_options(reader, endian, ())?,
                            ),
                            "NiBillboardNode" => Block::NiBillboardNode(
                                NiBillboardNode::read_options(reader, endian, ())?,
                            ),
                            "NiBooleanExtraData" => Block::NiBooleanExtraData(
                                NiBooleanExtraData::read_options(reader, endian, ())?,
                            ),
                            "NiCollisionObject" => Block::NiCollisionObject(
                                NiCollisionObject::read_options(reader, endian, ())?,
                            ),
                            "NiCollisionData" => Block::NiCollisionData(
                                NiCollisionData::read_options(reader, endian, ())?,
                            ),
                            "NiStencilProperty" => Block::NiStencilProperty(
                                NiStencilProperty::read_options(reader, endian, ())?,
                            ),
                            "NiTimeController" => Block::NiTimeController(
                                NiTimeController::read_options(reader, endian, ())?,
                            ),
                            "NiInterpController" => Block::NiInterpController(
                                NiInterpController::read_options(reader, endian, ())?,
                            ),
                            "NiSingleInterpController" => Block::NiSingleInterpController(
                                NiSingleInterpController::read_options(reader, endian, ())?,
                            ),
                            "NiFloatInterpController" => Block::NiFloatInterpController(
                                NiFloatInterpController::read_options(reader, endian, ())?,
                            ),
                            "NiAlphaController" => Block::NiAlphaController(
                                NiAlphaController::read_options(reader, endian, ())?,
                            ),
                            "NiInterpolator" => Block::NiInterpolator(
                                NiInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiKeyBasedInterpolator" => Block::NiKeyBasedInterpolator(
                                NiKeyBasedInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiFloatInterpolator" => Block::NiFloatInterpolator(
                                NiFloatInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiFloatData" => {
                                Block::NiFloatData(NiFloatData::read_options(reader, endian, ())?)
                            }
                            "NiParticleSystem" => Block::NiParticleSystem(
                                NiParticleSystem::read_options(reader, endian, ())?,
                            ),
                            "NiPSysEmitterCtlr" => Block::NiPSysEmitterCtlr(
                                NiPSysEmitterCtlr::read_options(reader, endian, ())?,
                            ),
                            "NiPSysUpdateCtlr" => Block::NiPSysUpdateCtlr(
                                NiPSysUpdateCtlr::read_options(reader, endian, ())?,
                            ),
                            "NiBoolInterpolator" => Block::NiBoolInterpolator(
                                NiBoolInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiBoolData" => {
                                Block::NiBoolData(NiBoolData::read_options(reader, endian, ())?)
                            }
                            "NiColorData" => {
                                Block::NiColorData(NiColorData::read_options(reader, endian, ())?)
                            }
                            "NiPSysData" => {
                                Block::NiPSysData(Box::new(NiPSysData::read_options(
                                    reader, endian, (),
                                )?))
                            }
                            "NiPSysAgeDeathModifier" => Block::NiPSysAgeDeathModifier(
                                NiPSysAgeDeathModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysBoxEmitter" => Block::NiPSysBoxEmitter(
                                NiPSysBoxEmitter::read_options(reader, endian, ())?,
                            ),
                            "NiPSysSpawnModifier" => Block::NiPSysSpawnModifier(
                                NiPSysSpawnModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysGrowFadeModifier" => Block::NiPSysGrowFadeModifier(
                                NiPSysGrowFadeModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysColorModifier" => Block::NiPSysColorModifier(
                                NiPSysColorModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysRotationModifier" => Block::NiPSysRotationModifier(
                                NiPSysRotationModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysPositionModifier" => Block::NiPSysPositionModifier(
                                NiPSysPositionModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysBoundUpdateModifier" => Block::NiPSysBoundUpdateModifier(
                                NiPSysBoundUpdateModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysGravityModifier" => Block::NiPSysGravityModifier(
                                NiPSysGravityModifier::read_options(reader, endian, ())?,
                            ),
                            "NiPSysColliderManager" => Block::NiPSysColliderManager(
                                NiPSysColliderManager::read_options(reader, endian, ())?,
                            ),
                            "NiPSysPlanarCollider" => Block::NiPSysPlanarCollider(
                                NiPSysPlanarCollider::read_options(reader, endian, ())?,
                            ),
                            "NiTransformController" => Block::NiTransformController(
                                NiTransformController::read_options(reader, endian, ())?,
                            ),
                            "NiTransformInterpolator" => Block::NiTransformInterpolator(
                                NiTransformInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiTransformData" => Block::NiTransformData(
                                NiTransformData::read_options(reader, endian, ())?,
                            ),
                            "NiColorExtraData" => Block::NiColorExtraData(
                                NiColorExtraData::read_options(reader, endian, ())?,
                            ),
                            "NiFlipController" => Block::NiFlipController(
                                NiFlipController::read_options(reader, endian, ())?,
                            ),
                            "NiFloatExtraData" => Block::NiFloatExtraData(
                                NiFloatExtraData::read_options(reader, endian, ())?,
                            ),
                            "NiTextureTransformController" => Block::NiTextureTransformController(
                                NiTextureTransformController::read_options(reader, endian, ())?,
                            ),
                            "NiPixelData" => {
                                Block::NiPixelData(NiPixelData::read_options(reader, endian, ())?)
                            }
                            "NiVisController" => Block::NiVisController(
                                NiVisController::read_options(reader, endian, ())?,
                            ),
                            "NiTextureEffect" => Block::NiTextureEffect(
                                NiTextureEffect::read_options(reader, endian, ())?,
                            ),
                            "NiSourceCubeMap" => Block::NiSourceCubeMap(
                                NiSourceCubeMap::read_options(reader, endian, ())?,
                            ),
                            "NiShadeProperty" => Block::NiShadeProperty(
                                NiShadeProperty::read_options(reader, endian, ())?,
                            ),
                            "NiGeomMorpherController" => Block::NiGeomMorpherController(
                                NiGeomMorpherController::read_options(reader, endian, ())?,
                            ),
                            "NiMorphData" => {
                                Block::NiMorphData(NiMorphData::read_options(reader, endian, ())?)
                            }
                            "NiDitherProperty" => Block::NiDitherProperty(
                                NiDitherProperty::read_options(reader, endian, ())?,
                            ),
                            "NiMaterialColorController" => Block::NiMaterialColorController(
                                NiMaterialColorController::read_options(reader, endian, ())?,
                            ),
                            "NiPoint3Interpolator" => Block::NiPoint3Interpolator(
                                NiPoint3Interpolator::read_options(reader, endian, ())?,
                            ),
                            "NiPosData" => {
                                Block::NiPosData(NiPosData::read_options(reader, endian, ())?)
                            }
                            "NiSkinInstance" => Block::NiSkinInstance(
                                NiSkinInstance::read_options(reader, endian, ())?,
                            ),
                            "NiSkinData" => {
                                Block::NiSkinData(NiSkinData::read_options(reader, endian, ())?)
                            }
                            "NiSkinPartition" => Block::NiSkinPartition(
                                NiSkinPartition::read_options(reader, endian, ())?,
                            ),
                            "NiPathInterpolator" => Block::NiPathInterpolator(
                                NiPathInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiTriStrips" => {
                                Block::NiTriStrips(NiTriStrips::read_options(reader, endian, ())?)
                            }
                            "NiTriStripsData" => Block::NiTriStripsData(
                                NiTriStripsData::read_options(reader, endian, ())?,
                            ),
                            "NiPSysMeshEmitter" => Block::NiPSysMeshEmitter(
                                NiPSysMeshEmitter::read_options(reader, endian, ())?,
                            ),
                            "NiPSysCylinderEmitter" => Block::NiPSysCylinderEmitter(
                                NiPSysCylinderEmitter::read_options(reader, endian, ())?,
                            ),
                            "NiPSysSphereEmitter" => Block::NiPSysSphereEmitter(
                                NiPSysSphereEmitter::read_options(reader, endian, ())?,
                            ),
                            "NiPSysResetOnLoopCtlr" => Block::NiPSysResetOnLoopCtlr(
                                NiPSysResetOnLoopCtlr::read_options(reader, endian, ())?,
                            ),
                            "NiDirectionalLight" => Block::NiDirectionalLight(
                                NiDirectionalLight::read_options(reader, endian, ())?,
                            ),
                            "NiFloatsExtraData" => Block::NiFloatsExtraData(
                                NiFloatsExtraData::read_options(reader, endian, ())?,
                            ),
                            "NiCamera" => {
                                Block::NiCamera(NiCamera::read_options(reader, endian, ())?)
                            }
                            "NiPointLight" => {
                                Block::NiPointLight(NiPointLight::read_options(reader, endian, ())?)
                            }
                            "NiLookAtInterpolator" => Block::NiLookAtInterpolator(
                                NiLookAtInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiSpotLight" => {
                                Block::NiSpotLight(NiSpotLight::read_options(reader, endian, ())?)
                            }
                            "NiWireframeProperty" => Block::NiWireframeProperty(
                                NiWireframeProperty::read_options(reader, endian, ())?,
                            ),
                            "NiIntegersExtraData" => Block::NiIntegersExtraData(
                                NiIntegersExtraData::read_options(reader, endian, ())?,
                            ),
                            "NiPalette" => {
                                Block::NiPalette(NiPalette::read_options(reader, endian, ())?)
                            }
                            "NiSortAdjustNode" => Block::NiSortAdjustNode(
                                NiSortAdjustNode::read_options(reader, endian, ())?,
                            ),
                            "NiMultiTargetTransformController" => {
                                Block::NiMultiTargetTransformController(
                                    NiMultiTargetTransformController::read_options(
                                        reader,
                                        endian,
                                        (),
                                    )?,
                                )
                            }
                            "NiExtraDataController" => Block::NiExtraDataController(
                                NiExtraDataController::read_options(reader, endian, ())?,
                            ),
                            "NiFloatExtraDataController" => Block::NiFloatExtraDataController(
                                NiFloatExtraDataController::read_options(reader, endian, ())?,
                            ),
                            "NiLightDimmerController" => Block::NiLightDimmerController(
                                NiLightDimmerController::read_options(reader, endian, ())?,
                            ),
                            "NiPSysModifierFloatCtlr" => Block::NiPSysModifierFloatCtlr(
                                NiPSysModifierFloatCtlr::read_options(reader, endian, ())?,
                            ),
                            "NiPSysInitialRotAngleCtlr" => Block::NiPSysInitialRotAngleCtlr(
                                NiPSysInitialRotAngleCtlr::read_options(reader, endian, ())?,
                            ),
                            "NiMeshParticleSystem" => Block::NiMeshParticleSystem(
                                NiMeshParticleSystem::read_options(reader, endian, ())?,
                            ),
                            "NiBoneLODController" => Block::NiBoneLODController(
                                NiBoneLODController::read_options(reader, endian, ())?,
                            ),
                            "NiMeshPSysData" => Block::NiMeshPSysData(Box::new(
                                NiMeshPSysData::read_options(reader, endian, ())?,
                            )),
                            "NiPSysModifierBoolCtlr" => Block::NiPSysModifierBoolCtlr(
                                NiPSysModifierBoolCtlr::read_options(reader, endian, ())?,
                            ),
                            "NiPSysModifierActiveCtlr" => Block::NiPSysModifierActiveCtlr(
                                NiPSysModifierActiveCtlr::read_options(reader, endian, ())?,
                            ),
                            "NiPSysMeshUpdateModifier" => Block::NiPSysMeshUpdateModifier(
                                NiPSysMeshUpdateModifier::read_options(reader, endian, ())?,
                            ),
                            "NiBoolTimelineInterpolator" => Block::NiBoolTimelineInterpolator(
                                NiBoolTimelineInterpolator::read_options(reader, endian, ())?,
                            ),
                            "NiPSysDragModifier" => Block::NiPSysDragModifier(
                                NiPSysDragModifier::read_options(reader, endian, ())?,
                            ),
                            "NiTriShapeDynamicData" => Block::NiTriShapeDynamicData(
                                NiTriShapeDynamicData::read_options(reader, endian, ())?,
                            ),
                            "NiKeyframeController" => Block::NiTransformController(
                                NiTransformController::read_options(reader, endian, ())?,
                            ),
                            "NiKeyframeData" => Block::NiTransformData(
                                NiTransformData::read_options(reader, endian, ())?,
                            ),
                            "NiVisData" => {
                                Block::NiBoolData(NiBoolData::read_options(reader, endian, ())?)
                            }
                            "NiAmbientLight" => Block::NiAmbientLight(
                                NiAmbientLight::read_options(reader, endian, ())?,
                            ),
                            _ => {
                                return Err(binrw::Error::Custom {
                                    pos: reader.stream_position()?,
                                    err: Box::new(NifError::UnknownBlock(
                                        index,
                                        block_type.clone(),
                                    )),
                                });
                            }
                        })
                    })();

                let block = match parsed {
                    Ok(block) => block,
                    Err(e) if matches!(e.root_cause(), binrw::Error::Custom { .. }) => {
                        return Err(e);
                    }
                    Err(e) => {
                        return Err(binrw::Error::Custom {
                            pos: offset,
                            err: Box::new(NifError::BlockParse {
                                index,
                                block_type: block_type.clone(),
                                offset,
                                previous_offset,
                                detail: e.to_string(),
                            }),
                        });
                    }
                };
                previous_offset = offset;
                blocks.push(block);
            }
            None => {
                return Err(binrw::Error::Custom {
                    pos: reader.stream_position()?,
                    err: Box::new(NifError::InvalidBlockTypeIndex),
                });
            }
        }
    }
    // println!("Finished reading at {}", reader.seek(SeekFrom::Current(0))?);

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::Endian;
    use std::io::Cursor;

    fn int_prefixed(bytes: &[u8]) -> BinResult<(Vec<u8>, u64)> {
        let mut reader = Cursor::new(bytes.to_vec());
        let value = parse_int_prefixed_bytes(&mut reader, Endian::Little, ())?;
        Ok((value, reader.position()))
    }

    fn int_prefixed_written(value: &[u8]) -> Vec<u8> {
        let mut written = Vec::new();
        write_int_prefixed_bytes(
            &value.to_vec(),
            &mut Cursor::new(&mut written),
            Endian::Little,
            (),
        )
        .expect("write");
        written
    }

    fn lf_terminated(bytes: &[u8]) -> BinResult<(String, u64)> {
        let mut reader = Cursor::new(bytes.to_vec());
        let value = parse_lf_terminated_string(&mut reader, Endian::Little, ())?;
        Ok((value, reader.position()))
    }

    #[test]
    fn int_prefixed_string_leaves_the_reader_after_the_payload() {
        let (value, position) = int_prefixed(b"\x04\x00\x00\x00name\xAA").expect("parse");
        assert_eq!(value, b"name");
        assert_eq!(position, 8);
    }

    #[test]
    fn int_prefixed_string_accepts_an_empty_payload() {
        let (value, position) = int_prefixed(b"\x00\x00\x00\x00\xAA").expect("parse");
        assert_eq!(value, b"");
        assert_eq!(position, 4);
    }

    #[test]
    fn int_prefixed_string_preserves_bytes_that_are_not_utf8() {
        let euc_kr = [0x04, 0x00, 0x00, 0x00, 0xbe, 0xcb, 0xc6, 0xc4];
        let (value, _) = int_prefixed(&euc_kr).expect("parse");
        assert_eq!(value, [0xbe, 0xcb, 0xc6, 0xc4]);
        assert_eq!(int_prefixed_written(&value), euc_kr);
    }

    #[test]
    fn int_prefixed_string_rejects_a_truncated_payload() {
        assert!(int_prefixed(b"\x08\x00\x00\x00name").is_err());
    }

    #[test]
    fn int_prefixed_string_rejects_a_count_beyond_the_stream() {
        assert!(int_prefixed(b"\xFF\xFF\xFF\xFFname").is_err());
    }

    #[test]
    fn lf_terminated_string_leaves_the_reader_after_the_newline() {
        let (value, position) = lf_terminated(b"20.0.0.4\n\xAA").expect("parse");
        assert_eq!(value, "20.0.0.4");
        assert_eq!(position, 9);
    }

    #[test]
    fn lf_terminated_string_rejects_a_missing_newline() {
        assert!(lf_terminated(b"20.0.0.4").is_err());
    }

    #[test]
    fn lf_terminated_string_rejects_an_unterminated_run() {
        let bytes = vec![b'x'; MAX_LF_TERMINATED_STRING_LEN * 2];
        assert!(lf_terminated(&bytes).is_err());
    }
}
