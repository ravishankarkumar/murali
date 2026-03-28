// src/renderer/shaders/triangle.wgsl

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>
};

// Uniform buffer with a 4x4 matrix
struct Uniforms {
    mvp: mat4x4<f32>,
    alpha: f32,
    _pad0: vec3<f32>,
};
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) color: vec4<f32>) -> VertexOut {
    var out: VertexOut;
    // apply MVP to transform positions
    out.pos = uniforms.mvp * vec4(position, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // Vertex alpha is multiplied by uniform alpha
    return vec4(in.color.rgb, in.color.a * uniforms.alpha);
}
