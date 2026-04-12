# Animated Wireframe Surfaces Guide

## Overview

ParametricSurface now supports **animated wireframe drawing** where grid lines are progressively revealed in real-time. This creates stunning visualizations of 3D surfaces being "drawn" in space.

## Animation Phases

The wireframe animation has two distinct phases:

### Phase 1: Horizontal Lines (0.0 - 0.5 progress)
- Draws lines in the u-direction (horizontal grid lines)
- Creates the "skeleton" of the surface
- Reveals the surface structure progressively

### Phase 2: Vertical Lines (0.5 - 1.0 progress)
- Draws lines in the v-direction (vertical grid lines)
- Completes the grid structure
- Creates a beautiful "weaving" effect

## Basic Usage

```rust
use murali::frontend::collection::graph::parametric_surface::{
    ParametricSurface, SurfaceRenderMode
};

let surface = ParametricSurface::new(u_range, v_range, f)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_write_progress(0.0);  // Start invisible

let surface_id = scene.add_tattva(surface, Vec3::ZERO);

// Animate the wireframe drawing
timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(4.0)
    .ease(Ease::InOutQuad)
    .write_surface()
    .spawn();
```

## With Heat Map Coloring

Combine animation with color functions for maximum visual impact:

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_write_progress(0.0)
    .with_color_fn(|z| {
        // Heat map: blue (cold) to red (hot)
        let normalized = (z + 1.0) / 2.0;
        if normalized < 0.5 {
            let t = normalized * 2.0;
            Vec4::new(0.0, t, 1.0, 1.0)  // Blue to cyan
        } else {
            let t = (normalized - 0.5) * 2.0;
            Vec4::new(t, 1.0 - t, 0.0, 1.0)  // Cyan to red
        }
    });
```

## Staggered Animation

Create impressive sequences by staggering multiple surfaces:

```rust
// Surface 1: draw from 0.0 to 2.0s
timeline
    .animate(surface1_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::InOutQuad)
    .write_surface()
    .spawn();

// Surface 2: draw from 1.0 to 3.0s (overlapping)
timeline
    .animate(surface2_id)
    .at(1.0)
    .for_duration(2.0)
    .ease(Ease::InOutQuad)
    .write_surface()
    .spawn();

// Surface 3: draw from 2.0 to 4.0s (overlapping)
timeline
    .animate(surface3_id)
    .at(2.0)
    .for_duration(2.0)
    .ease(Ease::InOutQuad)
    .write_surface()
    .spawn();
```

## Combined with Rotation

Rotate surfaces while drawing for dynamic visualization:

```rust
// Draw the wireframe
timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(4.0)
    .ease(Ease::InOutQuad)
    .write_surface()
    .spawn();

// Rotate simultaneously
timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(4.0)
    .ease(Ease::Linear)
    .rotate_to(glam::Quat::from_axis_angle(Vec3::Y, PI * 0.5))
    .spawn();
```

## Easing Functions

Different easing functions create different effects:

- `Ease::Linear` - Constant speed drawing
- `Ease::InOutQuad` - Smooth acceleration/deceleration
- `Ease::InCubic` - Slow start, fast end
- `Ease::OutCubic` - Fast start, slow end
- `Ease::InOutCubic` - Smooth throughout

## Complete Example

```rust
use glam::{Vec3, Vec4};
use murali::frontend::collection::graph::parametric_surface::{
    ParametricSurface, SurfaceRenderMode
};
use murali::frontend::animation::Ease;
use std::f32::consts::PI;

// Create animated wireframe surface
let surface = ParametricSurface::new(
    (-PI, PI),
    (-PI, PI),
    |u, v| Vec3::new(u, v, u.sin() * v.cos()),
)
.with_samples(28, 28)
.with_render_mode(SurfaceRenderMode::Wireframe)
.with_write_progress(0.0)
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    if normalized < 0.5 {
        let t = normalized * 2.0;
        Vec4::new(0.0, t, 1.0, 1.0)
    } else {
        let t = (normalized - 0.5) * 2.0;
        Vec4::new(t, 1.0 - t, 0.0, 1.0)
    }
});

let surface_id = scene.add_tattva(surface, Vec3::ZERO);

// Animate
let mut timeline = Timeline::new();

timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(4.0)
    .ease(Ease::InOutQuad)
    .write_surface()
    .spawn();

timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(4.0)
    .ease(Ease::Linear)
    .rotate_to(glam::Quat::from_axis_angle(Vec3::Z, PI * 0.5))
    .spawn();

scene.timelines.insert("main".to_string(), timeline);
```

## Performance Tips

- **Wireframe is efficient** - Uses line primitives instead of mesh triangles
- **Fewer samples = faster** - Use 20-28 samples for smooth animation
- **Color functions are fast** - Simple math operations per line
- **Stagger animations** - Overlapping animations create visual interest without performance hit

## Use Cases

### Educational Visualization
- Show how parametric surfaces are constructed
- Demonstrate mathematical concepts in 3D
- Visualize surface topology

### Data Visualization
- Animate heat maps showing data changes
- Reveal complex surfaces progressively
- Create engaging presentations

### Scientific Visualization
- Visualize physics simulations
- Show computational results
- Demonstrate mathematical transformations

### Artistic Visualization
- Create beautiful mathematical art
- Combine with music for synchronized animations
- Generate engaging visual content

## Examples

Run the examples to see animated wireframes in action:

```bash
# Single animated surface
cargo run --example parametric_surface_wireframe_animated

# Multiple surfaces with staggered animation
cargo run --example parametric_surface_wireframe_showcase
```

## Advanced Techniques

### Morphing Between Surfaces

```rust
// Start with one surface, animate to another
// (Requires implementing surface morphing)
```

### Progressive Color Mapping

```rust
// Change color function during animation
// (Requires dynamic property updates)
```

### Multi-Surface Choreography

Create complex animations by coordinating multiple surfaces with different timings, rotations, and color schemes.

## Troubleshooting

### Animation not visible
- Ensure `write_progress` starts at 0.0
- Check that `write_surface()` animation is added to timeline
- Verify render mode is `Wireframe` or `SolidWithWireframe`

### Lines not colored
- Ensure `color_fn` is set with `.with_color_fn()`
- Check that color function returns valid `Vec4` values
- Verify z-coordinate range matches your surface

### Performance issues
- Reduce sample count (try 16-20 instead of 32)
- Use `Wireframe` mode instead of `SolidWithWireframe`
- Simplify color function calculations
