// The hull the outline pass draws: every vertex pushed out along its own normal by
// outlineThickness, so the shell stands proud of the surface by that much everywhere.
//
// The original works between WorldView and Projection, displacing in view space. View is a
// rotation and a translation, which preserve length, so displacing the world position along the
// world normal by the same amount lands in the same place. The thickness is therefore a world
// distance rather than a screen width, and a distant object's outline thins with perspective.
fn displace(position: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    return position + normalize(normal) * model.params.x;
}
