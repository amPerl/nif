// From the game's own ToonShading.vsh and ToonShading.psh, which are the stock Gamebryo sample
// shader's assembly. Gamebryo's ToonShading.NSF supplies the bindings.
//
// The vertex shader clamps N dot L and passes it through a YUV luminance conversion whose weights
// sum to 1, so what reaches the pixel shader is just the clamped N dot L. The pixel shader then:
//     mad r1, t1, c2, c1     ; ramp * lightDiffuse + lightAmbient
//     mul r0.rgb, r1, t0     ; times the base map
//     mov r0.a, c0           ; alpha forced to 1
//
// The ramp is a 256x1 gradient sampled at N dot L, clamped, with TEXF_Point so the bands stay
// hard rather than blending into each other.

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let base = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in));

    // the light direction is the way the light travels, so facing it is the negation
    let intensity = clamp(dot(surface_normal(in), -camera.light_dir.xyz), 0.0, 1.0);
    // a 256x1 ramp, so v is arbitrary. The source hands the same value to both.
    let ramp = textureSample(slot1_texture, slot1_sampler, vec2<f32>(intensity, intensity)).rgb;

    let light = ramp * camera.light_diffuse.rgb + camera.light_ambient.rgb;
    let shaded = base.rgb * light;

    // the pixel shader writes alpha 1 outright
    return vec4<f32>(mix(lit_colour(in), shaded, camera.flags.y), 1.0);
}
