//! Shader code for rendering traces entirely on the GPU.
//!
//! The vertex shader for building the spread of the trace on-the-fly from Implicit In-order Forests
//! sampled directly in the shader.
//! See [Tristan Hume's post](https://thume.ca/2021/03/14/iforests/) for reference.
//!
//! The fragment shader uses  a variation of
//! E. Chan and F. Durand, “Fast Prefiltered Lines,” in GPU Gems 2: Programming Techniques for
//! High-Performance Graphics and General-Purpose Computation, M. Pharr, Ed. Boston, MA,
//! USA: Addison-Wesley Professional, 2005, ch. 22, pp. 345–359. [Online].
//! Available: https://developer.nvidia.com/gpugems/gpugems2/part-iii-high-quality-rendering/chapter-22-fast-prefiltered-lines

const PI: f32 = 3.14159265359;
const TWO_PI: f32 = 6.28318530718;

struct Uniforms {
    line_color: vec4f,
    spread_alpha: f32,
    antialias_width: f32,
    stroke: f32,
    min_value: f32,
    max_value: f32,
    begin: f32,
    end: f32,
    range: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexIn {
    @location(0) position: vec2f,
}

struct VertexOut {
    @builtin(position) position: vec4f,
    @location(0) line_distance: f32,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let x = in.position.x;
    let time = uniforms.range * 0.5 * (x + 1.0) + uniforms.begin;
    let min_max_avg = sample(time);
    let value = select(min_max_avg.x, min_max_avg.y, in.position.y > 0.0);
    let y = map_physical_to_normalized(value);
    let position = vec4(in.position.x, y, 0.0, 1.0);
    let distance = min_max_avg.z - value;
    return VertexOut(position, distance);
}

fn sample(time: f32) -> vec3f {
    let value = 50.0 * sin(TWO_PI * time / 1000.0) + 50.0;
    let minimum = value - 5.0;
    let maximum = value + 5.0;
    let average = value;
    return vec3(minimum, maximum, average);
}

fn map_physical_to_normalized(value: f32) -> f32 {
    return 2.0 * (value - uniforms.min_value) / (uniforms.max_value - uniforms.min_value) - 1.0;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    let line_coverage = line_coverage(in.line_distance);
    let spread_color = vec4f(uniforms.line_color.rgb, uniforms.spread_alpha);
    let color = mix(spread_color, uniforms.line_color, line_coverage);
    return color;
}

fn line_coverage(line_distance: f32) -> f32 {
    let coverage = 1.0 - smoothstep(0.5, 1.5, abs(line_distance));
    return coverage;
}
