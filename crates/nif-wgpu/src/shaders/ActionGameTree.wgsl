// From the game's own ActionGameTree.fx, technique ActionGameTree.
//
// The vertex shader takes only position and uv: no normal, so there is no lambert term and the
// light is constant over the whole mesh. That flatness is the shader, not a simplification.
//
// The shipped source has `MaterialColor` commented out and a hardcoded 0.6 in its place:
//     Out.Color = (vLightDiffuse * float3(0.6f, 0.6f, 0.6f)) + vLightAmbient;
// so the material colour a shape carries is deliberately ignored.

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let texel = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in));

    let flat_light = camera.light_diffuse.rgb * 0.6 + camera.light_ambient.rgb;
    let shaded = texel.rgb * flat_light;

    // the pixel shader returns the texture's own alpha, and the vertex colour is never read.
    // The file's own NiAlphaProperty still decides whether that alpha tests or blends.
    let alpha = texel.a;
    if (model.alpha.y > 0.5 && alpha <= model.alpha.x) {
        discard;
    }
    return vec4<f32>(mix(lit_colour(in), shaded, camera.flags.y), alpha);
}
