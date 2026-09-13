//! Vertex shader for building the spread of the trace on-the-fly from Implicit In-order Forests
//! sampled directly in the shader.
//! See [Tristan Hume's post](https://thume.ca/2021/03/14/iforests/) for reference.

struct VertexOutput {
    @builtin(position) normalized_position: vec4<f32>,
    // TODO: whatever is needed to place the average line.
    // TODO: Maybe trace color if not done via uniform.
};

@vertex
fn vs_main(
    @builtin(position) in_position: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.normalized_position = vec4<f32>(in_position.xy, 0.0, 1.0);
    return out;
}
