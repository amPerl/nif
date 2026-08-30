@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let lit = lit_colour(in);
    let texel = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in));
    let slot1 = textureSample(slot1_texture, slot1_sampler, slot_uv(1u, in));
    let slot2 = textureSample(slot2_texture, slot2_sampler, slot_uv(2u, in));

    // the dark map accumulates before the base map, which then modulates onto it, so it
    // multiplies. An absent slot is white and changes nothing.
    let replace = model.sources.w * camera.flags.x;
    let base = texel.rgb * slot1.rgb;
    // the glow map is added in a ONE,ONE pass after everything, so it is unlit
    // the glow map and the environment map both add after the base modulates, which is where
    // the engine puts them: a stage each, both D3DTOP_ADD, neither touched by the lighting
    let shaded = mix(lit * base, base, replace) + slot2.rgb + environment(in);

    // alpha follows the same source as diffuse, and the texture modulates it like the colour
    let alpha_src = mix(model.diffuse.a, in.color.a, model.sources.y);
    let alpha = mix(alpha_src, alpha_src * texel.a, camera.flags.y);
    if (model.alpha.y > 0.5 && alpha <= model.alpha.x) {
        discard;
    }
    return vec4<f32>(mix(lit, shaded, camera.flags.y), alpha);
}
