// src/renderer/shaders/text.wgsl

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct Uniforms {
    mvp: mat4x4<f32>,
    alpha: f32,
    _pad0: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var text_tex: texture_2d<f32>;
@group(1) @binding(1)
var text_sampler: sampler;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
) -> VertexOut {
    var out: VertexOut;
    out.pos = uniforms.mvp * vec4(position, 1.0);
    out.uv = uv;
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let sampled = textureSample(text_tex, text_sampler, in.uv);
    return vec4<f32>(sampled.rgb * in.color.rgb, sampled.a * in.color.a * uniforms.alpha);
}
