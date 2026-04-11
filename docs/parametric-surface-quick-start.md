# ParametricSurface Quick Start

## 5-Minute Setup

### 1. Import

```rust
use murali::frontend::collection::graph::parametric_surface::ParametricSurface;
use glam::Vec3;
```

### 2. Create a Surface

```rust
let sphere = ParametricSurface::new(
    (0.0, std::f32::consts::PI),      // u_range
    (0.0, 2.0 * std::f32::consts::PI), // v_range
    |u, v| {
        let sin_u = u.sin();
        Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
    }
);
```

### 3. Add to Scene

```rust
scene.add_tattva(sphere, Vec3::ZERO);
```

### 4. Customize (Optional)

```rust
let surface = ParametricSurface::new(u_range, v_range, f)
    .with_samples(40, 40)                              // Mesh density
    .with_color(Vec4::new(0.44, 0.84, 0.71, 1.0));   // Color
```

## Common Surfaces

### Sphere (1 line)
```rust
|u, v| Vec3::new(u.sin() * v.cos(), u.sin() * v.sin(), u.cos())
```

### Torus (4 lines)
```rust
|u, v| {
    let r = 0.4; let R = 1.2;
    Vec3::new((R + r * v.cos()) * u.cos(), (R + r * v.cos()) * u.sin(), r * v.sin())
}
```

### Wavy (1 line)
```rust
|u, v| Vec3::new(u, v, u.sin() * v.cos())
```

### Saddle (1 line)
```rust
|u, v| Vec3::new(u, v, u * u - v * v)
```

## With Axes

```rust
use murali::frontend::collection::composite::axes3d::Axes3D;

scene.add_tattva(
    Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0))
        .with_step(1.0),
    Vec3::ZERO,
);

scene.add_tattva(surface, Vec3::ZERO);
```

## With Animation

```rust
let surface_id = scene.add_tattva(surface, Vec3::ZERO);

let mut timeline = Timeline::new();
timeline
    .animate(surface_id)
    .at(0.0)
    .for_duration(8.0)
    .rotate_to(Quat::from_axis_angle(Vec3::Z, 2.0 * PI))
    .spawn();

scene.timelines.insert("main".to_string(), timeline);
```

## Tips

- **Smooth surfaces**: Use 40-48 samples per dimension
- **Fast rendering**: Use 16-24 samples per dimension
- **Parameter ranges**: Match mathematical conventions (e.g., 0 to 2π for angles)
- **Colors**: Use `Vec4::new(r, g, b, a)` with values 0.0-1.0

## Examples

Run the examples to see it in action:

```bash
cargo run --example parametric_surface_showcase
cargo run --example parametric_surface_advanced
cargo run --example parametric_surface_animated
```

## Next Steps

- See `docs/parametric-surface-guide.md` for detailed documentation
- Check `examples/parametric_surface_*.rs` for complete examples
- Explore mathematical surfaces at [Wolfram MathWorld](https://mathworld.wolfram.com/)
