// The outline half of the toon cartoon techniques. The hull is built in this pass's own vertex
// displacement; all that is left here is the colour it is filled with.
//
// The original declares no pixel shader at all: its vertex program writes outlineColor to the
// diffuse and the fixed function stage passes it straight through. Its own comment says why,
// that an NSF cannot set a material for a pixel shader constant map.
//
@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // outlineColor, which the shape supplies and the technique declares as black
    return vec4<f32>(model.attribute_color.rgb, 1.0);
}
