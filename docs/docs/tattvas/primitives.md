---
sidebar_position: 1
---

# Primitives

Basic geometric shapes. All live under `murali::frontend::collection::primitives`.

## Circle

```rust
use murali::frontend::collection::primitives::circle::Circle;

Circle::new(radius: f32, segments: u32, color: Vec4)
```

`segments` controls smoothness — 32 is fine for small circles, 64+ for large ones.

Add a stroke outline:

```rust
Circle::new(1.0, 64, fill_color)
    .with_stroke(0.04, stroke_color)
```

## Square

```rust
use murali::frontend::collection::primitives::square::Square;

Square::new(size: f32, color: Vec4)
```

## Rectangle

```rust
use murali::frontend::collection::primitives::rectangle::Rectangle;

Rectangle::new(width: f32, height: f32, color: Vec4)
```

## Ellipse

```rust
use murali::frontend::collection::primitives::ellipse::Ellipse;

Ellipse::new(rx: f32, ry: f32, segments: u32, color: Vec4)
```

## Line

```rust
use murali::frontend::collection::primitives::line::Line;

Line::new(start: Vec3, end: Vec3, thickness: f32, color: Vec4)
```

Lines are rendered via the GPU line pipeline — geometry is generated in the vertex shader, not uploaded as vertex data. Supports dashed lines:

```rust
Line::new(start, end, 0.04, color)
    .with_dash(dash_length, gap_length)
```

## Polygon

```rust
use murali::frontend::collection::primitives::polygon::Polygon;

// Regular polygon with n sides
Polygon::regular(sides: u32, radius: f32, color: Vec4)
```

## Arrow

```rust
use murali::frontend::collection::primitives::arrow::Arrow;
use glam::Vec2;

// Default tip proportions
Arrow::with_default_tip(start: Vec2, end: Vec2, thickness: f32, color: Vec4)

// Custom tip
Arrow::new(start, end, thickness, tip_length, tip_width, color)
```

`start` and `end` are 2D. The arrow is placed at the `Vec3` position passed to `add_tattva`.

## Path

A freeform path built from segments.

```rust
use murali::frontend::collection::primitives::path::Path;

let path = Path::new(vec![
    Vec3::new(-2.0, 0.0, 0.0),
    Vec3::new(0.0, 2.0, 0.0),
    Vec3::new(2.0, 0.0, 0.0),
], thickness, color);
```

## Cube

```rust
use murali::frontend::collection::primitives::cube::Cube;

Cube::new(size: f32, color: Vec4)
```

A 3D wireframe cube. Useful for 3D scene orientation.
