# Earth to Torus Morphing Guide

## Overview

This guide demonstrates how to create fascinating visualizations showing Earth transforming into a toroidal (donut-shaped) planet. This showcases the power of parametric surfaces for topology transformation and mathematical visualization.

## Concept

The examples show two different topologies of Earth:
1. **Spherical Earth** - Traditional sphere topology
2. **Toroidal Earth** - Donut-shaped topology

Both use Earth-like coloring based on latitude/height to make the transformation visually intuitive.

## Example 1: Basic Earth to Torus

**File**: `examples/earth_to_torus_morph.rs`

Shows a rotating sphere with Earth-like coloring. The sphere rotates on both X and Y axes to show the 3D structure.

```rust
let earth = ParametricSurface::new(
    (0.0, PI),
    (0.0, 2.0 * PI),
    |u, v| {
        let sin_u = u.sin();
        Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
    },
)
.with_samples(40, 40)
.with_render_mode(SurfaceRenderMode::SolidWithWireframe)
.with_color_fn(|z| {
    // Earth-like coloring based on latitude
    let normalized = (z + 1.0) / 2.0;
    if normalized < 0.3 {
        Vec4::new(0.9, 0.9, 0.9, 1.0)  // Polar ice
    } else if normalized < 0.5 {
        Vec4::new(0.3, 0.7, 0.95, 1.0)  // Arctic
    } else if normalized < 0.7 {
        Vec4::new(0.1, 0.5, 0.8, 1.0)   // Ocean
    } else {
        Vec4::new(0.2, 0.6, 0.3, 1.0)   // Land
    }
});
```

## Example 2: Side-by-Side Comparison

**File**: `examples/earth_torus_advanced.rs`

Shows both Earth (sphere) and Toroidal Earth (torus) side-by-side, both rotating with the same Earth-like coloring scheme.

```rust
// Sphere Earth
let earth = ParametricSurface::new(
    (0.0, PI),
    (0.0, 2.0 * PI),
    |u, v| {
        let sin_u = u.sin();
        Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
    },
);

// Toroidal Earth
let torus = ParametricSurface::new(
    (0.0, 2.0 * PI),
    (0.0, 2.0 * PI),
    |u, v| {
        let r = 0.5;  // Minor radius
        let R = 1.2;  // Major radius
        let x = (R + r * v.cos()) * u.cos();
        let y = (R + r * v.cos()) * u.sin();
        let z = r * v.sin();
        Vec3::new(x, y, z)
    },
);
```

## Earth-Like Coloring

The coloring scheme maps height/latitude to Earth-like colors:

```rust
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    if normalized < 0.2 {
        Vec4::new(0.95, 0.95, 0.95, 1.0)  // Polar ice caps
    } else if normalized < 0.35 {
        Vec4::new(0.3, 0.7, 0.95, 1.0)    // Arctic regions
    } else if normalized < 0.65 {
        Vec4::new(0.1, 0.5, 0.8, 1.0)     // Oceans
    } else if normalized < 0.8 {
        Vec4::new(0.3, 0.7, 0.95, 1.0)    // Tropical regions
    } else {
        Vec4::new(0.95, 0.95, 0.95, 1.0)  // Antarctic
    }
})
```

## Parametric Equations

### Sphere (Earth)
```
x = sin(u) * cos(v)
y = sin(u) * sin(v)
z = cos(u)

u ∈ [0, π]    (latitude)
v ∈ [0, 2π]   (longitude)
```

### Torus (Toroidal Earth)
```
x = (R + r*cos(v)) * cos(u)
y = (R + r*cos(v)) * sin(u)
z = r * sin(v)

u ∈ [0, 2π]   (around major circle)
v ∈ [0, 2π]   (around minor circle)
R = 1.2       (major radius)
r = 0.5       (minor radius)
```

## Rendering Modes

Both examples use `SurfaceRenderMode::SolidWithWireframe` to show:
- **Solid mesh** - The surface itself
- **Wireframe** - The parametric grid structure

This combination makes the topology transformation very clear.

## Animation

Both surfaces rotate continuously:

```rust
// Rotate around Y axis (longitude)
timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(8.0)
    .ease(Ease::Linear)
    .rotate_to(glam::Quat::from_axis_angle(Vec3::Y, 2.0 * PI))
    .spawn();

// Rotate around X axis (tilt)
timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(8.0)
    .ease(Ease::Linear)
    .rotate_to(glam::Quat::from_axis_angle(Vec3::X, PI * 0.2))
    .spawn();
```

## Running the Examples

```bash
# Basic Earth to Torus
cargo run --example earth_to_torus_morph

# Side-by-side comparison
cargo run --example earth_torus_advanced
```

## Customization Ideas

### 1. Different Coloring Schemes

```rust
// Rainbow gradient
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    let hue = normalized * 6.0;
    // ... rainbow color mapping
})

// Temperature map
.with_color_fn(|z| {
    let normalized = (z + 1.0) / 2.0;
    Vec4::new(normalized, 0.5, 1.0 - normalized, 1.0)
})
```

### 2. Different Torus Parameters

```rust
// Thin torus (like a ring)
let r = 0.2;  // Small minor radius
let R = 1.5;  // Large major radius

// Fat torus (like a donut)
let r = 0.8;  // Large minor radius
let R = 1.0;  // Small major radius
```

### 3. Animated Morphing

To create a true morphing animation between sphere and torus, you would need to:
1. Create a parametric function that blends between the two
2. Use a time parameter to control the blend
3. Animate the blend parameter over time

Example blend function:
```rust
fn morph_surface(u: f32, v: f32, t: f32) -> Vec3 {
    let sphere = sphere_point(u, v);
    let torus = torus_point(u, v);
    sphere.lerp(torus, t)  // Blend between 0 and 1
}
```

### 4. Multiple Surfaces

Create a sequence of intermediate topologies:
- Sphere
- Bulging sphere
- Pinched sphere
- Torus with hole
- Complete torus

## Educational Applications

These examples are perfect for:
- **Topology education** - Visualizing different surface topologies
- **Mathematics visualization** - Parametric surfaces and transformations
- **Planetary science** - Hypothetical planet shapes
- **Computer graphics** - Surface rendering and animation

## Performance Notes

- **Samples**: 36-40 samples per dimension for smooth surfaces
- **Render mode**: `SolidWithWireframe` shows structure clearly
- **Rotation**: Continuous rotation at 8-second period
- **Frame rate**: Smooth 30 FPS animation

## Future Enhancements

1. **True morphing** - Implement smooth transition between topologies
2. **Texture mapping** - Add Earth texture maps
3. **Lighting** - Add realistic lighting and shadows
4. **Multiple topologies** - Show more exotic surface topologies
5. **Interactive control** - Allow user to control morphing progress

## References

- Parametric surfaces: https://en.wikipedia.org/wiki/Parametric_surface
- Torus: https://en.wikipedia.org/wiki/Torus
- Sphere: https://en.wikipedia.org/wiki/Sphere
