// A pass that leaves the geometry where it is, which is every pass but an outline hull.
fn displace(position: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    return position;
}
