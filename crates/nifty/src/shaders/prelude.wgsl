struct Camera {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    flags: vec4<f32>,
    // the direction the light travels, with the rim strength in w
    light_dir: vec4<f32>,
    light_ambient: vec4<f32>,
    light_diffuse: vec4<f32>,
    light_specular: vec4<f32>,
};
struct Model {
    model: mat4x4<f32>,
    diffuse: vec4<f32>,
    emissive: vec4<f32>,
    // emissive from vertex colour, diffuse from vertex colour, lighting enabled, apply replace
    sources: vec4<f32>,
    // alpha test threshold, alpha test enabled
    alpha: vec4<f32>,
    // two rows per bound slot. The third row is always (0, 0, 1), and row0.w names which uv
    // set the slot reads. Which slots these are is the shape's shader's choice.
    uv: array<vec4<f32>, 8>,
    // whatever the shape's shader wants to be told, which is its own attributes at the values
    // the shape supplies. Unread by the shaders that only combine textures.
    params: vec4<f32>,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> model: Model;
// Four texture slots. Which map each holds is the shape's shader's choice: the fixed function
// path binds base, dark and glow, and a custom shader binds whatever its source reads.
@group(2) @binding(0) var slot0_texture: texture_2d<f32>;
@group(2) @binding(1) var slot0_sampler: sampler;
@group(2) @binding(2) var slot1_texture: texture_2d<f32>;
@group(2) @binding(3) var slot1_sampler: sampler;
@group(2) @binding(4) var slot2_texture: texture_2d<f32>;
@group(2) @binding(5) var slot2_sampler: sampler;
@group(2) @binding(6) var slot3_texture: texture_2d<f32>;
@group(2) @binding(7) var slot3_sampler: sampler;

// A pass that moves the vertex has its own `displace` spliced in here, above the vertex stage
// that calls it. WGSL has no forward declarations, so the order matters.
// <displacement>

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) uv1: vec2<f32>,
    @location(4) uv2: vec2<f32>,
    @location(5) normal: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) uv: vec2<f32>,
    @location(4) uv1: vec2<f32>,
    @location(5) uv2: vec2<f32>,
) -> VertexOut {
    let world = model.model * vec4<f32>(position, 1.0);
    // the model matrix is a rigid transform plus a uniform scale, so
    // rotating the normal by it is enough and an inverse transpose would be the same direction
    let world_normal = (model.model * vec4<f32>(normal, 0.0)).xyz;
    // a pass that draws a hull moves the vertex here; every other pass leaves it alone
    let moved = displace(world.xyz, world_normal);
    var out: VertexOut;
    out.clip = camera.view_proj * vec4<f32>(moved, 1.0);
    out.world = moved;
    out.color = color;
    out.uv = uv;
    out.uv1 = uv1;
    out.uv2 = uv2;
    out.normal = world_normal;
    return out;
}

/// The uv a slot samples at, after its own set choice and its own transform.
fn slot_uv(slot: u32, in: VertexOut) -> vec2<f32> {
    let row0 = model.uv[slot * 2u];
    let row1 = model.uv[slot * 2u + 1u];
    var source = in.uv;
    if (row0.w > 1.5) {
        source = in.uv2;
    } else if (row0.w > 0.5) {
        source = in.uv1;
    }
    let uvw = vec3<f32>(source, 1.0);
    return vec2<f32>(dot(row0.xyz, uvw), dot(row1.xyz, uvw));
}

@fragment
fn fs_line(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}

@fragment
fn fs_wire(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(0.35, 0.95, 0.55, 1.0);
}

@fragment
fn fs_highlight(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.65, 0.15, 1.0);
}

/// The lit colour a shape would have with no texture, which every shader starts from.
/// Split out of the fixed function path so a custom shader gets the same lighting model
/// rather than inventing its own.
/// The shading normal. The geometry's own normal where it has one, which is what the engine
/// lit with; a face normal off the derivatives where it does not, which is flat but never zero.
fn surface_normal(in: VertexOut) -> vec3<f32> {
    if (dot(in.normal, in.normal) > 1e-8) {
        return normalize(in.normal);
    }
    // the sign of a derivative normal depends on framebuffer handedness, so orient it
    // toward the eye rather than trusting it, which holds with culling off too
    var derived = normalize(cross(dpdx(in.world), dpdy(in.world)));
    let to_eye = normalize(camera.eye.xyz - in.world);
    if (dot(derived, to_eye) < 0.0) {
        derived = -derived;
    }
    return derived;
}

fn lit_colour(in: VertexOut) -> vec3<f32> {
    let normal = surface_normal(in);
    let to_eye = normalize(camera.eye.xyz - in.world);

    // the light travels away from its source, so the direction back to it is negated
    let to_light = -camera.light_dir.xyz;
    let lambert = clamp(dot(normal, to_light), 0.0, 1.0);
    let fill = camera.light_dir.w * clamp(dot(normal, to_eye), 0.0, 1.0);

    let shade = camera.light_ambient.rgb + camera.light_diffuse.rgb * lambert + vec3<f32>(fill);

    // the texture modulates the lit colour, so emissive is inside the multiply, not over it:
    // emissive 1,1,1 is a full brightness texture, not white. shade replaces the light sum.
    let vertex = in.color.rgb;
    let emissive_src = mix(model.emissive.rgb, vertex, model.sources.x);
    let diffuse_src = mix(model.diffuse.rgb, vertex, model.sources.y);
    let material_lit = clamp(
        emissive_src + diffuse_src * shade * model.sources.z,
        vec3<f32>(0.0),
        vec3<f32>(1.0)
    );

    let plain = vec3<f32>(0.78, 0.80, 0.84) * shade;
    let lit = mix(plain, material_lit, camera.flags.x);
    return lit;
}
