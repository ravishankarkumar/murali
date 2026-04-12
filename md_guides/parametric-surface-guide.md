# Parametric Surface Guide

## Overview

`ParametricSurface` is a 3D visualization primitive that renders parametric surfaces defined by a function `f(u, v) -> Vec3`. This enables creation of complex 3D shapes like spheres, tori, and custom mathematical surfaces.

## Basic Usage

### Simple Sphere

```rust
use glam::Vec3;
use murali::frontend::collection::graph::parametric_surface::ParametricSurface;
use std::f32::consts::PI;

let sphere = ParametricSurface::new(
    (0.0, PI),           // u_range (theta: 0 to π)
    (0.0, 2.0 * PI),     // v_range (phi: 0 to 2π)
    |u, v| {
        let sin_u = u.sin();
        Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
    },
);

scene.add_tattva(sphere, Vec3::ZERO);
```

## Configuration

### Sampling Resolution

Control the mesh density with `with_samples(u_samples, v_samples)`:

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_samples(40, 40);  // Higher = smoother but slower
```

### Color

Set the surface color with `with_color(color)`:

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_color(Vec4::new(0.44, 0.84, 0.71, 1.0));
```

## Common Parametric Surfaces

### Sphere

Standard sphere with radius 1:

```rust
|u, v| {
    let sin_u = u.sin();
    Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
}
```

Parameters:
- `u_range`: `(0.0, PI)` - latitude
- `v_range`: `(0.0, 2*PI)` - longitude

### Torus

Torus with major radius R and minor radius r:

```rust
|u, v| {
    let r = 0.4;  // minor radius
    let R = 1.2;  // major radius
    let x = (R + r * v.cos()) * u.cos();
    let y = (R + r * v.cos()) * u.sin();
    let z = r * v.sin();
    Vec3::new(x, y, z)
}
```

Parameters:
- `u_range`: `(0.0, 2*PI)` - around the major circle
- `v_range`: `(0.0, 2*PI)` - around the minor circle

### Wavy Surface

A simple wave surface:

```rust
|u, v| {
    let x = u;
    let y = v;
    let z = u.sin() * v.cos();
    Vec3::new(x, y, z)
}
```

Parameters:
- `u_range`: `(-PI, PI)` - x-axis
- `v_range`: `(-PI, PI)` - y-axis

### Saddle Surface (Hyperbolic Paraboloid)

```rust
|u, v| {
    Vec3::new(u, v, u * u - v * v)
}
```

Parameters:
- `u_range`: `(-2.0, 2.0)`
- `v_range`: `(-2.0, 2.0)`

### Helicoid

A spiral surface:

```rust
|u, v| {
    let x = v * u.cos();
    let y = v * u.sin();
    let z = u;
    Vec3::new(x, y, z)
}
```

Parameters:
- `u_range`: `(0.0, 4*PI)` - height/rotation
- `v_range`: `(0.0, 2.0)` - radius

## Integration with 3D Axes

Combine with `Axes3D` for proper visualization:

```rust
use murali::frontend::collection::composite::axes3d::Axes3D;

scene.add_tattva(
    Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0))
        .with_step(1.0)
        .with_axis_thickness(0.04),
    Vec3::ZERO,
);

scene.add_tattva(surface, Vec3::ZERO);
```

## Performance Considerations

- **Sampling**: Higher `u_samples` and `v_samples` create smoother surfaces but increase mesh complexity
  - Typical: 32-40 samples per dimension
  - Maximum: 64+ samples (may impact performance)
  - Minimum: 8-16 samples (visible faceting)

- **Mesh Size**: Total vertices = `u_samples * v_samples`
  - 32x32 = 1,024 vertices
  - 40x40 = 1,600 vertices
  - 64x64 = 4,096 vertices

## Examples

See the following examples for complete implementations:

- `examples/parametric_surface_showcase.rs` - Basic sphere
- `examples/parametric_surface_advanced.rs` - Multiple surfaces (torus, wavy, custom)

## Mathematical Background

A parametric surface is defined by:
```
S(u, v) = (x(u, v), y(u, v), z(u, v))
```

Where:
- `u` and `v` are parameters in specified ranges
- The function maps 2D parameter space to 3D world space
- The surface is tessellated into triangles for rendering

## Limitations

- Currently supports solid colors only (no per-vertex coloring)
- No built-in lighting/shading (uses flat colors)
- No animation support yet (can be added via `write_progress` pattern)
- Mesh generation is CPU-based (not GPU-accelerated)

## Future Enhancements

- Per-vertex coloring based on surface properties
- Gradient coloring based on height/curvature
- Animation support (progressive reveal)
- Normal calculation for lighting
- Texture mapping support
