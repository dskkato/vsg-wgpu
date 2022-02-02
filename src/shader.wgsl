// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct CircleUniform {
    x: f32;
    y: f32;
    radius: f32;
};

[[group(0), binding(0)]]
var<uniform> param: CircleUniform;

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

fn draw_circle(coord: vec2<f32> , radius: f32) -> f32 {
    return step(length(coord), radius);
}

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {

    let d = draw_circle(
        vec2<f32>(in.position.x, in.position.y) - vec2<f32>(400.0, 300.0),
        10.0);
        // param.radius);
    return vec4<f32>(d, d, d, 1.0);
}