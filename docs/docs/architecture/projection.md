---
sidebar_position: 5
---

# Projection & RenderPrimitive

Projection is the step that converts a tattva's state into a list of `RenderPrimitive` values. It's a pure CPU operation — no GPU involvement.

## The Project trait

Every tattva state type implements `Project`:

```rust
pub trait Project: Send + Sync {
    fn project(&self, ctx: &mut ProjectionCtx);
}
```

The implementation pushes primitives into `ctx`:

```rust
impl Project for Circle {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // tessellate a circle into triangles
        // push a RenderPrimitive::Mesh
        ctx.push_mesh(mesh);
    }
}
```

## ProjectionCtx

`ProjectionCtx` is passed to `project()` and accumulates the output:

```rust
pub struct ProjectionCtx {
    pub props: SharedProps,       // position, rotation, scale, opacity
    pub primitives: Vec<RenderPrimitive>,
}
```

The `props` are available so the projection can take position or scale into account if needed (e.g. for size-dependent tessellation).

## RenderPrimitive

The output of projection is a `Vec<RenderPrimitive>`:

```rust
pub enum RenderPrimitive {
    Mesh(Box<Mesh>),
    Line { start, end, thickness, color, dash_length, gap_length, dash_offset },
    Text { content, height, color, offset },
    Latex { source, height, color, offset },
    Typst { source, height, color, offset },
}
```

- `Mesh` — pre-tessellated triangle data, uploaded directly to a vertex buffer
- `Line` — endpoint data, geometry generated in the vertex shader
- `Text` — glyph layout request, resolved by `LabelResources` into a mesh
- `Latex` — LaTeX source, compiled → SVG → RGBA → texture quad
- `Typst` — Typst source, same pipeline as Latex but using the Typst backend

## Mesh format

A `Mesh` contains:

```rust
pub struct Mesh {
    pub data: MeshData,
    pub indices: Vec<u32>,
}

pub enum MeshData {
    Empty,
    Mesh(Vec<MeshVertex>),   // position + UV + color
    Text(Vec<TextVertex>),   // position + UV (color comes from uniform)
}
```

Tessellation is done with `lyon_tessellation` for filled shapes and custom code for parametric surfaces.
