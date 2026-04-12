---
sidebar_position: 5
---

# Math

Math tattvas live under `murali::frontend::collection::math`. These are higher-level constructs for mathematical notation.

## EquationLayout

Renders a sequence of individually colored and animatable equation parts, laid out horizontally.

```rust
use murali::frontend::collection::math::equation::{EquationLayout, EquationPart};

scene.add_tattva(
    EquationLayout::new(
        vec![
            EquationPart::new("E").with_color(Vec4::new(1.0, 0.8, 0.2, 1.0)),
            EquationPart::new("=").with_color(Vec4::new(0.9, 0.9, 0.9, 1.0)),
            EquationPart::new("mc²").with_color(Vec4::new(0.4, 0.8, 1.0, 1.0)),
        ],
        0.5, // world_height
    ),
    Vec3::ZERO,
);
```

Each `EquationPart` can have:
- `.with_color(Vec4)` — individual color
- `.with_key(str)` — identity key for equation continuity animations
- `.with_opacity(f32)` — per-part opacity
- `.with_scale(f32)` — per-part size scale
- `.with_offset(Vec3)` — per-part positional nudge

Parts are laid out left-to-right, centered as a group. Gap between parts defaults to `world_height * 0.35`.

## Matrix

Renders a 2D matrix with bracket notation.

```rust
use murali::frontend::collection::math::matrix::Matrix;

scene.add_tattva(
    Matrix::new(
        vec![
            vec!["1", "2", "3"],
            vec!["4", "5", "6"],
            vec!["7", "8", "9"],
        ],
        0.4, // cell_height
    ),
    Vec3::ZERO,
);
```

Individual cells can be styled:

```rust
let mut m = Matrix::new(entries, 0.4);

// Highlight a cell
m.cell_mut(1, 1).unwrap().highlight = Some(Vec4::new(0.3, 0.6, 1.0, 0.3));

// Color a cell
m.cell_mut(0, 0).unwrap().color = Vec4::new(1.0, 0.5, 0.2, 1.0);

// Assign a key for animation continuity
m.cell_mut(0, 0).unwrap().key = Some("a11".to_string());
```

Bracket style is configurable:

```rust
let mut m = Matrix::new(entries, 0.4);
m.bracket_color = Vec4::new(0.9, 0.9, 0.9, 1.0);
m.bracket_thickness = 0.03;
m.h_gap = 0.5;  // horizontal gap between columns
m.v_gap = 0.2;  // vertical gap between rows
```
