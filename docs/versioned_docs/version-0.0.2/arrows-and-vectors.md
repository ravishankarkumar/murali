---
sidebar_position: 9
---

# Arrows & vectors

The `Arrow` primitive draws a directed line with a triangular tip. It's useful for vector fields, diagrams, and annotations.

## Basic usage

```rust
use murali::frontend::collection::primitives::arrow::Arrow;
use glam::Vec2;

// Arrow::with_default_tip(start, end, thickness, color)
scene.add_tattva(
    Arrow::with_default_tip(
        Vec2::new(-2.0, 0.0),
        Vec2::new(2.0, 0.0),
        0.05,
        Vec4::new(1.0, 0.5, 0.0, 1.0),
    ),
    Vec3::ZERO,
);
```

`start` and `end` are 2D positions in world space. The arrow is placed at the `Vec3` position passed to `add_tattva`.

## Custom tip proportions

```rust
// Arrow::new(start, end, thickness, tip_length, tip_width, color)
Arrow::new(
    Vec2::new(-1.0, 0.0),
    Vec2::new(1.0, 0.0),
    0.04,
    0.4,  // tip length (fraction of total length)
    0.2,  // tip width
    Vec4::new(0.5, 0.8, 1.0, 1.0),
)
```

## Vector field pattern

Arrows are cheap — you can add many of them to build a vector field:

```rust
for i in 0..8 {
    let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
    scene.add_tattva(
        Arrow::with_default_tip(
            Vec2::ZERO,
            Vec2::new(angle.cos() * 0.8, angle.sin() * 0.8),
            0.03,
            Vec4::new(0.6, 0.8, 1.0, 0.9),
        ),
        Vec3::new(0.0, 0.0, 0.0),
    );
}
```

## Animating arrows with updaters

Arrow direction and scale can be driven by updaters at runtime. See the [Updaters](updaters) page for the full pattern.
