---
sidebar_position: 2
---

# Tattvas

A tattva is any object that can be added to a scene — shapes, text, composites, procedural geometry. Every tattva has a position, visibility, opacity, and can be targeted by animations.

## Adding a tattva

```rust
let id = scene.add_tattva(Circle::new(1.0, 64, Vec4::new(1.0, 0.5, 0.0, 1.0)), Vec3::new(0.0, 0.0, 0.0));
```

`add_tattva` returns a `TattvaId` you use to animate or reference the object later.

## Primitives

All primitives live under `murali::frontend::collection::primitives`.

### Circle

```rust
Circle::new(radius: f32, segments: u32, color: Vec4)
```

### Square

```rust
Square::new(size: f32, color: Vec4)
```

### Rectangle

```rust
Rectangle::new(width: f32, height: f32, color: Vec4)
```

### Ellipse

```rust
Ellipse::new(rx: f32, ry: f32, segments: u32, color: Vec4)
```

### Line

```rust
Line::new(start: Vec3, end: Vec3, thickness: f32, color: Vec4)
```

### Polygon

```rust
// Regular polygon with n sides
Polygon::regular(sides: u32, radius: f32, color: Vec4)
```

### Cube

```rust
Cube::new(size: f32, color: Vec4)
```

### Path

A freeform path built from a list of points.

```rust
use murali::frontend::collection::primitives::path::Path;

let path = Path::new(vec![
    Vec3::new(-2.0, 0.0, 0.0),
    Vec3::new(0.0, 2.0, 0.0),
    Vec3::new(2.0, 0.0, 0.0),
], 0.05, Vec4::new(1.0, 1.0, 1.0, 1.0));
```

## Text

Text lives under `murali::frontend::collection::text`.

### Label

Lightweight glyph-based text, good for labels and numbers.

```rust
use murali::frontend::collection::text::label::Label;

Label::new("Hello, world!", 0.4)
    .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0))
```

The second argument is the font size in world units.

### Latex

Renders a LaTeX expression via embedded compilation → SVG → RGBA texture.

```rust
use murali::frontend::collection::text::latex::Latex;

Latex::new(r"E = mc^2", 0.6)
    .with_color(Vec4::new(0.98, 0.88, 0.38, 1.0))
```

Requires the `typst_embedded` feature (enabled by default).

## Composite tattvas

### Axes

```rust
use murali::frontend::collection::composite::axes::Axes;

let mut axes = Axes::new((-5.0, 5.0), (-3.0, 3.0));
axes.x_step = 1.0;
axes.y_step = 1.0;
axes.thickness = 0.03;
axes.tick_size = 0.18;
axes.color = Vec4::new(0.75, 0.79, 0.85, 1.0);
scene.add_tattva(axes, Vec3::ZERO);
```

## Mutating tattva properties

After adding a tattva you can mutate its properties directly:

```rust
if let Some(t) = scene.get_tattva_any_mut(id) {
    let mut props = t.props().write();
    props.visible = false;
    props.opacity = 0.0;
}
```
