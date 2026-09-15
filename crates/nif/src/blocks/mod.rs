mod ni_animation;
mod ni_main;
mod ni_particle;

pub use ni_animation::*;
pub use ni_main::*;
pub use ni_particle::*;

use crate::common::{BlockRef, Triangle};
use binrw::BinWrite;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
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
    NiBooleanExtraData(NiBooleanExtraData),
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
    NiPSysData(Box<NiPSysData>),
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
    NiCamera(NiCamera),
    NiPointLight(NiPointLight),
    NiLookAtInterpolator(NiLookAtInterpolator),
    NiSpotLight(NiSpotLight),
    NiWireframeProperty(NiWireframeProperty),
    NiIntegersExtraData(NiIntegersExtraData),
    NiPalette(NiPalette),
    NiSortAdjustNode(NiSortAdjustNode),
    NiMultiTargetTransformController(NiMultiTargetTransformController),
    NiExtraDataController(NiExtraDataController),
    NiFloatExtraDataController(NiFloatExtraDataController),
    NiLightDimmerController(NiLightDimmerController),
    NiPSysModifierFloatCtlr(NiPSysModifierFloatCtlr),
    NiPSysInitialRotAngleCtlr(NiPSysInitialRotAngleCtlr),
    NiMeshParticleSystem(NiMeshParticleSystem),
    NiBoneLODController(NiBoneLODController),
    NiMeshPSysData(Box<NiMeshPSysData>),
    NiPSysModifierBoolCtlr(NiPSysModifierBoolCtlr),
    NiPSysModifierActiveCtlr(NiPSysModifierActiveCtlr),
    NiPSysMeshUpdateModifier(NiPSysMeshUpdateModifier),
    NiBoolTimelineInterpolator(NiBoolTimelineInterpolator),
    NiPSysDragModifier(NiPSysDragModifier),
    NiTriShapeDynamicData(NiTriShapeDynamicData),
    NiAmbientLight(NiAmbientLight),
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
            Block::NiBooleanExtraData(_) => "NiBooleanExtraData",
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
            Block::NiCamera(_) => "NiCamera",
            Block::NiPointLight(_) => "NiPointLight",
            Block::NiLookAtInterpolator(_) => "NiLookAtInterpolator",
            Block::NiSpotLight(_) => "NiSpotLight",
            Block::NiWireframeProperty(_) => "NiWireframeProperty",
            Block::NiIntegersExtraData(_) => "NiIntegersExtraData",
            Block::NiPalette(_) => "NiPalette",
            Block::NiSortAdjustNode(_) => "NiSortAdjustNode",
            Block::NiMultiTargetTransformController(_) => "NiMultiTargetTransformController",
            Block::NiExtraDataController(_) => "NiExtraDataController",
            Block::NiFloatExtraDataController(_) => "NiFloatExtraDataController",
            Block::NiLightDimmerController(_) => "NiLightDimmerController",
            Block::NiPSysModifierFloatCtlr(_) => "NiPSysModifierFloatCtlr",
            Block::NiPSysInitialRotAngleCtlr(_) => "NiPSysInitialRotAngleCtlr",
            Block::NiMeshParticleSystem(_) => "NiMeshParticleSystem",
            Block::NiBoneLODController(_) => "NiBoneLODController",
            Block::NiMeshPSysData(_) => "NiMeshPSysData",
            Block::NiPSysModifierBoolCtlr(_) => "NiPSysModifierBoolCtlr",
            Block::NiPSysModifierActiveCtlr(_) => "NiPSysModifierActiveCtlr",
            Block::NiPSysMeshUpdateModifier(_) => "NiPSysMeshUpdateModifier",
            Block::NiBoolTimelineInterpolator(_) => "NiBoolTimelineInterpolator",
            Block::NiPSysDragModifier(_) => "NiPSysDragModifier",
            Block::NiTriShapeDynamicData(_) => "NiTriShapeDynamicData",
            Block::NiAmbientLight(_) => "NiAmbientLight",
        }
    }

    pub fn object_net(&self) -> Option<&NiObjectNET> {
        let obj: &NiObjectNET = match self {
            Block::NiObjectNET(b) => b,
            Block::NiAvObject(b) => b,
            Block::NiNode(b) => b,
            Block::NiSwitchNode(b) => b,
            Block::NiLODNode(b) => b,
            Block::NiBillboardNode(b) => b,
            Block::NiTriShape(b) => b,
            Block::NiTriStrips(b) => b,
            Block::NiParticleSystem(b) => b,
            Block::NiMeshParticleSystem(b) => b,
            Block::NiTextureEffect(b) => b,
            Block::NiCamera(b) => b,
            Block::NiPointLight(b) => b,
            Block::NiSpotLight(b) => b,
            Block::NiAmbientLight(b) => b,
            Block::NiDirectionalLight(b) => b,
            Block::NiZBufferProperty(b) => b,
            Block::NiVertexColorProperty(b) => b,
            Block::NiTexturingProperty(b) => b,
            Block::NiSourceTexture(b) => b,
            Block::NiSourceCubeMap(b) => b,
            Block::NiAlphaProperty(b) => b,
            Block::NiMaterialProperty(b) => b,
            Block::NiSpecularProperty(b) => b,
            Block::NiStencilProperty(b) => b,
            Block::NiShadeProperty(b) => b,
            Block::NiDitherProperty(b) => b,
            _ => return None,
        };
        Some(obj)
    }

    pub fn as_time_controller(&self) -> Option<&NiTimeController> {
        let controller: &NiTimeController = match self {
            Block::NiTimeController(b) => b,
            Block::NiInterpController(b) => b,
            Block::NiSingleInterpController(b) => b,
            Block::NiFloatInterpController(b) => b,
            Block::NiAlphaController(b) => b,
            Block::NiTransformController(b) => b,
            Block::NiFlipController(b) => b,
            Block::NiTextureTransformController(b) => b,
            Block::NiVisController(b) => b,
            Block::NiGeomMorpherController(b) => b,
            Block::NiMaterialColorController(b) => b,
            Block::NiMultiTargetTransformController(b) => b,
            Block::NiExtraDataController(b) => b,
            Block::NiFloatExtraDataController(b) => b,
            Block::NiLightDimmerController(b) => b,
            Block::NiBoneLODController(b) => b,
            Block::NiPSysUpdateCtlr(b) => b,
            Block::NiPSysResetOnLoopCtlr(b) => b,
            Block::NiPSysEmitterCtlr(b) => b,
            Block::NiPSysModifierBoolCtlr(b) => b,
            Block::NiPSysModifierFloatCtlr(b) => b,
            Block::NiPSysModifierActiveCtlr(b) => b,
            Block::NiPSysInitialRotAngleCtlr(b) => b,
            _ => return None,
        };
        Some(controller)
    }

    /// The modifier a particle system runs, for the ones that are modifiers. `order` decides
    /// when it runs, and the engine sorts a system's list ascending on it.
    pub fn as_psys_modifier(&self) -> Option<&NiPSysModifier> {
        let modifier: &NiPSysModifier = match self {
            Block::NiPSysAgeDeathModifier(b) => b,
            Block::NiPSysBoundUpdateModifier(b) => b,
            Block::NiPSysPositionModifier(b) => b,
            Block::NiPSysSpawnModifier(b) => b,
            Block::NiPSysRotationModifier(b) => b,
            Block::NiPSysColorModifier(b) => b,
            Block::NiPSysGrowFadeModifier(b) => b,
            Block::NiPSysGravityModifier(b) => b,
            Block::NiPSysDragModifier(b) => b,
            Block::NiPSysColliderManager(b) => b,
            Block::NiPSysMeshUpdateModifier(b) => b,
            Block::NiPSysBoxEmitter(b) => b,
            Block::NiPSysCylinderEmitter(b) => b,
            Block::NiPSysSphereEmitter(b) => b,
            Block::NiPSysMeshEmitter(b) => b,
            _ => return None,
        };
        Some(modifier)
    }

    /// A particle system in either of its forms.
    ///
    /// `NiMeshParticleSystem` adds no fields of its own to `NiParticleSystem`: what differs is
    /// only that its particles are instances of a mesh rather than sprites, which is the drawing
    /// and not the simulation. Everything that advances a system wants this rather than the one
    /// variant.
    pub fn particle_system(&self) -> Option<&NiParticleSystem> {
        let system: &NiParticleSystem = match self {
            Block::NiParticleSystem(b) => b,
            Block::NiMeshParticleSystem(b) => b,
            _ => return None,
        };
        Some(system)
    }

    /// The stored particle data behind a system's `data_ref`, in either of its forms.
    ///
    /// `NiMeshPSysData` adds a pool size, a fill flag, a generation list and the meshes it
    /// instances on top of `NiPSysData`. Everything the simulation reads is underneath.
    pub fn psys_data(&self) -> Option<&NiPSysData> {
        let data: &NiPSysData = match self {
            Block::NiPSysData(b) => b,
            Block::NiMeshPSysData(b) => b,
            _ => return None,
        };
        Some(data)
    }

    pub fn geometry(&self) -> Option<&NiGeometry> {
        let geometry: &NiGeometry = match self {
            Block::NiTriShape(b) => b,
            Block::NiTriStrips(b) => b,
            Block::NiParticleSystem(b) => b,
            Block::NiMeshParticleSystem(b) => b,
            _ => return None,
        };
        Some(geometry)
    }

    /// The stored geometry behind a shape's `data_ref`, whichever form holds it.
    pub fn geometry_data(&self) -> Option<&NiGeometryData> {
        let data: &NiGeometryData = match self {
            Block::NiTriShapeData(b) => b,
            Block::NiTriShapeDynamicData(b) => b,
            Block::NiTriStripsData(b) => b,
            _ => return None,
        };
        Some(data)
    }

    /// The triangles a shape stores, whichever form holds them, with the data they index into.
    /// A strip is expanded here so a caller never has to know which of the two it has.
    pub fn triangles<'a>(
        &self,
        blocks: &'a [Block],
    ) -> Option<(&'a NiGeometryData, Vec<Triangle>)> {
        let data = match self {
            Block::NiTriShape(shape) => shape.data_ref.get(blocks)?,
            Block::NiTriStrips(strips) => strips.data_ref.get(blocks)?,
            _ => return None,
        };
        match data {
            Block::NiTriShapeData(data) => Some((&data.base.base, data.triangles.clone()?)),
            Block::NiTriShapeDynamicData(data) => {
                Some((&data.base.base.base, data.base.triangles.clone()?))
            }
            Block::NiTriStripsData(data) => Some((&data.base.base, data.triangles().collect())),
            _ => None,
        }
    }

    pub fn av_object(&self) -> Option<&NiAvObject> {
        let obj: &NiAvObject = match self {
            Block::NiAvObject(b) => b,
            Block::NiNode(b) => b,
            Block::NiSwitchNode(b) => b,
            Block::NiLODNode(b) => b,
            Block::NiBillboardNode(b) => b,
            Block::NiTriShape(b) => b,
            Block::NiTriStrips(b) => b,
            Block::NiParticleSystem(b) => b,
            Block::NiMeshParticleSystem(b) => b,
            Block::NiTextureEffect(b) => b,
            Block::NiCamera(b) => b,
            Block::NiPointLight(b) => b,
            Block::NiSpotLight(b) => b,
            Block::NiAmbientLight(b) => b,
            Block::NiDirectionalLight(b) => b,
            _ => return None,
        };
        Some(obj)
    }

    pub fn node(&self) -> Option<&NiNode> {
        let obj: &NiNode = match self {
            Block::NiNode(b) => b,
            Block::NiSwitchNode(b) => b,
            Block::NiLODNode(b) => b,
            Block::NiBillboardNode(b) => b,
            // A node that says how its subtree is sorted and nothing else. Left out, the
            // subtree is left out with it: `i_smoker` keeps both its shapes under one and drew
            // nothing at all.
            Block::NiSortAdjustNode(b) => b,
            _ => return None,
        };
        Some(obj)
    }

    pub fn child_refs(&self) -> Option<&[BlockRef]> {
        Some(&self.node()?.child_refs)
    }

    pub fn children<'b>(&self, blocks: &'b [Block]) -> Option<Vec<(BlockRef, &'b Block)>> {
        Some(
            self.child_refs()?
                .iter()
                .filter_map(|r| r.get(blocks).map(|b| (*r, b)))
                .collect(),
        )
    }

    /// The light behind any of the four light blocks, which share everything but their falloff.
    pub fn light(&self) -> Option<&NiLight> {
        let light: &NiLight = match self {
            Block::NiAmbientLight(b) => b,
            Block::NiDirectionalLight(b) => b,
            Block::NiPointLight(b) => b,
            Block::NiSpotLight(b) => b,
            _ => return None,
        };
        Some(light)
    }

    /// The effects a node holds. They reach everything under it, unlike a property, which the
    /// nearest one of its kind replaces.
    pub fn effect_refs(&self) -> Option<&[BlockRef]> {
        Some(&self.node()?.effect_refs)
    }

    pub fn property_refs(&self) -> Option<&[BlockRef]> {
        Some(&self.av_object()?.property_refs)
    }

    pub fn properties<'b>(&self, blocks: &'b [Block]) -> Option<Vec<(BlockRef, &'b Block)>> {
        Some(
            self.property_refs()?
                .iter()
                .filter_map(|r| r.get(blocks).map(|b| (*r, b)))
                .collect(),
        )
    }

    pub fn extra_data_refs(&self) -> Option<&[BlockRef]> {
        Some(&self.object_net()?.extra_data_refs)
    }

    pub fn extra_data<'b>(&self, blocks: &'b [Block]) -> Option<Vec<(BlockRef, &'b Block)>> {
        Some(
            self.extra_data_refs()?
                .iter()
                .filter_map(|r| r.get(blocks).map(|b| (*r, b)))
                .collect(),
        )
    }
}
