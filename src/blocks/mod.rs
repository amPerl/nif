mod ni_alpha_property;
mod ni_av_object;
mod ni_billboard_node;
mod ni_collision_data;
mod ni_collision_object;
mod ni_integer_extra_data;
mod ni_lod_node;
mod ni_material_property;
mod ni_node;
mod ni_object_net;
mod ni_range_lod_data;
mod ni_source_texture;
mod ni_specular_property;
mod ni_string;
mod ni_string_extra_data;
mod ni_switch_node;
mod ni_texturing_property;
mod ni_tri_shape;
mod ni_tri_shape_data;
mod ni_vertex_color_property;
mod ni_z_buffer_property;

pub use ni_alpha_property::*;
pub use ni_av_object::*;
pub use ni_billboard_node::*;
pub use ni_collision_data::*;
pub use ni_collision_object::*;
pub use ni_integer_extra_data::*;
pub use ni_lod_node::*;
pub use ni_material_property::*;
pub use ni_node::*;
pub use ni_object_net::*;
pub use ni_range_lod_data::*;
pub use ni_source_texture::*;
pub use ni_specular_property::*;
pub use ni_string::*;
pub use ni_string_extra_data::*;
pub use ni_switch_node::*;
pub use ni_texturing_property::*;
pub use ni_tri_shape::*;
pub use ni_tri_shape_data::*;
pub use ni_vertex_color_property::*;
pub use ni_z_buffer_property::*;

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
}
