struct Uniforms {
    background: vec4f,
    antialias_width: f32,
    octave_divisions: f32,
    baseline_spacing: f32,
    ramp_power: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexIn {
    @builtin(vertex_index) vertex_index: u32,
}

struct VertexOut {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let uv = vec2f(vec2u((in.vertex_index << 1) & 2, in.vertex_index & 2));
    let position = vec4f(uv * 2.0 - 1.0, 0.0, 1.0);
    return VertexOut(position, uv);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    let time = shaded_contours(
        in.texcoord.x,
        uniforms.baseline_spacing,
        uniforms.antialias_width,
        uniforms.ramp_power,
        uniforms.octave_divisions,
    );
    let physical = shaded_contours(
        in.texcoord.y,
        uniforms.baseline_spacing,
        uniforms.antialias_width,
        uniforms.ramp_power,
        uniforms.octave_divisions,
    );
    let value = (1.0 - time) * (1.0 - physical);
    let color = value * uniforms.background;
    return vec4f(color.rgb, 1.0);
}

/// Ricky Reusser's contour shading code as described in his
/// [Adaptive Contouring in Fragment Shaders Notebook](https://observablehq.com/@rreusser/locally-scaled-domain-coloring-part-1-contour-plots)
fn shaded_contours(f: f32, min_spacing: f32, antialias_width: f32, ramp_power: f32, octave_divisions: f32) -> f32 {
    // Compile-time constants
    const octaves: u32 = 4;
    const f_octaves: f32 = f32(octaves);

    let screen_space_grad: f32 = length(vec2f(dpdx(f), dpdy(f))) / abs(f);

    let local_octave = log2(screen_space_grad * min_spacing) / log2(octave_divisions);
    let contour_spacing = pow(octave_divisions, ceil(local_octave));

    var plot_var = log2(abs(f)) / contour_spacing;
    var width_scale = contour_spacing / screen_space_grad;

    var contour_sum: f32 = 0.0;
    for (var i: u32 = 0; i < octaves; i += 1) {
        // A weight which fades in the smallest octave and fades out the largest
        let t = f32(i + 1) - fract(local_octave);
        let weight = smoothstep(0.0, 1.0, t) * smoothstep(f_octaves, f_octaves - 1.0, t);

        // Shading for this octave is the contrast ramp with a chunk cut out of the corner for antialiasing
        let y = fract(plot_var);
        contour_sum += weight * min(
            contrast_function(y, ramp_power),
            (1.0 - y) * 0.5 * width_scale / antialias_width
        );

        // Adjust scales for the next octave
        width_scale *= octave_divisions;
        plot_var /= octave_divisions;
    }

    return contour_sum / f_octaves;
}

fn contrast_function(x: f32, power: f32) -> f32 {
    let y = 2.0 * x - 1.0;
    return 0.5 + 0.5 * pow(abs(y), power) * sign(y);
}

/// Quick approximate gamma correction
fn gamma_correct(value: f32) -> f32 {
    return pow(value, 0.454);
}
