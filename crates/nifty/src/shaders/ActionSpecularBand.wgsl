// From the game's own ActionSpecularBand.fx, technique ActionSpecularBand.
//
// A sweeping highlight rather than a lit surface. It projects the normal and the direction to the
// camera onto the object's own XY plane, takes the cosine between them, and uses that as the u
// coordinate into the gloss texture. So the band moves around the body as the camera orbits.
//
// The pass is additive, `SRCBLEND = ONE, DESTBLEND = ONE`, so this draws on top of whatever
// already shaded the surface and can only add light.
//
// `Reflection` is an `ATTRIBUTE` defaulting to 100, and a shape supplies one only through shader
// extra data, which none carries, so 100 is what it resolves to: a very tight highlight that sits
// at the 0.1 floor across most of the body.

// the exponent the source names Reflection
const REFLECTION: f32 = 100.0;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // The source computes in object space, via `mul(vCameraPos, InvWorld) - Position`. A NIF
    // transform is a rotation with a single uniform scale, so normalizing the model matrix's
    // columns recovers that rotation and transposing it inverts it. No inverse needs uploading.
    let basis = mat3x3<f32>(
        model.model[0].xyz,
        model.model[1].xyz,
        model.model[2].xyz,
    );
    let inverse_rotation = transpose(mat3x3<f32>(
        normalize(basis[0]),
        normalize(basis[1]),
        normalize(basis[2]),
    ));

    let normal = normalize((inverse_rotation * surface_normal(in)).xy);
    let to_eye = normalize((inverse_rotation * (camera.eye.xyz - in.world)).xy);

    // the 0.1 floor is the source's, and it is what the body reads at away from the highlight
    let band = max(0.1, pow(max(0.0, dot(normal, to_eye)), REFLECTION));

    // the u coordinate is the band, the v is the map's own. The sampler mirrors in u, which the
    // shader pins rather than taking from the map's clamp mode.
    let gloss = textureSample(slot0_texture, slot0_sampler, vec2<f32>(band, slot_uv(0u, in).y));

    // the source scales by the specular light's red channel alone, not by its colour
    return vec4<f32>(gloss.rgb * camera.light_specular.r, band);
}
