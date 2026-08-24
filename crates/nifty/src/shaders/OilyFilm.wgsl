// A thin film interference effect over a base map: the specular term picks a colour out of an
// interference ramp, and a warp map is added on top.
//
// The source summarises itself, and the summary is exact:
//     Oc = B * v0 + TF * v1 + WE
// where B is the base map, TF the interference ramp sampled at N dot V, WE the warp map sampled
// at (N dot V, WarpAlpha), v0 the diffuse lighting and v1 the specular. Its header claims the
// output alpha is WE; the code moves in WarpAlpha instead, and the code is what ran.
//
// The original lights with two directional lights and scales each specular term by the light's
// green channel, because green carries most of the luminance. Only one light exists here.
//
// `Exponent` and `WarpAlpha` are attributes with declared defaults, and a shape supplies its own
// only through shader extra data, which none carries.

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let exponent = model.params.y;
    let warp_alpha = model.params.x;

    let normal = surface_normal(in);
    let to_eye = normalize(camera.eye.xyz - in.world);
    let to_light = -camera.light_dir.xyz;

    // the interference ramp is read at N dot V, which stands in for how far light travels
    // through a film of even thickness
    let facing = clamp(dot(normal, to_eye), 0.0, 1.0);

    let half_vector = normalize(to_light + to_eye);
    let lambert = max(0.0, dot(normal, to_light));
    // the fixed function lit instruction gives no specular where the surface faces away
    var specular = 0.0;
    if (lambert > 0.0) {
        specular = pow(max(0.0, dot(half_vector, normal)), exponent);
    }

    let diffuse = camera.light_diffuse.rgb * lambert + camera.light_ambient.rgb;
    // scaled by the light's green channel, which is where most of the luminance sits
    let specular_colour = camera.light_specular.g * specular;

    let base = textureSample(slot0_texture, slot0_sampler, slot_uv(0u, in)).rgb;
    let film = textureSample(slot1_texture, slot1_sampler, vec2<f32>(facing, facing)).rgb;
    let warp = textureSample(slot2_texture, slot2_sampler, vec2<f32>(facing, warp_alpha)).rgb;

    let shaded = base * diffuse + film * specular_colour + warp;
    return vec4<f32>(mix(lit_colour(in), shaded, camera.flags.y), warp_alpha);
}
