// The outline half of the toon cartoon techniques. The hull is built in this pass's own vertex
// displacement; all that is left here is the colour it is filled with.
//
// The original declares no pixel shader at all: its vertex program writes outlineColor to the
// diffuse and the fixed function stage passes it straight through. Its own comment says why,
// that an NSF cannot set a material for a pixel shader constant map.
//
// outlineColor is a colour attribute rather than a float, and nothing binds those yet, so this
// is the value the source declares.
const OUTLINE_COLOR: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(OUTLINE_COLOR, 1.0);
}
