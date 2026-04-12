---
sidebar_position: 8
---

# Graphs & plots

Murali has a graph module for plotting functions and data in 2D, and parametric surfaces in 3D.

## Setup: NumberPlane + Axes

The typical pattern is to layer a `NumberPlane` grid behind `Axes`, then add your graph on top.

```rust
use murali::frontend::collection::composite::{axes::Axes, number_plane::NumberPlane};

scene.add_tattva(
    NumberPlane::new((-5.0, 5.0), (-3.5, 3.5)).with_step(1.0),
    Vec3::ZERO,
);

scene.add_tattva(
    Axes::new((-5.0, 5.0), (-3.5, 3.5))
        .with_step(1.0)
        .with_thickness(0.03)
        .with_tick_size(0.16)
        .with_color(Vec4::new(0.86, 0.89, 0.93, 1.0)),
    Vec3::ZERO,
);
```

## FunctionGraph

Plots a Rust function `f32 -> f32` over an x range.

```rust
use murali::frontend::collection::graph::function_graph::FunctionGraph;

fn cubic(x: f32) -> f32 {
    0.08 * x * x * x - 0.55 * x
}

scene.add_tattva(
    FunctionGraph::new((-4.6, 4.6), cubic).with_samples(220),
    Vec3::ZERO,
);
```

- `with_samples(n)` — number of line segments (default is reasonable, increase for smooth curves)
- The function receives x in world units and returns y in world units

## ScatterPlot

Plots a list of `Vec2` points as dots.

```rust
use murali::frontend::collection::graph::scatter_plot::ScatterPlot;
use glam::vec2;

scene.add_tattva(
    ScatterPlot::new(vec![
        vec2(-3.8, cubic(-3.8)),
        vec2(0.0,  cubic(0.0)),
        vec2(3.8,  cubic(3.8)),
    ]),
    Vec3::ZERO,
);
```

## ParametricSurface (3D)

Renders a 3D surface defined by a parametric function `(u, v) -> Vec3`.

```rust
use murali::frontend::collection::graph::parametric_surface::ParametricSurface;
use murali::frontend::collection::composite::axes3d::Axes3D;
use std::f32::consts::PI;

// 3D axes
scene.add_tattva(
    Axes3D::new((-1.5, 1.5), (-1.5, 1.5), (-1.5, 1.5))
        .with_step(0.5)
        .with_axis_thickness(0.04),
    Vec3::ZERO,
);

// Sphere
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

- First two args are `(u_range, v_range)`
- The closure maps `(u, v)` to a world-space `Vec3`
- `with_samples(u_steps, v_steps)` controls mesh resolution

For 3D scenes, set the camera back further and use the orbit controller to explore:

```rust
scene.camera_mut().position = Vec3::new(3.0, 3.0, 6.0);
```
