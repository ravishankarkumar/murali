---
sidebar_position: 3
---

# Animations

Murali uses a `Timeline` to schedule animations. Animations are time-driven — they depend on elapsed time, not frame count.

## Basic setup

```rust
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;

let mut timeline = Timeline::new();

timeline
    .animate(square_id)
    .at(0.0)           // start time in seconds
    .for_duration(2.0) // duration in seconds
    .ease(Ease::InOutQuad)
    .move_to(Vec3::new(3.0, 0.0, 0.0))
    .spawn();

scene.set_timeline("main", timeline);
```

You can have multiple named timelines in a scene.

## Animation types

### move_to

Moves a tattva to a world-space position.

```rust
.move_to(Vec3::new(2.0, 1.0, 0.0))
```

Optionally specify a starting position:

```rust
.move_to(Vec3::new(2.0, 1.0, 0.0))
.from_vec3(Vec3::new(-2.0, 0.0, 0.0))
```

### fade_to

Animates opacity.

```rust
.fade_to(0.0)        // fade out
.fade_to(1.0)        // fade in
```

### scale_to

Scales a tattva.

```rust
.scale_to(Vec3::new(2.0, 2.0, 1.0))
```

### rotate_to

Rotates a tattva.

```rust
use glam::Quat;
.rotate_to(Quat::from_rotation_z(std::f32::consts::PI))
```

### appear

Reveals a tattva by animating its staged opacity from hidden to visible.

```rust
.appear()
```

### morph_from

Morphs one tattva's shape into another.

```rust
// target morphs from source
timeline
    .animate(circle_id)
    .at(0.5)
    .for_duration(2.0)
    .ease(Ease::InOutQuad)
    .morph_from(square_id)
    .spawn();
```

Hide the target initially so it can appear through the morph cleanly:

```rust
scene.hide_tattva(circle_id);
```

### morph_matching_staged

Morphs one group of tattvas into another while automatically staging the target group.

```rust
timeline.morph_matching_staged(
    source_ids,
    target_ids,
    &mut scene,
    1.0,
    3.0,
    Ease::InOutCubic,
);
```

Use raw `morph_matching(...)` only when you need manual control over target visibility/state.

### match_transform

Snaps a tattva's transform to match another tattva's.

```rust
.match_transform(source_id)
```

## Easing functions

| Variant | Description |
|---|---|
| `Ease::Linear` | Constant speed |
| `Ease::InQuad` | Accelerate in |
| `Ease::OutQuad` | Decelerate out |
| `Ease::InOutQuad` | Smooth in and out |
| `Ease::InCubic` | Stronger accelerate in |
| `Ease::OutCubic` | Stronger decelerate out |
| `Ease::InOutCubic` | Stronger smooth in and out |
| `Ease::InOutSmooth` | Smoothstep (C1 continuous) |

## Callbacks

Run arbitrary code at a point in time:

```rust
timeline.call_at(2.0, |scene| {
    // runs once at t=2.0
});
```

Run a callback over a duration (receives normalized `t` from 0.0 to 1.0):

```rust
timeline.call_during(1.0, 2.0, |scene, t| {
    // runs every frame between t=1.0 and t=3.0
});
```

## Camera animations

```rust
use murali::frontend::animation::camera_animation_builder::CameraAnimationBuilder;

timeline
    .animate_camera()
    .at(0.0)
    .for_duration(2.0)
    .move_to(Vec3::new(0.0, 0.0, 5.0))
    .spawn();
```
