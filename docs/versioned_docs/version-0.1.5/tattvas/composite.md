---
sidebar_position: 3
---

# Composite

Composite tattvas are made of multiple primitives. They live under `murali::frontend::collection::composite`.

## Axes

2D coordinate axes with tick marks.

```rust
use murali::frontend::collection::composite::axes::Axes;

let axes = Axes::new(x_range: (f32, f32), y_range: (f32, f32))
    .with_step(1.0)          // tick spacing on both axes
    .with_thickness(0.03)    // line thickness
    .with_tick_size(0.16)    // tick mark length
    .with_color(Vec4)
    .without_ticks();        // optional: hide ticks

scene.add_tattva(axes, Vec3::ZERO);
```

Fields can also be set directly:

```rust
let mut axes = Axes::new((-5.0, 5.0), (-3.0, 3.0));
axes.x_step = 1.0;
axes.y_step = 0.5;
axes.thickness = 0.03;
axes.tick_size = 0.18;
axes.color = Vec4::new(0.75, 0.79, 0.85, 1.0);
```

Projects to `RenderPrimitive::Line` segments — one for each axis and one per tick mark.

## NumberPlane

A full grid background with distinct axis and grid line colors.

```rust
use murali::frontend::collection::composite::number_plane::NumberPlane;

scene.add_tattva(
    NumberPlane::new((-5.0, 5.0), (-3.5, 3.5))
        .with_step(1.0),
    Vec3::ZERO,
);
```

Default colors: grid lines are a muted grey, axis lines are brighter. Both are configurable via struct fields:

```rust
let mut plane = NumberPlane::new((-5.0, 5.0), (-3.5, 3.5));
plane.grid_color = Vec4::new(0.25, 0.28, 0.33, 1.0);
plane.axis_color = Vec4::new(0.78, 0.82, 0.88, 1.0);
plane.grid_thickness = 0.01;
plane.axis_thickness = 0.03;
```

Typically layered behind `Axes` and a graph:

```rust
scene.add_tattva(NumberPlane::new(...), Vec3::ZERO);  // bottom
scene.add_tattva(Axes::new(...), Vec3::ZERO);          // middle
scene.add_tattva(FunctionGraph::new(...), Vec3::ZERO); // top
```

## Axes3D

3D coordinate axes for use with parametric surfaces and 3D graphs.

```rust
use murali::frontend::collection::composite::axes3d::Axes3D;

scene.add_tattva(
    Axes3D::new((-1.5, 1.5), (-1.5, 1.5), (-1.5, 1.5))
        .with_step(0.5)
        .with_axis_thickness(0.04),
    Vec3::ZERO,
);
```

Layout helpers (`to_edge`, `next_to`) don't apply meaningfully to 3D scenes — position manually.
