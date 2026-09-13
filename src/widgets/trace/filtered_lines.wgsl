//! Fragment shader for rendering the trace's spread and average line based on a variation of
//! E. Chan and F. Durand, “Fast Prefiltered Lines,” in GPU Gems 2: Programming Techniques for
//! High-Performance Graphics and General-Purpose Computation, M. Pharr, Ed. Boston, MA,
//! USA: Addison-Wesley Professional, 2005, ch. 22, pp. 345–359. [Online].
//! Available: https://developer.nvidia.com/gpugems/gpugems2/part-iii-high-quality-rendering/chapter-22-fast-prefiltered-lines

struct VertexOutput {
    @builtin(position) normalized_position: vec4<f32>,
    // TODO: whatever is needed to place the average line.
    // TODO: Maybe trace color if not done via uniform.
};

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
