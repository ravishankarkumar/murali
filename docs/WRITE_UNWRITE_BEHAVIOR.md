# Write and Unwrite Effect Behavior

## Overview

The write and unwrite effects implement true Manim-style sector filling where:
- **Write**: The sector grows as the outline is drawn, with fill appearing in the drawn portion
- **Unwrite**: The sector shrinks as the outline is erased, with fill remaining in the unwritten portion

## Write Effect

### Animation Properties
- `trim_start`: 0.0 (fixed)
- `trim_end`: 0.0 → 1.0 (animates)
- `fill_opacity`: 1.0 (fixed)

### Visual Progression
```
0% drawn:   [empty]
25% drawn:  [quarter sector filled]
50% drawn:  [half sector filled]
75% drawn:  [three-quarter sector filled]
100% drawn: [full shape filled]
```

### How It Works
1. As `trim_end` increases from 0 to 1, the outline is drawn
2. The fill is clipped to only show the drawn sector
3. The sector grows smoothly, with fill appearing in the drawn portion
4. When complete, the entire shape is filled

### Code
```rust
timeline.animate(shape_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .write()
    .spawn();
```

## Unwrite Effect

### Animation Properties
- `trim_start`: 0.0 → 1.0 (animates)
- `trim_end`: 1.0 (fixed)
- `fill_opacity`: 1.0 (fixed)

### Visual Progression
```
0% erased:   [full shape filled]
25% erased:  [three-quarter sector filled]
50% erased:  [half sector filled]
75% erased:  [quarter sector filled]
100% erased: [empty]
```

### How It Works
1. As `trim_start` increases from 0 to 1, the outline is erased from the end
2. The fill is clipped to only show the unwritten portion
3. The sector shrinks smoothly, with fill remaining in the unwritten portion
4. When complete, nothing is visible

### Code
```rust
timeline.animate(shape_id)
    .at(2.5)
    .for_duration(2.0)
    .ease(Ease::InCubic)
    .unwrite()
    .spawn();
```

## Detailed Examples

### Circle Write Effect
```
Start:  ○ (empty circle outline)
        ↓
25%:    ◐ (quarter sector filled)
        ↓
50%:    ◑ (half sector filled)
        ↓
75%:    ◕ (three-quarter sector filled)
        ↓
End:    ● (full circle filled)
```

### Circle Unwrite Effect
```
Start:  ● (full circle filled)
        ↓
25%:    ◕ (three-quarter sector filled)
        ↓
50%:    ◑ (half sector filled)
        ↓
75%:    ◐ (quarter sector filled)
        ↓
End:    ○ (empty circle outline)
```

### Rectangle Write Effect
```
Start:  ┌─┐ (empty rectangle)
        │ │
        └─┘
        ↓
25%:    ┌─┐ (triangle at corner filled)
        │▓│
        └─┘
        ↓
50%:    ┌─┐ (half rectangle filled)
        │▓│
        └▓┘
        ↓
75%:    ┌─┐ (three-quarter filled)
        │▓│
        └▓┘
        ↓
End:    ┌─┐ (full rectangle filled)
        │▓│
        └▓┘
```

### Rectangle Unwrite Effect
```
Start:  ┌─┐ (full rectangle filled)
        │▓│
        └▓┘
        ↓
25%:    ┌─┐ (three-quarter filled)
        │▓│
        └▓┘
        ↓
50%:    ┌─┐ (half rectangle filled)
        │▓│
        └─┘
        ↓
75%:    ┌─┐ (triangle at corner filled)
        │▓│
        └─┘
        ↓
End:    ┌─┐ (empty rectangle)
        │ │
        └─┘
```

## Key Differences from Opacity-Based Approach

### Opacity-Based (❌ Not Used)
- Fill fades in/out gradually
- Color appears suddenly when opacity reaches full
- Doesn't create authentic sector effect
- Looks like transparency, not filling

### Sector-Based (✅ Current Implementation)
- Fill appears only in drawn/unwritten portion
- No opacity fading - solid fill
- Authentic Manim-style sector effect
- Looks like the shape is being drawn/erased

## Implementation Details

### Write Animation
```rust
impl Animation for WritePath {
    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let eased_t = self.ease.eval(t);
        
        path.state.trim_end = eased_t;        // Draw outline
        path.state.fill_opacity = 1.0;        // Fill is clipped to sector
    }
}
```

### Unwrite Animation
```rust
impl Animation for UnwritePath {
    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let eased_t = self.ease.eval(t);
        
        path.state.trim_start = eased_t;      // Erase from end
        path.state.trim_end = 1.0;            // Keep end fixed
        path.state.fill_opacity = 1.0;        // Fill in unwritten portion
    }
}
```

### Sector Fill Rendering
```rust
// Build trimmed sector path
let fill_points = self.build_trimmed_fill_path(trim_start_dist, trim_end_dist);

// Close the path back to start
points.push(first_point);

// Tessellate the sector polygon
tessellator.tessellate_path(&lpath, &FillOptions::default(), &mut geometry);

// Render solid fill
ctx.emit(RenderPrimitive::Mesh(mesh));
```

## Easing Functions

Both write and unwrite effects support all easing functions:
- `Ease::Linear` - Constant speed
- `Ease::OutCubic` - Fast start, slow end (recommended for write)
- `Ease::InCubic` - Slow start, fast end (recommended for unwrite)
- `Ease::InOutCubic` - Smooth both ways

## Supported Shapes

Any shape that can be converted to a path:
- Circles
- Rectangles
- Squares
- Polygons
- Lines
- Custom paths

## Performance

- Path length calculation: O(n) per frame
- Sector path building: O(n) with curve sampling
- Tessellation: O(m) where m = sector points
- Overall: Efficient for real-time animation

## Compatibility

✅ Works with all shapes
✅ Works with all easing functions
✅ Doesn't interfere with morphing
✅ Doesn't interfere with other animations
✅ Works with fill colors and gradients

## Examples

- `write_effect_showcase.rs` - Basic write/unwrite effects
- `manim_sector_fill_demo.rs` - Comparison of filled vs outline shapes
