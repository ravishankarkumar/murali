---
sidebar_position: 6
---

# Text & LaTeX pipeline

Murali has two text backends that share the same final step (texture quad), but differ significantly in how they produce the texture.

## Label (glyph atlas)

`Label` uses [fontdue](https://github.com/mooman219/fontdue) for CPU-side glyph rasterization.

The pipeline:

1. `LabelResources` is initialized once and holds the font + a glyph atlas texture
2. `layout_label` computes glyph positions and UVs for the given string and world height
3. `build_label_mesh` produces a `Mesh` with one quad per glyph, sampling from the atlas
4. The atlas texture is uploaded once as a bind group and reused for all labels

The atlas is a single RGBA texture. All glyphs share it, so multiple labels in a scene use the same bind group — efficient for the GPU.

Labels are best for short strings, numbers, and axis annotations where you don't need math typesetting.

## LaTeX

LaTeX uses an external `latex` + `dvisvgm` toolchain to compile expressions to SVG, then rasterizes to RGBA.

The pipeline:

1. `compile_latex(source, cache_dir)` — shells out to `latex` + `dvisvgm`, caches the SVG to disk in a temp directory keyed by source hash
2. `rasterize_svg` — uses `resvg` + `tiny-skia` to render the SVG to RGBA at a resolution derived from the viewport height
3. The RGBA is uploaded as a texture and rendered as a single textured quad
4. `normalized_world_height` computes the quad's world-space dimensions from the raster metrics

The disk cache means repeated renders of the same expression are fast — only the first compile is slow.

Requires `latex` and `dvisvgm` to be installed on the system. Use `murali doctor` to check.

## Typst (default math backend)

Typst is the preferred math backend. It's embedded directly in the binary — no external tools required.

The pipeline:

1. `TypstBackend::render_to_svg(source, pt_size)` — compiles Typst markup to SVG in-process
2. `rasterize_svg_to_rgba` — same `resvg` rasterizer as LaTeX
3. The result is cached in an LRU cache (`TypstRasterCache`, capacity 128) keyed by `"{height}::{source}"`
4. Uploaded as a texture quad, same as LaTeX

The LRU cache means frequently used expressions (axis labels, repeated formulas) are only rasterized once per session.

The scale factor for rasterization is clamped between 1× and 8× based on the viewport height and the requested world height, keeping texture resolution reasonable.

## Shared final step

Both LaTeX and Typst end up calling `build_textured_quad`:

```rust
fn build_textured_quad(
    raster_width: u32,
    raster_height: u32,
    world_height: f32,
    color: Vec4,
) -> Mesh
```

This produces a rectangle with the correct aspect ratio in world space. The `color` is multiplied in the fragment shader, so you can tint expressions.
