// Vertex shader

struct CurcleUniform {
    ctr: vec2<f32>;
    radius: f32;
};
[[group(1), binding(0)]]
var<uniform> param: CurcleUniform;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> vec2<f32> {
    return model.position;
}

[[stage(fragment)]]
fn fs_main(in: vec2<f32>) -> [[location(0)]] vec4<f32> {
    let d = distance(param.ctr, in);
    let color = vec4<f32>(d, d, d, 1.0);
    return color;
}