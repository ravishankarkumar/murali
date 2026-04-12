---
sidebar_position: 4
---

# Graphs

Graph tattvas live under `murali::frontend::collection::graph`.

## FunctionGraph

Plots a Rust function `f32 -> f32` over an x range as a polyline.

```rust
use murali::frontend::collection::graph::function_graph::FunctionGraph;

fn sine(x: f32) -> f32 { x.sin() }

scene.add_tattva(
    FunctionGraph::new((-5.0, 5.0), sine)
        .with_samples(200),
    Vec3::ZERO,
);
```

`with_samples(n)` controls the number of line segments. More samples = smoother curve, more primitives.

## ScatterPlot

Plots a list of `Vec2` points as dots.

```rust
use murali::frontend::collection::graph::scatter_plot::ScatterPlot;
use glam::vec2;

scene.add_tattva(
    ScatterPlot::new(vec![
        vec2(-2.0, 1.5),
        vec2(0.0, 0.0),
        vec2(2.0, -1.5),
    ]),
    Vec3::ZERO,
);
```

## ParametricCurve

A 2D parametric curve `t -> Vec2`.

```rust
use murali::frontend::collection::graph::parametric_curve::ParametricCurve;

scene.add_tattva(
    ParametricCurve::new((0.0, std::f32::consts::TAU), |t| {
        glam::vec2(t.cos(), t.sin())
    }).with_samples(128),
    Vec3::ZERO,
);
```

## ParametricSurface

A 3D surface defined by `(u, v) -> Vec3`. Renders as a wireframe mesh.

```rust
use murali::frontend::collection::graph::parametric_surface::ParametricSurface;
use std::f32::consts::PI;

scene.add_tattva(
    ParametricSurface::new(
        (0.0, PI),
        (0.0, 2.0 * PI),
        |u, v| Vec3::new(u.sin() * v.cos(), u.sin() * v.sin(), u.cos()),
    )
    .with_samples(40, 40)
    .with_color(Vec4::new(0.44, 0.84, 0.71, 1.0)),
    Vec3::ZERO,
);
```

`with_samples(u_steps, v_steps)` controls mesh resolution.

## VectorField

Displays arrows at grid points representing a vector function `Vec2 -> Vec2`.

```rust
use murali::frontend::collection::graph::vector_field::VectorField;

scene.add_tattva(
    VectorField::new(
        (-4.0, 4.0), (-3.0, 3.0),
        8, 6,
        |p| glam::vec2(-p.y, p.x), // rotation field
    )
    .with_color(Vec4::new(0.5, 0.7, 1.0, 0.8))
    .with_length_scale(0.4),
    Vec3::ZERO,
);
```

Color vectors by magnitude:

```rust
.with_color_fn(|magnitude| {
    let t = (magnitude / 3.0).clamp(0.0, 1.0);
    Vec4::new(t, 0.5, 1.0 - t, 0.8)
})
```

## StreamLines

Traces flow paths through a vector field using Euler integration.

```rust
use murali::frontend::collection::graph::stream_lines::{StreamLines, line_start_points};

scene.add_tattva(
    StreamLines::from_grid(
        (-4.0, 4.0), (-3.0, 3.0),
        6, 5,
        |p| glam::vec2(-p.y, p.x),
    )
    .with_color(Vec4::new(0.4, 0.8, 1.0, 0.7))
    .with_step_size(0.05)
    .with_max_steps(500)
    .with_bounds(glam::Vec2::new(-5.0, -4.0), glam::Vec2::new(5.0, 4.0)),
    Vec3::ZERO,
);
```

Helper functions for starting points:

```rust
// Points along a line
line_start_points(Vec2::new(-3.0, 0.0), Vec2::new(3.0, 0.0), 10)

// Points in a circle
circle_start_points(Vec2::ZERO, 2.0, 12)
```
