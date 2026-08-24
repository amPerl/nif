// From the game's own ActionGameCartoonFX.fx, technique ActionGameCartoonFX.
//
// A decal composited over a flat body colour by the decal's own alpha, then lit. The source
// splits the work between vertex and pixel shader, but it reduces: the vertex colour it carries
// is `MaterialColor * LightColor`, so the pixel shader's
//     (Decal.rgb * Decal.a) * LightColor + (1 - Decal.a) * MaterialColor * LightColor
// is just `LightColor * mix(MaterialColor, Decal.rgb, Decal.a)`.
//
// `MaterialColor` is an `ATTRIBUTE` defaulting to white, and every corpus shape carries no shader
// extra data at all, so white is what it resolves to rather than anything read from the file.
//
// The file also declares a `ToonTex` ramp, but its `NTM` binding is commented out and no
// technique here samples it: the header says the toon outline path is deprecated. 5,111 of the
// 5,500 shapes still carry `ToonRamp.bmp` in a shader slot, left over from when it was read.

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let decal = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in));

    // the source lights with the geometry's own normal against the light's direction of travel
    let intensity = max(0.0, dot(surface_normal(in), -camera.light_dir.xyz));
    let light = camera.light_ambient.rgb + intensity * camera.light_diffuse.rgb;

    let body = vec3<f32>(1.0, 1.0, 1.0);
    let shaded = light * mix(body, decal.rgb, decal.a);

    // the pixel shader returns alpha 1 outright, so the decal's alpha places it and never
    // makes the surface see through
    return vec4<f32>(mix(lit_colour(in), shaded, camera.flags.y), 1.0);
}
