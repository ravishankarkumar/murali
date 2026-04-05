# Write & Unwrite Effects Guide

## Overview

Murali now supports **write** and **unwrite** effects for paths and shapes, inspired by Manim's signature animation style. These effects create the illusion of a path being drawn or erased by an invisible pen, with proper sector filling - only the completed portion of the shape is filled.

## How It Works

### Write Effect (Manim-Style Sector Filling)
The write effect animates a path with proper sector filling:
- **Outline Phase**: The path is traced from start to end using the `trim_end` property
- **Fill Phase**: The interior fill appears only for the completed sector
- The fill geometry is clipped to match the drawn portion, creating authentic sector filling
- For circles: the fill appears as a growing sector/pie slice
- For rectangles: the fill appears as a growing quadrilateral/triangle
- For any closed path: the fill follows the outline naturally

### Unwrite Effect
The unwrite effect reverses the write animation:
- **Erase Phase**: The outline is erased from the end (using `trim_start`)
- **Fill Preservation**: The fill remains in the portion that hasn't been erased yet
- The sector shrinks as the outline is erased, keeping the fill in the remaining portion

## Path Trim Properties

Paths now have three new properties for controlling the write effect:

```rust
pub struct Path {
    // ... existing fields ...
    
    /// Trim start: 0.0 = start of path, 1.0 = end of path
    pub trim_start: f32,
    
    /// Trim end: 0.0 = start of path, 1.0 = end of path
    pub trim_end: f32,
    
    /// Fill opacity: 0.0 = no fill, 1.0 = full fill
    pub fill_opacity: f32,
}
```

- **trim_start**: Controls where the path drawing begins (default: 0.0)
- **trim_end**: Controls where the path drawing ends (default: 1.0)
- **fill_opacity**: Controls the opacity of the path fill (default: 1.0)

## Usage

### Basic Write Animation (Manim-Style Sector Filling)

```rust
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::to_path::ToPath;
use murali::frontend::animation::Ease;
use glam::{Vec3, Vec4};
use murali::frontend::style::Style;
use murali::projection::style::ColorSource;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // Create a filled circle
    let circle = Circle::new(1.5, 48, Vec4::new(0.19, 0.64, 0.33, 1.0));
    let mut circle_path = circle.to_path();
    
    // Add fill to the circle
    circle_path.style = Style::new()
        .with_stroke(circle_path.style.stroke.unwrap_or_default())
        .with_fill(ColorSource::Solid(Vec4::new(0.19, 0.64, 0.33, 1.0)));
    
    let circle_id = scene.add_tattva(circle_path, Vec3::ZERO);

    // Animate with write effect - the fill appears as the outline is drawn
    timeline.animate(circle_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
```

**Result**: As the circle outline is drawn, the interior fill appears progressively, creating the authentic Manim sector-filling effect.

### Unwrite Animation

```rust
// Reverse the write effect - erase from the end while keeping fill in remaining portion
timeline.animate(circle_id)
    .at(2.5)
    .for_duration(2.0)
    .ease(Ease::InCubic)
    .unwrite()
    .spawn();
```

**Result**: 
- The outline is erased from the end
- The fill remains in the portion that hasn't been erased yet
- The sector shrinks smoothly as the outline disappears

## Sector Filling Behavior

The write effect implements true Manim-style sector filling:

- **As the outline is drawn** (trim_end goes from 0 to 1), the fill is clipped to only show the completed sector
- **For circles**: The fill appears as a growing sector/pie slice
- **For rectangles**: The fill appears as a growing quadrilateral or triangle
- **For any closed path**: The fill follows the outline, creating a natural "filling" effect

This is achieved by:
1. Calculating the path length
2. Extracting only the drawn portion of the path
3. Closing that portion back to the start point
4. Tessellating the resulting sector polygon
5. Rendering the clipped fill alongside the trimmed stroke

**Result**: No opacity-based fading - the fill appears solid only where the outline has been drawn.

### Example: Filled Square with Write Effect

```rust
use murali::frontend::collection::primitives::rectangle::Rectangle;

let square = Rectangle::new(2.0, 2.0, Vec4::new(0.92, 0.26, 0.21, 1.0));
let mut square_path = square.to_path();

// Add fill
square_path.style = Style::new()
    .with_stroke(square_path.style.stroke.unwrap_or_default())
    .with_fill(ColorSource::Solid(Vec4::new(0.92, 0.26, 0.21, 1.0)));

let square_id = scene.add_tattva(square_path, Vec3::ZERO);

// Write animation - fill appears as outline is drawn
timeline.animate(square_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .write()
    .spawn();
```

### Combining with Other Animations

You can combine write effects with other animations:

```rust
// Write the path while moving it
timeline.animate(circle_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .write()
    .spawn();

timeline.animate(circle_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .move_to(Vec3::new(3.0, 0.0, 0.0))
    .spawn();
```

## Converting Shapes to Paths

To use write effects on shapes, convert them to paths using the `ToPath` trait:

```rust
use murali::frontend::collection::primitives::to_path::ToPath;

// Circle to path
let circle = Circle::new(1.5, 48, color);
let path = circle.to_path();

// Rectangle to path
let rect = Rectangle::new(2.0, 1.5, color);
let path = rect.to_path();

// Square to path
let square = Square::new(1.5, color);
let path = square.to_path();

// Polygon to path
let polygon = Polygon::new(vertices, color);
let path = polygon.to_path();

// Line to path
let line = Line::new(start, end, thickness, color);
let path = line.to_path();
```

## Easing Functions

The write effect respects easing functions for smooth animations:

```rust
use murali::frontend::animation::Ease;

// Linear progression
.ease(Ease::Linear)

// Smooth acceleration
.ease(Ease::OutCubic)

// Smooth deceleration
.ease(Ease::InCubic)

// Smooth both ways
.ease(Ease::InOutCubic)
```

## Advanced: Manual Trim Control

For more control, you can manually set trim properties:

```rust
if let Some(path) = scene.get_tattva_typed_mut::<Path>(path_id) {
    path.state.trim_start = 0.0;
    path.state.trim_end = 0.5;  // Draw only half the path
    path.state.fill_opacity = 0.5;  // Semi-transparent fill
    path.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::STYLE);
}
```

## Performance Considerations

- Write effects calculate path length on each frame for accurate trimming
- For complex paths with many segments, consider using simpler geometries
- The effect is GPU-efficient as it uses existing line rendering with trim parameters

## Example: Agentic Flow Chart with Write Effects

You can apply write effects to agentic flow charts:

```rust
use murali::frontend::collection::ai::agentic_flow_chart::AgenticFlowChart;

let flow_chart = AgenticFlowChart::new(/* ... */);
let flow_id = scene.add_tattva(flow_chart, Vec3::ZERO);

// Animate the flow chart appearance
timeline.animate(flow_id)
    .at(0.0)
    .for_duration(3.0)
    .ease(Ease::OutCubic)
    .write()
    .spawn();
```

## Troubleshooting

### Write effect not visible
- Ensure the path has a stroke defined: `.with_color(color)`
- Check that `trim_end` is being animated from 0.0 to 1.0
- Verify the path has segments (not empty)

### Fill not appearing
- Ensure the path has a fill defined: `.with_style(style.with_fill(...))`
- Check that `fill_opacity` is being animated from 0.0 to 1.0
- The fill phase starts at 90% of the animation duration

### Performance issues
- Reduce the number of path segments
- Use simpler shapes when possible
- Consider using multiple shorter animations instead of one long one

## See Also

- [Path Primitive Documentation](./path-guide.md)
- [Animation System Guide](./animation-guide.md)
- [Example: write_effect_showcase.rs](../examples/write_effect_showcase.rs)
