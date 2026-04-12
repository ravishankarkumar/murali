# Parametric Surface Wireframe & Heat Map Guide

## Overview

ParametricSurface now supports three rendering modes:
1. **Solid** - Filled mesh (default)
2. **Wireframe** - Grid lines only
3. **SolidWithWireframe** - Both filled mesh and grid lines

Additionally, you can apply color functions to map surface properties (like height/temperature) to colors for heat map visualization.

## Render Modes

### Solid (Default)

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_render_mode(SurfaceRenderMode::Solid);
```

Renders a filled mesh with solid color.

### Wireframe

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_render_mode(SurfaceRenderMode::Wireframe);
```

Renders only the grid lines, showing the surface structure clearly.

### Solid with Wireframe

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_render_mode(SurfaceRenderMode::SolidWithWireframe);
```

Renders both the filled mesh and grid lines for maximum visual clarity.

## Color Functions (Heat Maps)

Apply a color function to map surface properties to colors:

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_color_fn(|z| {
        // Map height (z) to color
        let normalized = (z + 1.0) / 2.0; // Normalize to [0, 1]
        Vec4::new(normalized, 0.5, 1.0 - normalized, 1.0)
    });
```

The color function receives the **z-coordinate (height)** of each point and returns a `Vec4` color.

## Common Heat Map Patterns

### Blue to Red (Cold to Hot)

```rust
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    if normalized < 0.5 {
        let t = normalized * 2.0;
        Vec4::new(0.0, t, 1.0, 1.0)  // Blue to cyan
    } else {
        let t = (normalized - 0.5) * 2.0;
        Vec4::new(t, 1.0 - t, 0.0, 1.0)  // Cyan to red
    }
})
```

### Grayscale

```rust
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    Vec4::new(normalized, normalized, normalized, 1.0)
})
```

### Purple to Yellow

```rust
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    Vec4::new(
        0.5 + 0.5 * normalized,
        0.2 + 0.8 * normalized,
        0.8 - 0.8 * normalized,
        1.0,
    )
})
```

### Rainbow Gradient

```rust
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    let hue = normalized * 6.0;
    match hue as i32 {
        0 => Vec4::new(1.0, normalized * 6.0, 0.0, 1.0),
        1 => Vec4::new(1.0 - (normalized - 1.0/6.0) * 6.0, 1.0, 0.0, 1.0),
        2 => Vec4::new(0.0, 1.0, (normalized - 2.0/6.0) * 6.0, 1.0),
        3 => Vec4::new(0.0, 1.0 - (normalized - 3.0/6.0) * 6.0, 1.0, 1.0),
        4 => Vec4::new((normalized - 4.0/6.0) * 6.0, 0.0, 1.0, 1.0),
        _ => Vec4::new(1.0, 0.0, 1.0 - (normalized - 5.0/6.0) * 6.0, 1.0),
    }
})
```

## Complete Example

```rust
use glam::{Vec3, Vec4};
use murali::frontend::collection::graph::parametric_surface::{
    ParametricSurface, SurfaceRenderMode
};

let wavy = ParametricSurface::new(
    (-PI, PI),
    (-PI, PI),
    |u, v| {
        Vec3::new(u, v, u.sin() * v.cos())
    },
)
.with_samples(32, 32)
.with_render_mode(SurfaceRenderMode::Wireframe)
.with_color_fn(|z| {
    // Heat map: blue (cold) to red (hot)
    let normalized = (z + 1.0) / 2.0;
    if normalized < 0.5 {
        let t = normalized * 2.0;
        Vec4::new(0.0, t, 1.0, 1.0)
    } else {
        let t = (normalized - 0.5) * 2.0;
        Vec4::new(t, 1.0 - t, 0.0, 1.0)
    }
});

scene.add_tattva(wavy, Vec3::ZERO);
```

## Use Cases

### Temperature Visualization
Show temperature distribution on a surface using heat map colors.

### Elevation Maps
Visualize terrain height using color gradients.

### Physics Simulations
Display scalar fields (pressure, density, etc.) on parametric surfaces.

### Data Visualization
Map any scalar value to surface color for intuitive data representation.

## Performance Tips

- **Wireframe mode** is faster than solid mode (fewer vertices to render)
- **SolidWithWireframe** combines both but may be slower
- Use fewer samples (16-24) for wireframe if performance is an issue
- Color functions are evaluated per-line, so keep them simple

## Examples

Run the wireframe example to see all three modes in action:

```bash
cargo run --example parametric_surface_wireframe
```

This example shows:
- Wavy surface with heat map wireframe
- Sphere with solid + wireframe + gradient coloring
- Torus with rainbow wireframe gradient
