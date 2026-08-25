// Toon shading with the outline done per pixel rather than as a hull, which is what separates
// this from the cartoon techniques despite the shared name fragment. One pass, no expanded
// shell: a pixel whose surface turns far enough away from the camera is simply painted the
// outline colour instead of shaded.
//
//     lit    = max(0, N dot L) * lightDiffuse
//     luma   = dot(lit, YUV weights)
//     colour = base * MaterialDiffuse * ramp(luma)
//     outline where N dot V is at or below outlineThickness
//
// The original lights with two directional lights and sums them before the luminance transfer.
// There is one light here. That transfer is a real colour conversion in this shader, unlike the
// plain ToonShading, where the same weights are applied to a scalar and only broadcast it.
//
// The outline test comes out of a `cnd` against 0.5 on a value the vertex stage biased and
// scaled, which reduces to `N dot V <= outlineThickness`. Reducing it needs the scale to be the
// half the technique declares; the vertex program's own comment lists it as 1, and at 1 there is
// no outline at any thickness. The declared constant is what runs.

// the YUV luminance weights the technique declares
const LUMA: vec3<f32> = vec3<f32>(0.299, 0.587, 0.114);

// outlineColor is a colour attribute, and nothing binds those yet, so this is what it declares
const OUTLINE_COLOR: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let normal = surface_normal(in);
    let to_eye = normalize(camera.eye.xyz - in.world);
    let to_light = -camera.light_dir.xyz;

    let lit = max(0.0, dot(normal, to_light)) * camera.light_diffuse.rgb;
    let luma = dot(lit, LUMA);

    let base = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in));
    // the ramp is a 1D gradient read at the luminance, and its point filter is what bands it
    let ramp = textureSample(slot1_texture, slot1_sampler, vec2<f32>(luma, luma)).rgb;

    let shaded = base.rgb * model.diffuse.rgb * ramp;
    let facing = dot(normal, to_eye);
    let colour = select(shaded, OUTLINE_COLOR, facing <= model.params.x);

    // the pixel shader writes alpha 1 outright
    return vec4<f32>(mix(lit_colour(in), colour, camera.flags.y), 1.0);
}
