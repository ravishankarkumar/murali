// src/renderer/shaders/line.wgsl

struct LineData {
    start: vec4<f32>,
    end: vec4<f32>,
    color: vec4<f32>,
    props: vec4<f32>, // x = thickness
};

struct Lines {
    data: array<LineData>,
};

@group(0) @binding(0) var<storage, read> lines: Lines;

struct Camera {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) dist: f32,
    @location(2) dash_params: vec3<f32>, // x=dash, y=gap, z=offset
};

@vertex
fn vs_main(
    @builtin(vertex_index) v_idx: u32,
    @builtin(instance_index) i_idx: u32,
) -> VertexOutput {
    let line = lines.data[i_idx];
    let start = line.start.xyz;
    let end = line.end.xyz;
    let thickness = line.props.x;
    let dash_params = line.props.yzw;

    let dir = normalize(end - start);
    let up = vec3<f32>(0.0, 0.0, 1.0);
    let side = normalize(cross(dir, up)) * (thickness * 0.5);

    var pos: vec3<f32>;
    var dist: f32;
    let quad_idx = v_idx % 6u;
    let line_len = length(end - start);

    if (quad_idx == 0u) { pos = start - side; dist = 0.0; }
    else if (quad_idx == 1u) { pos = start + side; dist = 0.0; }
    else if (quad_idx == 2u) { pos = end - side; dist = line_len; }
    else if (quad_idx == 3u) { pos = start + side; dist = 0.0; }
    else if (quad_idx == 4u) { pos = end + side; dist = line_len; }
    else { pos = end - side; dist = line_len; }

    var out: VertexOutput;
    out.position = camera.view_proj * vec4<f32>(pos, 1.0);
    out.color = line.color;
    out.dist = dist;
    out.dash_params = dash_params;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dash = in.dash_params.x;
    let gap = in.dash_params.y;
    let offset = in.dash_params.z;

    if (dash > 0.0) {
        let total = dash + gap;
        if ((in.dist + offset) % total > dash) {
            discard;
        }
    }

    return in.color;
}