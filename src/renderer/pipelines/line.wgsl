struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32, @builtin(instance_index) i_idx: u32) -> VertexOutput {
    // In a real implementation, we'd pass LineData via a Storage Buffer.
    // For this boilerplate, imagine we are fetching the start/end from a buffer.
    var out: VertexOutput;
    out.clip_position = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Placeholder
    out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}