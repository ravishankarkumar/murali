# Vector Fields and StreamLines Guide

## Overview

Murali now has full support for vector field visualization, matching Manim's capabilities. This includes:
- **VectorField**: Displays arrows at grid points showing vector direction and magnitude
- **StreamLines**: Shows flow paths that particles would follow in the field
- **Force Fields**: Dynamic fields that update based on moving charges (via updaters)

## VectorField

Vector fields display arrows at grid points, where each arrow represents the vector value at that location.

### Basic Usage

```rust
use murali::frontend::collection::graph::vector_field::VectorField;

// Create a radial vector field
let field = VectorField::new(
    (-3.0, 3.0),  // x range
    (-2.0, 2.0),  // y range
    10,           // x steps
    8,            // y steps
    |pos: Vec2| pos,  // Field function: radial outward
)
.with_color(Vec4::new(0.5, 0.7, 1.0, 0.8))
.with_length_scale(0.5);

scene.add_tattva(field, Vec3::ZERO);
```

### Configuration Options

```rust
VectorField::new(x_range, y_range, x_steps, y_steps, field_fn)
    .with_color(color)                    // Base color for all vectors
    .with_color_fn(|magnitude| color)     // Color based on magnitude
    .with_length_scale(scale)             // Scale factor for vector lengths
    .with_length_limits(min, max)         // Min/max vector lengths
    .with_arrow_style(thickness, tip_length, tip_width)  // Arrow appearance
```

### Common Vector Fields

#### Radial Field (Source/Sink)
```rust
|pos: Vec2| pos  // Points away from origin
|pos: Vec2| -pos // Points toward origin
```

#### Rotational Field (Vortex)
```rust
|pos: Vec2| Vec2::new(-pos.y, pos.x)  // Counter-clockwise rotation
```

#### Gradient Field
```rust
|pos: Vec2| Vec2::new(1.0, pos.y)  // Gradient in y direction
```

#### Saddle Point
```rust
|pos: Vec2| Vec2::new(pos.x, -pos.y)  // Saddle at origin
```

#### Electric Field (Point Charge)
```rust
|pos: Vec2| {
    let charge_pos = Vec2::new(1.0, 0.0);
    let delta = pos - charge_pos;
    let dist = delta.length().max(0.1);
    delta.normalize() / (dist * dist)  // Inverse square law
}
```

### Magnitude-Based Coloring

```rust
let field = VectorField::new(...)
    .with_color_fn(|magnitude: f32| {
        // Blue for low magnitude, red for high
        let t = (magnitude * 0.5).min(1.0);
        Vec4::new(t, 0.3, 1.0 - t, 0.9)
    });
```

## StreamLines

Streamlines show the paths that particles would follow if placed in the vector field. They are curves that are tangent to the field at every point.

### Basic Usage

```rust
use murali::frontend::collection::graph::stream_lines::StreamLines;

// Create streamlines from starting points
let start_points = vec![
    Vec2::new(-2.0, 0.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(0.0, 0.0),
];

let streams = StreamLines::new(
    start_points,
    |pos: Vec2| Vec2::new(-pos.y, pos.x),  // Vortex field
)
.with_color(Vec4::new(1.0, 0.5, 0.5, 0.8))
.with_thickness(0.03)
.with_step_size(0.05)
.with_max_steps(200);

scene.add_tattva(streams, Vec3::ZERO);
```

### Creating Starting Points

#### From a Grid
```rust
let streams = StreamLines::from_grid(
    (-3.0, 3.0),  // x range
    (-2.0, 2.0),  // y range
    5,            // x count
    4,            // y count
    field_fn,
);
```

#### Along a Line
```rust
use murali::frontend::collection::graph::stream_lines::line_start_points;

let starts = line_start_points(
    Vec2::new(-2.0, 1.0),  // start
    Vec2::new(2.0, 1.0),   // end
    8,                      // count
);
```

#### Around a Circle
```rust
use murali::frontend::collection::graph::stream_lines::circle_start_points;

let starts = circle_start_points(
    Vec2::ZERO,  // center
    1.5,         // radius
    12,          // count
);
```

### Configuration Options

```rust
StreamLines::new(start_points, field_fn)
    .with_color(color)                           // Line color
    .with_color_fn(|pos, magnitude| color)       // Color based on position/magnitude
    .with_thickness(thickness)                   // Line thickness
    .with_step_size(step_size)                   // Integration step (smaller = more accurate)
    .with_max_steps(max_steps)                   // Maximum points per streamline
    .with_bounds(min, max)                       // Constrain streamlines to region
```

### Integration Method

StreamLines use Euler integration to trace paths:
1. Start at initial point
2. Evaluate vector field at current position
3. Move in that direction by step_size
4. Repeat until max_steps or out of bounds

For more accuracy, use smaller step_size (but more computation).

## Force Fields with Updaters

Combine vector fields with the updater system to create dynamic force fields that respond to moving charges.

### Example: Moving Charge

```rust
// Create grid of force vectors
let mut vector_ids = Vec::new();
for x in -5..=5 {
    for y in -3..=3 {
        let arrow = Arrow::with_default_tip(...);
        let id = scene.add_tattva(arrow, vec3(x as f32, y as f32, 0.0));
        vector_ids.push((id, vec3(x as f32, y as f32, 0.0)));
    }
}

// Add updater to each vector
for (vector_id, vector_pos) in vector_ids {
    scene.add_updater(vector_id, move |scene, vid, _dt| {
        // Get charge position
        let charge_pos = get_charge_position(scene, charge_id);
        
        // Calculate force at this vector's position
        let delta = vector_pos - charge_pos;
        let dist = delta.length().max(0.3);
        let force = delta.normalize() / (dist * dist);
        
        // Update vector rotation and scale
        update_vector(scene, vid, force);
    });
}
```

See `examples/force_field_with_updaters.rs` for complete implementation.

## Comparison: Vector Fields vs StreamLines

| Aspect | Vector Field | StreamLines |
|--------|-------------|-------------|
| **Shows** | Direction at discrete points | Continuous flow paths |
| **Best for** | Understanding local behavior | Understanding global flow |
| **Density** | Fixed grid | Depends on starting points |
| **Computation** | O(grid_points) | O(start_points × max_steps) |
| **Use case** | Quick overview | Detailed flow analysis |

### When to Use Each

**Use Vector Fields when:**
- You want to see the field at many points quickly
- Local direction is more important than paths
- You need uniform coverage of the domain
- Showing field magnitude with color/length

**Use StreamLines when:**
- You want to visualize flow paths
- Understanding trajectories is important
- Showing how particles would move
- Creating visually appealing flow patterns

**Use Both when:**
- Teaching vector fields (show both representations)
- Need complete understanding of the field
- Creating publication-quality visualizations

## Examples

### 1. Electric Field Visualization

```rust
// Positive charge at origin
let field_fn = |pos: Vec2| {
    let dist = pos.length().max(0.1);
    pos.normalize() / (dist * dist)
};

// Vector field
let vectors = VectorField::new((-3.0, 3.0), (-3.0, 3.0), 12, 12, field_fn)
    .with_color_fn(|mag| {
        let t = (mag * 2.0).min(1.0);
        Vec4::new(t, 0.3, 1.0 - t, 0.8)
    });

// Streamlines from circle around charge
let starts = circle_start_points(Vec2::ZERO, 0.5, 12);
let streams = StreamLines::new(starts, field_fn)
    .with_color(Vec4::new(1.0, 0.5, 0.5, 0.8));
```

### 2. Fluid Flow Around Obstacle

```rust
let field_fn = |pos: Vec2| {
    let obstacle = Vec2::ZERO;
    let to_obstacle = pos - obstacle;
    let dist = to_obstacle.length();
    
    if dist < 0.5 {
        Vec2::ZERO  // Inside obstacle
    } else {
        // Uniform flow + repulsion
        let uniform = Vec2::new(1.0, 0.0);
        let repulsion = to_obstacle.normalize() * (0.2 / (dist * dist));
        uniform + repulsion
    }
};

let starts = line_start_points(Vec2::new(-3.0, -2.0), Vec2::new(-3.0, 2.0), 8);
let streams = StreamLines::new(starts, field_fn)
    .with_bounds(Vec2::new(-3.0, -2.0), Vec2::new(3.0, 2.0));
```

### 3. Magnetic Field (Dipole)

```rust
let field_fn = |pos: Vec2| {
    let north = Vec2::new(0.0, 0.5);
    let south = Vec2::new(0.0, -0.5);
    
    let to_north = pos - north;
    let to_south = pos - south;
    
    let dist_n = to_north.length().max(0.1);
    let dist_s = to_south.length().max(0.1);
    
    // North pole repels, south pole attracts
    let force_n = to_north.normalize() / (dist_n * dist_n);
    let force_s = -to_south.normalize() / (dist_s * dist_s);
    
    force_n + force_s
};
```

## Performance Tips

1. **Vector Fields**: Reduce grid resolution for faster rendering
2. **StreamLines**: Limit max_steps and use larger step_size for faster computation
3. **Dynamic Fields**: Update only visible vectors, skip distant ones
4. **Bounds**: Always set bounds to prevent infinite streamlines
5. **Color Functions**: Keep color calculations simple (avoid expensive operations)

## Advanced Techniques

### Animated Streamlines

Use updaters to animate particles along streamlines:

```rust
scene.add_updater(particle_id, move |scene, pid, _dt| {
    let pos = get_position(scene, pid);
    let velocity = field_fn(pos);
    let new_pos = pos + velocity * dt * speed;
    scene.set_position(pid, new_pos);
});
```

### Time-Varying Fields

Create fields that change over time:

```rust
let start_time = scene.scene_time;
let field_fn = move |pos: Vec2| {
    let t = scene.scene_time - start_time;
    let rotation = t * 0.5;
    // Rotate the field over time
    Vec2::new(
        pos.x * rotation.cos() - pos.y * rotation.sin(),
        pos.x * rotation.sin() + pos.y * rotation.cos(),
    )
};
```

### Multiple Charges

Superpose fields from multiple sources:

```rust
let charges = vec![
    (Vec2::new(-1.0, 0.0), 1.0),   // (position, charge)
    (Vec2::new(1.0, 0.0), -1.0),
];

let field_fn = move |pos: Vec2| {
    let mut total = Vec2::ZERO;
    for (charge_pos, charge) in &charges {
        let delta = pos - charge_pos;
        let dist = delta.length().max(0.1);
        let force = delta.normalize() * charge / (dist * dist);
        total += force;
    }
    total
};
```

## Examples to Run

```bash
# Vector fields
cargo run --example vector_field_showcase --release

# Streamlines
cargo run --example stream_lines_showcase --release

# Combined view
cargo run --example vector_field_and_streamlines --release

# Dynamic force fields
cargo run --example force_field_with_updaters --release
cargo run --example force_field_multiple_charges --release
```

## Comparison with Manim

| Feature | Manim | Murali | Status |
|---------|-------|--------|--------|
| VectorField | ✅ | ✅ | Implemented |
| StreamLines | ✅ | ✅ | Implemented |
| Magnitude coloring | ✅ | ✅ | Implemented |
| Custom starting points | ✅ | ✅ | Implemented |
| Dynamic fields | ✅ | ✅ | Via updaters |
| 3D vector fields | ✅ | ❌ | Not yet |

Murali now has feature parity with Manim for 2D vector field visualization!
