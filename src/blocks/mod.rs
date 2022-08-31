mod ni_animation;
mod ni_main;
mod ni_particle;

pub use ni_animation::*;
pub use ni_main::*;
pub use ni_particle::*;

use crate::common::BlockRef;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq)]
pub enum Block {
    NiObjectNET(NiObjectNET),
    NiAvObject(NiAvObject),
    NiNode(NiNode),
    NiZBufferProperty(NiZBufferProperty),
    NiVertexColorProperty(NiVertexColorProperty),
    NiTriShape(NiTriShape),
    NiStringExtraData(NiStringExtraData),
    NiTexturingProperty(NiTexturingProperty),
    NiSourceTexture(NiSourceTexture),
    NiAlphaProperty(NiAlphaProperty),
    NiMaterialProperty(NiMaterialProperty),
    NiTriShapeData(NiTriShapeData),
    NiIntegerExtraData(NiIntegerExtraData),
    NiSpecularProperty(NiSpecularProperty),
    NiSwitchNode(NiSwitchNode),
    NiLODNode(NiLODNode),
    NiRangeLODData(NiRangeLODData),
    NiBillboardNode(NiBillboardNode),
    NiCollisionObject(NiCollisionObject),
    NiCollisionData(NiCollisionData),
    NiStencilProperty(NiStencilProperty),
    NiTimeController(NiTimeController),
    NiInterpController(NiInterpController),
    NiSingleInterpController(NiSingleInterpController),
    NiFloatInterpController(NiFloatInterpController),
    NiAlphaController(NiAlphaController),
    NiInterpolator(NiInterpolator),
    NiKeyBasedInterpolator(NiKeyBasedInterpolator),
    NiFloatInterpolator(NiFloatInterpolator),
    NiFloatData(NiFloatData),
    NiParticleSystem(NiParticleSystem),
    NiPSysEmitterCtlr(NiPSysEmitterCtlr),
    NiPSysUpdateCtlr(NiPSysUpdateCtlr),
    NiBoolInterpolator(NiBoolInterpolator),
    NiBoolData(NiBoolData),
    NiColorData(NiColorData),
    NiPSysData(NiPSysData),
    NiPSysAgeDeathModifier(NiPSysAgeDeathModifier),
    NiPSysBoxEmitter(NiPSysBoxEmitter),
    NiPSysSpawnModifier(NiPSysSpawnModifier),
    NiPSysGrowFadeModifier(NiPSysGrowFadeModifier),
    NiPSysColorModifier(NiPSysColorModifier),
    NiPSysRotationModifier(NiPSysRotationModifier),
    NiPSysPositionModifier(NiPSysPositionModifier),
    NiPSysBoundUpdateModifier(NiPSysBoundUpdateModifier),
    NiPSysGravityModifier(NiPSysGravityModifier),
    NiPSysColliderManager(NiPSysColliderManager),
    NiPSysPlanarCollider(NiPSysPlanarCollider),
    NiTransformController(NiTransformController),
    NiTransformData(NiTransformData),
    NiTransformInterpolator(NiTransformInterpolator),
    NiColorExtraData(NiColorExtraData),
    NiFlipController(NiFlipController),
    NiFloatExtraData(NiFloatExtraData),
    NiTextureTransformController(NiTextureTransformController),
    NiPixelData(NiPixelData),
    NiVisController(NiVisController),
    NiTextureEffect(NiTextureEffect),
    NiSourceCubeMap(NiSourceCubeMap),
    NiShadeProperty(NiShadeProperty),
    NiGeomMorpherController(NiGeomMorpherController),
    NiMorphData(NiMorphData),
    NiDitherProperty(NiDitherProperty),
    NiMaterialColorController(NiMaterialColorController),
    NiPoint3Interpolator(NiPoint3Interpolator),
    NiPosData(NiPosData),
    NiSkinInstance(NiSkinInstance),
    NiSkinData(NiSkinData),
    NiSkinPartition(NiSkinPartition),
    NiPathInterpolator(NiPathInterpolator),
    NiTriStrips(NiTriStrips),
    NiTriStripsData(NiTriStripsData),
    NiPSysMeshEmitter(NiPSysMeshEmitter),
    NiPSysCylinderEmitter(NiPSysCylinderEmitter),
    NiPSysSphereEmitter(NiPSysSphereEmitter),
    NiPSysResetOnLoopCtlr(NiPSysResetOnLoopCtlr),
    NiDirectionalLight(NiDirectionalLight),
    NiFloatsExtraData(NiFloatsExtraData),
}

impl Block {
    pub fn name(&self) -> &'static str {
        match self {
            Block::NiObjectNET(_) => "NiObjectNET",
            Block::NiAvObject(_) => "NiAvObject",
            Block::NiNode(_) => "NiNode",
            Block::NiZBufferProperty(_) => "NiZBufferProperty",
            Block::NiVertexColorProperty(_) => "NiVertexColorProperty",
            Block::NiTriShape(_) => "NiTriShape",
            Block::NiStringExtraData(_) => "NiStringExtraData",
            Block::NiTexturingProperty(_) => "NiTexturingProperty",
            Block::NiSourceTexture(_) => "NiSourceTexture",
            Block::NiAlphaProperty(_) => "NiAlphaProperty",
            Block::NiMaterialProperty(_) => "NiMaterialProperty",
            Block::NiTriShapeData(_) => "NiTriShapeData",
            Block::NiIntegerExtraData(_) => "NiIntegerExtraData",
            Block::NiSpecularProperty(_) => "NiSpecularProperty",
            Block::NiSwitchNode(_) => "NiSwitchNode",
            Block::NiLODNode(_) => "NiLODNode",
            Block::NiRangeLODData(_) => "NiRangeLODData",
            Block::NiBillboardNode(_) => "NiBillboardNode",
            Block::NiCollisionObject(_) => "NiCollisionObject",
            Block::NiCollisionData(_) => "NiCollisionData",
            Block::NiStencilProperty(_) => "NiStencilProperty",
            Block::NiTimeController(_) => "NiTimeController",
            Block::NiInterpController(_) => "NiInterpController",
            Block::NiSingleInterpController(_) => "NiSingleInterpController",
            Block::NiFloatInterpController(_) => "NiFloatInterpController",
            Block::NiAlphaController(_) => "NiAlphaController",
            Block::NiInterpolator(_) => "NiInterpolator",
            Block::NiKeyBasedInterpolator(_) => "NiKeyBasedInterpolator",
            Block::NiFloatInterpolator(_) => "NiFloatInterpolator",
            Block::NiFloatData(_) => "NiFloatData",
            Block::NiParticleSystem(_) => "NiParticleSystem",
            Block::NiPSysEmitterCtlr(_) => "NiPSysEmitterCtlr",
            Block::NiPSysUpdateCtlr(_) => "NiPSysUpdateCtlr",
            Block::NiBoolInterpolator(_) => "NiBoolInterpolator",
            Block::NiBoolData(_) => "NiBoolData",
            Block::NiColorData(_) => "NiColorData",
            Block::NiPSysData(_) => "NiPSysData",
            Block::NiPSysAgeDeathModifier(_) => "NiPSysAgeDeathModifier",
            Block::NiPSysBoxEmitter(_) => "NiPSysBoxEmitter",
            Block::NiPSysSpawnModifier(_) => "NiPSysSpawnModifier",
            Block::NiPSysGrowFadeModifier(_) => "NiPSysGrowFadeModifier",
            Block::NiPSysColorModifier(_) => "NiPSysColorModifier",
            Block::NiPSysRotationModifier(_) => "NiPSysRotationModifier",
            Block::NiPSysPositionModifier(_) => "NiPSysPositionModifier",
            Block::NiPSysBoundUpdateModifier(_) => "NiPSysBoundUpdateModifier",
            Block::NiPSysGravityModifier(_) => "NiPSysGravityModifier",
            Block::NiPSysColliderManager(_) => "NiPSysColliderManager",
            Block::NiPSysPlanarCollider(_) => "NiPSysPlanarCollider",
            Block::NiTransformController(_) => "NiTransformController",
            Block::NiTransformData(_) => "NiTransformData",
            Block::NiTransformInterpolator(_) => "NiTransformInterpolator",
            Block::NiColorExtraData(_) => "NiColorExtraData",
            Block::NiFlipController(_) => "NiFlipController",
            Block::NiFloatExtraData(_) => "NiFloatExtraData",
            Block::NiTextureTransformController(_) => "NiTextureTransformController",
            Block::NiPixelData(_) => "NiPixelData",
            Block::NiVisController(_) => "NiVisController",
            Block::NiTextureEffect(_) => "NiTextureEffect",
            Block::NiSourceCubeMap(_) => "NiSourceCubeMap",
            Block::NiShadeProperty(_) => "NiShadeProperty",
            Block::NiGeomMorpherController(_) => "NiGeomMorpherController",
            Block::NiMorphData(_) => "NiMorphData",
            Block::NiDitherProperty(_) => "NiDitherProperty",
            Block::NiMaterialColorController(_) => "NiMaterialColorController",
            Block::NiPoint3Interpolator(_) => "NiPoint3Interpolator",
            Block::NiPosData(_) => "NiPosData",
            Block::NiSkinInstance(_) => "NiSkinInstance",
            Block::NiSkinData(_) => "NiSkinData",
            Block::NiSkinPartition(_) => "NiSkinPartition",
            Block::NiPathInterpolator(_) => "NiPathInterpolator",
            Block::NiTriStrips(_) => "NiTriStrips",
            Block::NiTriStripsData(_) => "NiTriStripsData",
            Block::NiPSysMeshEmitter(_) => "NiPSysMeshEmitter",
            Block::NiPSysCylinderEmitter(_) => "NiPSysCylinderEmitter",
            Block::NiPSysSphereEmitter(_) => "NiPSysSphereEmitter",
            Block::NiPSysResetOnLoopCtlr(_) => "NiPSysResetOnLoopCtlr",
            Block::NiDirectionalLight(_) => "NiDirectionalLight",
            Block::NiFloatsExtraData(_) => "NiFloatsExtraData",
        }
    }

    pub fn child_refs(&self) -> Option<Vec<BlockRef>> {
        let child_refs = match self {
            Block::NiNode(block) => Some(&block.child_refs),
            Block::NiSwitchNode(block) => Some(&block.child_refs),
            Block::NiBillboardNode(block) => Some(&block.child_refs),
            Block::NiLODNode(block) => Some(&block.child_refs),
            _ => None,
        };

        Some(child_refs?.clone())
    }

    pub fn children<'b>(&self, blocks: &'b [Block]) -> Option<Vec<(BlockRef, &'b Block)>> {
        Some(
            self.child_refs()?
                .into_iter()
                .filter_map(|r| r.get(blocks).map(|b| (r, b)))
                .collect(),
        )
    }

    pub fn property_refs(&self) -> Option<Vec<BlockRef>> {
        let property_refs = match self {
            Block::NiAvObject(block) => Some(&block.property_refs),
            Block::NiTextureEffect(block) => Some(&block.property_refs),
            Block::NiDirectionalLight(block) => Some(&block.property_refs),
            Block::NiParticleSystem(block) => Some(&block.property_refs),
            Block::NiTriShape(block) => Some(&block.property_refs),
            Block::NiTriStrips(block) => Some(&block.property_refs),
            Block::NiNode(block) => Some(&block.property_refs),
            Block::NiSwitchNode(block) => Some(&block.property_refs),
            Block::NiBillboardNode(block) => Some(&block.property_refs),
            Block::NiLODNode(block) => Some(&block.property_refs),
            _ => None,
        };

        Some(property_refs?.clone())
    }

    pub fn properties<'b>(&self, blocks: &'b [Block]) -> Option<Vec<(BlockRef, &'b Block)>> {
        Some(
            self.property_refs()?
                .into_iter()
                .filter_map(|r| r.get(blocks).map(|b| (r, b)))
                .collect(),
        )
    }

    pub fn extra_data_refs(&self) -> Option<Vec<BlockRef>> {
        let extra_data_refs = match self {
            Block::NiObjectNET(block) => Some(&block.extra_data_refs),
            Block::NiAvObject(block) => Some(&block.extra_data_refs),
            Block::NiNode(block) => Some(&block.extra_data_refs),
            Block::NiTriShape(block) => Some(&block.extra_data_refs),
            Block::NiSwitchNode(block) => Some(&block.extra_data_refs),
            Block::NiLODNode(block) => Some(&block.extra_data_refs),
            Block::NiBillboardNode(block) => Some(&block.extra_data_refs),
            Block::NiParticleSystem(block) => Some(&block.extra_data_refs),
            Block::NiTextureEffect(block) => Some(&block.extra_data_refs),
            Block::NiTriStrips(block) => Some(&block.extra_data_refs),
            Block::NiDirectionalLight(block) => Some(&block.extra_data_refs),
            Block::NiZBufferProperty(block) => Some(&block.extra_data_refs),
            Block::NiVertexColorProperty(block) => Some(&block.extra_data_refs),
            Block::NiTexturingProperty(block) => Some(&block.extra_data_refs),
            Block::NiSourceTexture(block) => Some(&block.extra_data_refs),
            Block::NiAlphaProperty(block) => Some(&block.extra_data_refs),
            Block::NiMaterialProperty(block) => Some(&block.extra_data_refs),
            Block::NiSpecularProperty(block) => Some(&block.extra_data_refs),
            Block::NiStencilProperty(block) => Some(&block.extra_data_refs),
            Block::NiShadeProperty(block) => Some(&block.extra_data_refs),
            Block::NiDitherProperty(block) => Some(&block.extra_data_refs),
            _ => None,
        };

        Some(extra_data_refs?.clone())
    }

    pub fn extra_data<'b>(&self, blocks: &'b [Block]) -> Option<Vec<(BlockRef, &'b Block)>> {
        Some(
            self.extra_data_refs()?
                .into_iter()
                .filter_map(|r| r.get(blocks).map(|b| (r, b)))
                .collect(),
        )
    }
}
