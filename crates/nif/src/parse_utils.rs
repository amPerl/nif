use super::blocks::{Block, *};
use super::common;
use crate::error::NifError;
use binrw::{io::Read, BinRead, BinResult};
use std::io::SeekFrom;

#[binrw::parser(reader, endian)]
pub fn parse_keys<T: BinRead>(
    num_keys: u32,
    key_type: Option<common::KeyType>,
) -> BinResult<Vec<common::Key<T>>>
where
    T: for<'a> BinRead<Args<'a> = ()>,
{
    if num_keys == 0 {
        return Ok(Vec::new());
    }
    let key_type = key_type.expect("num_keys was >0, key_type should exist");

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
    let key_type = key_type.expect("num_keys was >0, key_type should exist");
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
    let version: u32 =
        version_split[0] << 24 | version_split[1] << 16 | version_split[2] << 8 | version_split[3];
    Ok(version)
}

#[binrw::parser(reader)]
pub fn parse_lf_terminated_string() -> BinResult<String> {
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

#[binrw::parser(reader, endian)]
pub fn parse_int_prefixed_string() -> BinResult<String> {
    let count = u32::read_options(reader, endian, ())?;

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

#[binrw::parser(reader, endian)]
pub fn parse_blocks(strings: Vec<String>, block_type_indices: Vec<u16>) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::new();

    for block_type_index in block_type_indices {
        match strings.get(block_type_index as usize) {
            Some(block_type) => {
                // println!(
                //     "Reading block {} at {}",
                //     block_type,
                //     reader.seek(SeekFrom::Current(0))?
                // );

                let block = match block_type.as_ref() {
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
                    "NiSourceTexture" => {
                        Block::NiSourceTexture(NiSourceTexture::read_options(reader, endian, ())?)
                    }
                    "NiAlphaProperty" => {
                        Block::NiAlphaProperty(NiAlphaProperty::read_options(reader, endian, ())?)
                    }
                    "NiMaterialProperty" => Block::NiMaterialProperty(
                        NiMaterialProperty::read_options(reader, endian, ())?,
                    ),
                    "NiTriShapeData" => {
                        Block::NiTriShapeData(NiTriShapeData::read_options(reader, endian, ())?)
                    }
                    "NiIntegerExtraData" => Block::NiIntegerExtraData(
                        NiIntegerExtraData::read_options(reader, endian, ())?,
                    ),
                    "NiSpecularProperty" => Block::NiSpecularProperty(
                        NiSpecularProperty::read_options(reader, endian, ())?,
                    ),
                    "NiSwitchNode" => {
                        Block::NiSwitchNode(NiSwitchNode::read_options(reader, endian, ())?)
                    }
                    "NiLODNode" => Block::NiLODNode(NiLODNode::read_options(reader, endian, ())?),
                    "NiRangeLODData" => {
                        Block::NiRangeLODData(NiRangeLODData::read_options(reader, endian, ())?)
                    }
                    "NiBillboardNode" => {
                        Block::NiBillboardNode(NiBillboardNode::read_options(reader, endian, ())?)
                    }
                    "NiCollisionData" => {
                        Block::NiCollisionData(NiCollisionData::read_options(reader, endian, ())?)
                    }
                    "NiStencilProperty" => Block::NiStencilProperty(
                        NiStencilProperty::read_options(reader, endian, ())?,
                    ),
                    "NiTimeController" => {
                        Block::NiTimeController(NiTimeController::read_options(reader, endian, ())?)
                    }
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
                    "NiInterpolator" => {
                        Block::NiInterpolator(NiInterpolator::read_options(reader, endian, ())?)
                    }
                    "NiKeyBasedInterpolator" => Block::NiKeyBasedInterpolator(
                        NiKeyBasedInterpolator::read_options(reader, endian, ())?,
                    ),
                    "NiFloatInterpolator" => Block::NiFloatInterpolator(
                        NiFloatInterpolator::read_options(reader, endian, ())?,
                    ),
                    "NiFloatData" => {
                        Block::NiFloatData(NiFloatData::read_options(reader, endian, ())?)
                    }
                    "NiParticleSystem" => {
                        Block::NiParticleSystem(NiParticleSystem::read_options(reader, endian, ())?)
                    }
                    "NiPSysEmitterCtlr" => Block::NiPSysEmitterCtlr(
                        NiPSysEmitterCtlr::read_options(reader, endian, ())?,
                    ),
                    "NiPSysUpdateCtlr" => {
                        Block::NiPSysUpdateCtlr(NiPSysUpdateCtlr::read_options(reader, endian, ())?)
                    }
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
                        Block::NiPSysData(NiPSysData::read_options(reader, endian, ())?)
                    }
                    "NiPSysAgeDeathModifier" => Block::NiPSysAgeDeathModifier(
                        NiPSysAgeDeathModifier::read_options(reader, endian, ())?,
                    ),
                    "NiPSysBoxEmitter" => {
                        Block::NiPSysBoxEmitter(NiPSysBoxEmitter::read_options(reader, endian, ())?)
                    }
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
                    "NiTransformData" => {
                        Block::NiTransformData(NiTransformData::read_options(reader, endian, ())?)
                    }
                    "NiColorExtraData" => {
                        Block::NiColorExtraData(NiColorExtraData::read_options(reader, endian, ())?)
                    }
                    "NiFlipController" => {
                        Block::NiFlipController(NiFlipController::read_options(reader, endian, ())?)
                    }
                    "NiFloatExtraData" => {
                        Block::NiFloatExtraData(NiFloatExtraData::read_options(reader, endian, ())?)
                    }
                    "NiTextureTransformController" => Block::NiTextureTransformController(
                        NiTextureTransformController::read_options(reader, endian, ())?,
                    ),
                    "NiPixelData" => {
                        Block::NiPixelData(NiPixelData::read_options(reader, endian, ())?)
                    }
                    "NiVisController" => {
                        Block::NiVisController(NiVisController::read_options(reader, endian, ())?)
                    }
                    "NiTextureEffect" => {
                        Block::NiTextureEffect(NiTextureEffect::read_options(reader, endian, ())?)
                    }
                    "NiSourceCubeMap" => {
                        Block::NiSourceCubeMap(NiSourceCubeMap::read_options(reader, endian, ())?)
                    }
                    "NiShadeProperty" => {
                        Block::NiShadeProperty(NiShadeProperty::read_options(reader, endian, ())?)
                    }
                    "NiGeomMorpherController" => Block::NiGeomMorpherController(
                        NiGeomMorpherController::read_options(reader, endian, ())?,
                    ),
                    "NiMorphData" => {
                        Block::NiMorphData(NiMorphData::read_options(reader, endian, ())?)
                    }
                    "NiDitherProperty" => {
                        Block::NiDitherProperty(NiDitherProperty::read_options(reader, endian, ())?)
                    }
                    "NiMaterialColorController" => Block::NiMaterialColorController(
                        NiMaterialColorController::read_options(reader, endian, ())?,
                    ),
                    "NiPoint3Interpolator" => Block::NiPoint3Interpolator(
                        NiPoint3Interpolator::read_options(reader, endian, ())?,
                    ),
                    "NiPosData" => Block::NiPosData(NiPosData::read_options(reader, endian, ())?),
                    "NiSkinInstance" => {
                        Block::NiSkinInstance(NiSkinInstance::read_options(reader, endian, ())?)
                    }
                    "NiSkinData" => {
                        Block::NiSkinData(NiSkinData::read_options(reader, endian, ())?)
                    }
                    "NiSkinPartition" => {
                        Block::NiSkinPartition(NiSkinPartition::read_options(reader, endian, ())?)
                    }
                    "NiPathInterpolator" => Block::NiPathInterpolator(
                        NiPathInterpolator::read_options(reader, endian, ())?,
                    ),
                    "NiTriStrips" => {
                        Block::NiTriStrips(NiTriStrips::read_options(reader, endian, ())?)
                    }
                    "NiTriStripsData" => {
                        Block::NiTriStripsData(NiTriStripsData::read_options(reader, endian, ())?)
                    }
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
                    _ => {
                        return Err(binrw::Error::Custom {
                            pos: reader.seek(SeekFrom::Current(0))?,
                            err: Box::new(NifError::UnknownBlock(blocks.len(), block_type.clone())),
                        });
                    }
                };
                blocks.push(block);
            }
            None => {
                return Err(binrw::Error::Custom {
                    pos: reader.seek(SeekFrom::Current(0))?,
                    err: Box::new(NifError::InvalidBlockTypeIndex),
                });
            }
        }
    }
    // println!("Finished reading at {}", reader.seek(SeekFrom::Current(0))?);

    Ok(blocks)
}
