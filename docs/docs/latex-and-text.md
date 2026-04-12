---
sidebar_position: 6
---

# LaTeX & text

Murali has two text backends: a lightweight glyph renderer for labels, and an embedded Typst/LaTeX compiler for math.

## Label

`Label` uses fontdue for fast glyph rendering. Best for short strings, numbers, and axis labels.

```rust
use murali::frontend::collection::text::label::Label;

scene.add_tattva(
    Label::new("Hello, world!", 0.4)
        .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0)),
    Vec3::new(0.0, 0.0, 0.0),
);
```

The second argument to `Label::new` is the font size in world units.

## Latex

`Latex` compiles a LaTeX expression using embedded Typst, rasterizes the SVG output to an RGBA texture, and renders it as a textured quad.

```rust
use murali::frontend::collection::text::latex::Latex;

scene.add_tattva(
    Latex::new(r"\int_0^1 x^2 \, dx = \frac{1}{3}", 0.72)
        .with_color(Vec4::new(0.98, 0.88, 0.38, 1.0)),
    Vec3::new(0.0, 1.0, 0.0),
);
```

The second argument is the rendered height in world units.

### Feature flag

LaTeX requires the `typst_embedded` feature, which is enabled by default:

```toml
[dependencies]
murali = { git = "https://github.com/ravishankarkumar/murali" }
# typst_embedded is on by default
```

To disable it (smaller binary, no LaTeX):

```toml
murali = { git = "...", default-features = false }
```

## Layout helpers

Push text to the edge of the screen:

```rust
use murali::frontend::layout::Direction;

let id = scene.add_tattva(Label::new("Title", 0.45), Vec3::ZERO);
scene.to_edge(id, Direction::Up, 0.35);   // 0.35 world units of padding
```

Available directions: `Up`, `Down`, `Left`, `Right`.
