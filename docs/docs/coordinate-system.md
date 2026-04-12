---
sidebar_position: 5
---

# Coordinate system

Murali uses a right-handed world-space coordinate system with the origin at the center of the screen.

## Axes

- X — horizontal, positive to the right
- Y — vertical, positive upward
- Z — depth, positive toward the viewer

## Aspect ratio

The default viewport is 16:9. At the default camera distance (`z = 10.0`), the visible world space is roughly:

- X: `-7.1` to `7.1`
- Y: `-4.0` to `4.0`

All sizes (font sizes, shape radii, line thickness) are in these world units.

## Camera

The camera defaults to an orbit controller centered at the origin. For 2D scenes, set the camera position on the Z axis:

```rust
scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);
```

For 3D scenes, you can orbit freely in the preview window.

## Positioning tattvas

Positions are passed as `Vec3` to `add_tattva`:

```rust
// Center of screen
scene.add_tattva(shape, Vec3::new(0.0, 0.0, 0.0));

// Upper left area
scene.add_tattva(shape, Vec3::new(-4.0, 2.5, 0.0));
```

## Colors

Colors are `Vec4` in linear RGBA, values from `0.0` to `1.0`:

```rust
Vec4::new(r, g, b, a)

// White
Vec4::new(1.0, 1.0, 1.0, 1.0)

// Semi-transparent red
Vec4::new(1.0, 0.0, 0.0, 0.5)
```
