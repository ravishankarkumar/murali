---
sidebar_position: 1
---

# Common Tattva Properties

Most tattvas share the same scene-level properties even when their shape-specific APIs differ.

These common properties come from `DrawableProps` and are what timelines and scene helpers usually animate:

- `position` - world-space translation
- `rotation` - 3D orientation via `Quat`
- `scale` - per-axis scale
- `visible` - whether the tattva should render
- `opacity` - alpha multiplier from `0.0` to `1.0`
- `tag` - optional identifier for higher-level scene logic

## Preferred authoring style

For everyday scene code, prefer the intent helpers on `Scene` instead of reaching into tattva internals:

```rust
scene.set_position_2d(id, glam::Vec2::new(2.0, 1.0));
scene.set_position_3d(id, glam::Vec3::new(2.0, 1.0, 0.5));
scene.set_scale(id, glam::Vec3::splat(1.25));
scene.set_rotation(id, glam::Quat::from_rotation_z(0.3));
scene.set_opacity(id, 0.6);
scene.hide(id);
scene.show(id);
```

Use typed tattva access only when you need to modify a shape-specific field that is not part of the shared transform/visibility surface.

## What belongs here vs on tattva pages

Keep this page focused on cross-cutting behavior:

- transforms
- visibility
- opacity
- scene lookup and mutation patterns
- layout helpers that work across many tattvas

Keep family pages focused on the tattvas themselves:

- what each tattva is for
- its constructor
- its important custom fields
- best animation pairings
- gotchas

## Recommended docs pattern

For each tattva page, link back here instead of repeating the full shared property story.

A good split is:

- `Common Tattva Properties` for shared behavior
- one page per tattva family for orientation
- eventually one reference block per concrete tattva for constructors and notable fields
