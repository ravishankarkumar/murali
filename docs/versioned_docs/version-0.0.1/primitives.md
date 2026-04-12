---
sidebar_position: 2
---

# Primitives

v0.0.1 ships with a small set of basic shapes. All are added to the scene with `add_tattva(shape, position)`.

## Circle

```rust
use murali::frontend::collection::primitives::circle::Circle;

scene.add_tattva(
    Circle::new(
        1.0,                              // radius in world units
        64,                               // number of segments
        Vec4::new(0.2, 0.6, 1.0, 1.0),   // RGBA color
    ),
    Vec3::new(0.0, 0.0, 0.0),
);
```

More segments = smoother circle. 32–64 is fine for most sizes.

## Square

```rust
use murali::frontend::collection::primitives::square::Square;

scene.add_tattva(
    Square::new(1.5, Vec4::new(0.96, 0.42, 0.28, 1.0)),
    Vec3::new(-3.0, 0.0, 0.0),
);
```

## Rectangle

```rust
use murali::frontend::collection::primitives::rectangle::Rectangle;

scene.add_tattva(
    Rectangle::new(3.0, 1.5, Vec4::new(0.22, 0.50, 0.96, 1.0)),
    Vec3::new(0.0, 1.0, 0.0),
);
```

## Line

```rust
use murali::frontend::collection::primitives::line::Line;

scene.add_tattva(
    Line::new(
        Vec3::new(-4.0, -2.0, 0.0),  // start
        Vec3::new(4.0, -2.0, 0.0),   // end
        0.06,                         // thickness
        Vec4::new(1.0, 1.0, 1.0, 1.0),
    ),
    Vec3::ZERO,
);
```

## Colors

Colors are `Vec4` in linear RGBA, values `0.0` to `1.0`:

```rust
Vec4::new(r, g, b, a)

Vec4::new(1.0, 1.0, 1.0, 1.0)  // white, fully opaque
Vec4::new(0.0, 0.0, 0.0, 1.0)  // black
Vec4::new(1.0, 0.0, 0.0, 0.5)  // semi-transparent red
```

## Placing multiple shapes

```rust
// Left
scene.add_tattva(Square::new(1.5, red), Vec3::new(-3.0, 0.0, 0.0));

// Center
scene.add_tattva(Circle::new(1.0, 48, green), Vec3::new(0.0, 0.0, 0.0));

// Right
scene.add_tattva(Rectangle::new(2.0, 1.0, blue), Vec3::new(3.0, 0.0, 0.0));
```

All coordinates are in world units. At the default camera distance (`z = 10.0`), the visible range is roughly x: `-7` to `7`, y: `-4` to `4`.
