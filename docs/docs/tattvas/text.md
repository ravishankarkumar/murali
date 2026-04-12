---
sidebar_position: 2
---

# Text

All text tattvas live under `murali::frontend::collection::text`.

## Label

Lightweight glyph-based text using fontdue. Best for short strings, numbers, and axis annotations.

```rust
use murali::frontend::collection::text::label::Label;

Label::new(text: impl Into<String>, world_height: f32)
    .with_color(Vec4)
```

`world_height` is the font size in world units.

Supports character reveal for typewriter and reveal animations:

```rust
// Show 60% of characters, centered (reveal mode)
Label::new("Hello, world!", 0.4)
    .with_char_reveal(0.6)

// Typewriter mode: text grows left-to-right, left edge stays fixed
Label::new("Hello, world!", 0.4)
    .with_char_reveal(0.6)
    // set typewriter_mode = true directly on the struct
```

## Latex

Compiles a LaTeX expression via an external `latex` + `dvisvgm` toolchain, rasterizes the SVG to RGBA, and renders as a textured quad.

```rust
use murali::frontend::collection::text::latex::Latex;

Latex::new(source: impl Into<String>, world_height: f32)
    .with_color(Vec4)
```

Requires `latex` and `dvisvgm` installed on the system. Results are cached to disk (temp dir) by source hash, so repeated renders are fast.

Supports `with_char_reveal` for progressive reveal animations, same as `Label`.

## Typst

Renders a Typst expression using the embedded Typst compiler — no external tools required.

```rust
use murali::frontend::collection::text::typst::Typst;

Typst::new(source: impl Into<String>, world_height: f32)
    .with_color(Vec4)
```

Typst markup supports math, text, and layout. Results are cached in an LRU cache (128 entries) keyed by source + height.

For math expressions, Typst is the preferred backend over Latex since it requires no external tooling.

## CodeBlock

Renders syntax-highlighted code using the Typst backend's `#raw` block.

```rust
use murali::frontend::collection::text::code_block::CodeBlock;

CodeBlock::new(code: impl Into<String>, language: impl Into<String>, world_height: f32)
    .with_color(Vec4)
```

```rust
scene.add_tattva(
    CodeBlock::new("fn main() {\n    println!(\"Hello!\");\n}", "rust", 0.28),
    Vec3::new(0.0, 0.0, 0.0),
);
```

The `world_height` is the per-line height. Bounds are estimated from line count and max line length.
