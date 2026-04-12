# Updater System Guide

## Overview

The updater system allows you to attach callback functions to tattvas that execute every frame. This is similar to Manim's `add_updater()` functionality and enables dynamic, frame-by-frame updates based on the current state of the scene.

## Use Cases

1. **Dynamic Text**: Update labels to show current position, velocity, or other properties
2. **Velocity Vectors**: Draw vectors that change size/direction based on object motion
3. **Force Fields**: Update field vectors based on moving charged particles
4. **Dependent Objects**: Make one object follow or react to another
5. **Physics Simulations**: Update objects based on calculated physics
6. **Interactive Visualizations**: Respond to changing parameters in real-time

## Basic Usage

### Adding an Updater

```rust
scene.add_updater(tattva_id, |scene, id, dt| {
    // Your update logic here
    // - scene: mutable reference to the entire scene
    // - id: the tattva ID this updater is attached to
    // - dt: delta time since last frame
});
```

### Example: Position Tracker

```rust
let ball_id = scene.add_tattva(ball, Vec3::ZERO);
let label_id = scene.add_tattva(label, Vec3::ZERO);

scene.add_updater(label_id, move |scene, _label_id, _dt| {
    // Get ball's current position
    if let Some(ball) = scene.get_tattva_any(ball_id) {
        let props = DrawableProps::read(ball.props());
        let pos = props.position;
        drop(props);
        
        // Update label position to follow ball
        scene.set_position(label_id, Vec2::new(pos.x, pos.y + 1.0));
    }
});
```

## Advanced Examples

### Velocity Vector

```rust
let initial_velocity = Vec2::new(3.0, 4.0);
let gravity = -5.0;
let start_time = scene.scene_time;

scene.add_updater(vector_id, move |scene, vec_id, _dt| {
    // Get object position
    if let Some(obj) = scene.get_tattva_any(object_id) {
        let props = DrawableProps::read(obj.props());
        let pos = props.position;
        drop(props);
        
        // Calculate current velocity
        let t = scene.scene_time - start_time;
        let vx = initial_velocity.x;
        let vy = initial_velocity.y + gravity * t;
        
        // Update vector scale proportional to velocity magnitude
        if let Some(vec) = scene.get_tattva_any_mut(vec_id) {
            let mut props = DrawableProps::write(vec.props());
            props.scale = vec3(vx * 0.5, vy * 0.5, 1.0);
            props.position = vec3(pos.x, pos.y, 0.0);
        }
    }
});
```

### Force Field Updates

```rust
// Create a grid of force vectors
let mut vector_ids = Vec::new();
for x in -5..=5 {
    for y in -3..=3 {
        let vector = Line::new(Vec3::ZERO, Vec3::Y, 0.05, color);
        let id = scene.add_tattva(vector, vec3(x as f32, y as f32, 0.0));
        vector_ids.push(id);
    }
}

// Add updater to each vector to respond to charged particle
for vec_id in vector_ids {
    scene.add_updater(vec_id, move |scene, vid, _dt| {
        // Get particle position
        if let Some(particle) = scene.get_tattva_any(particle_id) {
            let p_props = DrawableProps::read(particle.props());
            let particle_pos = p_props.position;
            drop(p_props);
            
            // Get vector position
            if let Some(vector) = scene.get_tattva_any(vid) {
                let v_props = DrawableProps::read(vector.props());
                let vector_pos = v_props.position;
                drop(v_props);
                
                // Calculate force direction and magnitude
                let delta = particle_pos - vector_pos;
                let dist_sq = delta.length_squared().max(0.1);
                let force = delta.normalize() / dist_sq;
                
                // Update vector
                if let Some(vec) = scene.get_tattva_any_mut(vid) {
                    let mut props = DrawableProps::write(vec.props());
                    let angle = force.y.atan2(force.x);
                    props.rotation = Quat::from_rotation_z(angle);
                    props.scale = vec3(1.0, force.length() * 2.0, 1.0);
                }
            }
        }
    });
}
```

## Managing Updaters

### Removing Updaters

```rust
// Remove a specific updater by index
let updater_index = scene.add_updater(id, callback);
scene.remove_updater(updater_index);

// Remove all updaters for a tattva
scene.remove_updaters_for(tattva_id);

// Clear all updaters
scene.updaters.clear();
```

### Enabling/Disabling Updaters

```rust
// Disable an updater temporarily
scene.updaters.set_enabled(updater_index, false);

// Re-enable it later
scene.updaters.set_enabled(updater_index, true);
```

## Performance Considerations

1. **Minimize Work**: Updaters run every frame, so keep them efficient
2. **Early Returns**: Check conditions early and return if no update needed
3. **Batch Updates**: Group related updates when possible
4. **Avoid Allocations**: Reuse data structures instead of creating new ones
5. **Limit Updaters**: Don't add updaters to hundreds of objects if avoidable

## Comparison with Manim

| Manim | Murali |
|-------|--------|
| `mob.add_updater(func)` | `scene.add_updater(id, func)` |
| `mob.remove_updater(func)` | `scene.remove_updater(index)` |
| `mob.clear_updaters()` | `scene.remove_updaters_for(id)` |
| Updater receives `(mob, dt)` | Updater receives `(scene, id, dt)` |

## Best Practices

1. **Capture by Move**: Use `move` closures to capture variables by value
2. **Avoid Deadlocks**: Don't hold read locks while trying to get write locks
3. **Use `drop()`**: Explicitly drop read locks before acquiring write locks
4. **Check Existence**: Always check if tattvas exist before accessing them
5. **Document Intent**: Add comments explaining what each updater does

## Examples

See `examples/projectile_with_updaters.rs` for a complete working example demonstrating:
- Position tracking
- Velocity vectors that update based on physics
- Dynamic text labels showing current state
