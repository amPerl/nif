@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let lit = lit_colour(in);
    let map0 = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in)).rgb;
    let map1 = textureSample(slot1_texture, slot1_sampler, slot_uv(1u, in)).rgb;
    let detail = textureSample(slot2_texture, slot2_sampler, slot_uv(2u, in)).rgb;

    // TOP_BlendDiffuseAlpha is arg1 * a + arg2 * (1 - a), and arg1 holds the first map, so an
    // alpha of 1 selects map 0. The weight is a vertex alpha, which the hardware clamps and
    // which a file can store outside the range.
    let weight = clamp(in.color.a, 0.0, 1.0);
    let blended = mix(map1, map0, weight);

    // Modulate2x on the detail stage, with a half standing in when there is no detail map
    let shaded = blended * lit * detail * 2.0;

    // the vertex alpha is a blend weight rather than an opacity, which is why the shader turns
    // AlphaBlendEnable and AlphaTestEnable off, so nothing here can be transparent
    return vec4<f32>(mix(lit, shaded, camera.flags.y), 1.0);
}
