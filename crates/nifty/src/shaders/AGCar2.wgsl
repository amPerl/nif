// From the game's own AGC_Shader.fx, technique AGCar2, the `vs_1_1 VS_Car2` declaration. The
// file declares that technique name twice and the other one is `NBTMethod = "ATI"`, normal
// mapped, which needs tangent space the geometry does not carry.
//
// A mask texture decides what each pixel is. Its red and green select which of the material's
// two colours the body shows, and where both are zero the pixel is a window drawn from a second
// decal instead:
//
//     mask.r > 0   body over the material ambient
//     mask.g > 0   body over the material diffuse
//     otherwise    window, tinted by the material specular where mask.b says so
//
// The body composite is the same one `ActionGameCartoonFX` uses, a decal over a flat colour by
// the decal's own alpha, lit. What is new here is that the colour under it comes from the
// material rather than from an attribute, and that one shape carries both a body and its glass.

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // the mask reads the set its own TexDesc names, which is the second one on most shapes
    let mask = textureSample(slot2_texture, slot2_sampler, slot_uv(2u, in));

    // the source lights per vertex and rebuilds the same sum in the pixel shader
    let intensity = max(0.0, dot(surface_normal(in), -camera.light_dir.xyz));
    let light = camera.light_ambient.rgb + intensity * camera.light_diffuse.rgb;

    var colour: vec3<f32>;
    var alpha: f32;

    if (mask.r > 0.0 || mask.g > 0.0) {
        let body = select(model.diffuse.rgb, model.ambient.rgb, mask.r > 0.0);
        let decal = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in));
        colour = (decal.rgb * decal.a + (1.0 - decal.a) * body) * light;
        // the body is opaque at the material's own alpha, whatever the decal carries
        alpha = model.diffuse.a;
    } else {
        let decal = textureSample(slot1_texture, slot1_sampler, slot_uv(1u, in));
        let tint = select(vec3<f32>(1.0, 1.0, 1.0), model.specular.rgb, mask.b > 0.0);
        colour = decal.rgb * tint * light;
        // and the glass takes the decal's alpha as well as the material's
        alpha = decal.a * model.diffuse.a;
    }

    return vec4<f32>(mix(lit_colour(in), colour, camera.flags.y), alpha);
}
