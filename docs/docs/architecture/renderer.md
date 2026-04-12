---
sidebar_position: 4
---

# Renderer & pipelines

The `Renderer` owns all wgpu render pipelines and is responsible for encoding a single render pass per frame.

## Pipelines

There are three render pipelines:

### Mesh pipeline

Used for all filled geometry — circles, rectangles, polygons, parametric surfaces, etc.

- Vertex format: `MeshVertex` (position, UV, color)
- Bind groups: uniform (MVP + sampler), texture
- Blending: alpha blending
- Depth: write disabled, always pass (painter's algorithm — draw order matters)

### Text pipeline

Used for glyph atlas quads (Label) and LaTeX/Typst texture quads.

- Vertex format: `TextVertex` (position, UV)
- Bind groups: uniform (MVP + sampler), texture
- Blending: alpha blending
- Depth: `LessEqual`

### Line pipeline

Used for all `Line` primitives and anything that projects to `RenderPrimitive::Line`.

- No vertex buffer — geometry is generated in the vertex shader from a storage buffer
- Each line is 6 vertices (2 triangles forming a quad oriented along the line direction)
- Bind groups: line storage buffer, camera uniform
- Supports dashed lines via `dash_length`, `gap_length`, `dash_offset`

## Uniforms

Each mesh draw call gets a `Uniforms` slot in a pre-allocated uniform buffer:

```rust
pub struct Uniforms {
    pub mvp: [[f32; 4]; 4],  // model * view * projection
    pub alpha: f32,
    pub _padding: [f32; 3],
}
```

The buffer is allocated for up to 1000 drawables. Each slot is aligned to `min_uniform_buffer_offset_alignment` (typically 256 bytes). Dynamic offsets are used so all draws share one bind group.

## Frame loop

`render_scene` does the following each frame:

1. Acquire the next surface texture
2. Compute `view_proj` from the camera
3. Collect all `LineComponent` entities into a storage buffer
4. Collect all `MeshComponent` entities into a draw list
5. Begin a render pass (clear color + depth)
6. Draw lines (one instanced draw call for all lines)
7. Draw meshes one by one, writing MVP + alpha into the uniform buffer at the correct offset

## Headless rendering

For video export, `render_to_image` renders into an offscreen texture instead of the surface, then copies the result to a CPU-readable buffer via `map_async`. The padded row stride is handled correctly for wgpu's alignment requirements.

## DeviceManager

`DeviceManager` wraps the wgpu device, queue, surface, and config. It supports two modes:

- Windowed (`DeviceManager::new`) — creates a surface from a winit window, prefers sRGB format, high-performance adapter
- Headless (`DeviceManager::new_headless`) — no surface, uses `Rgba8UnormSrgb` format, falls back to low-power adapter if needed

The config is stored in a `RefCell` to allow interior mutability during resize without requiring `&mut self` everywhere.
