# Text Reveal Effects Guide

## Overview

Murali now supports **two distinct text animation modes**:

1. **Typewriter Effect** (`.write_text()` / `.unwrite_text()`)
   - Text stays in fixed position
   - Grows from left to right
   - Characters appear sequentially

2. **Reveal Effect** (`.reveal_text()` / `.unreveal_text()`)
   - Text shifts as it grows
   - Grows from center outward
   - Characters appear sequentially with shifting

## Typewriter Effect

### Description
Text remains in its original position and grows from left to right, like a typewriter typing.

### Visual Progression
```
Position: Fixed at (0, 0)

0% written:   ""
25% written:  "Hello"
50% written:  "Hello Mur"
75% written:  "Hello Mural"
100% written: "Hello Murali!"
```

### Usage

```rust
let label = Label::new("Hello Murali!", 0.5)
    .with_color(Vec4::new(0.19, 0.64, 0.33, 1.0));
let label_id = scene.add_tattva(label, Vec3::ZERO);

// Write effect - typewriter style
timeline.animate(label_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .write_text()
    .spawn();

// Unwrite effect - reverse typewriter
timeline.animate(label_id)
    .at(2.5)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .unwrite_text()
    .spawn();
```

### Best For
- UI text animations
- Dialogue/narration
- Sequential text reveals
- When text position should remain fixed

## Reveal Effect

### Description
Text shifts as it grows, appearing to expand from the center outward. Creates a more dynamic, eye-catching effect.

### Visual Progression
```
Position: Shifts as text grows

0% revealed:   ""                    (centered)
25% revealed:  "Hello"               (centered)
50% revealed:  "Hello Mur"           (centered)
75% revealed:  "Hello Mural"         (centered)
100% revealed: "Hello Murali!"       (centered)
```

### Usage

```rust
let label = Label::new("Hello Murali!", 0.5)
    .with_color(Vec4::new(0.92, 0.26, 0.21, 1.0));
let label_id = scene.add_tattva(label, Vec3::ZERO);

// Reveal effect - text grows from center
timeline.animate(label_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .reveal_text()
    .spawn();

// Unreveal effect - text shrinks to center
timeline.animate(label_id)
    .at(2.5)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .unreveal_text()
    .spawn();
```

### Best For
- Emphasis/highlight animations
- Single text focus
- Dynamic, attention-grabbing effects
- When text position can shift

## Text Properties

Both Label and LaTeX have two properties controlling reveal behavior:

```rust
pub struct Label {
    pub text: String,
    pub world_height: f32,
    pub color: Vec4,
    /// Character reveal progress: 0.0 = no characters, 1.0 = all characters
    pub char_reveal: f32,
    /// Reveal mode: true = typewriter (fixed), false = reveal (shifting)
    pub typewriter_mode: bool,
}
```

## Animation Builder Methods

### Typewriter Mode
```rust
.write_text()      // Reveal characters left to right
.unwrite_text()    // Hide characters right to left
```

### Reveal Mode
```rust
.reveal_text()     // Reveal characters from center
.unreveal_text()   // Hide characters to center
```

## Supported Text Types

- **Label**: Simple text for UI, annotations, or labels
- **LaTeX**: Mathematical equations and complex text

## Easing Functions

Both effects support all easing functions:
- `Ease::Linear` - Constant speed (recommended for text)
- `Ease::OutCubic` - Fast start, slow end
- `Ease::InCubic` - Slow start, fast end
- `Ease::InOutCubic` - Smooth both ways

## Comparison

| Feature | Typewriter | Reveal |
|---------|-----------|--------|
| Position | Fixed | Shifts |
| Growth Direction | Left to right | Center outward |
| Best For | UI, dialogue | Emphasis, focus |
| Visual Style | Classic typewriter | Dynamic, modern |
| Single Text | Good | Better |
| Multiple Texts | Better | Good |

## Examples

### Typewriter Example
```rust
// Text stays at position (0, 0) and grows left to right
let label = Label::new("Loading...", 0.4);
let label_id = scene.add_tattva(label, Vec3::ZERO);

timeline.animate(label_id)
    .at(0.0)
    .for_duration(1.5)
    .ease(Ease::Linear)
    .write_text()
    .spawn();
```

### Reveal Example
```rust
// Text shifts as it grows from center
let label = Label::new("Important!", 0.5)
    .with_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
let label_id = scene.add_tattva(label, Vec3::ZERO);

timeline.animate(label_id)
    .at(0.0)
    .for_duration(1.5)
    .ease(Ease::Linear)
    .reveal_text()
    .spawn();
```

### LaTeX Examples
```rust
// Typewriter mode
let latex = Latex::new("f(x) = x^2 + 2x + 1", 0.4);
let latex_id = scene.add_tattva(latex, Vec3::ZERO);

timeline.animate(latex_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .write_text()
    .spawn();

// Reveal mode
timeline.animate(latex_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .reveal_text()
    .spawn();
```

## Implementation Details

### Typewriter Mode
- Renders only the revealed portion of text
- No offset applied
- Text naturally grows from left to right

### Reveal Mode
- Renders only the revealed portion of text
- Applies center offset: `offset_x = (full_width - revealed_width) / 2.0`
- Text appears to grow from center outward

## Performance

- Character reveal calculation: O(n) where n = number of characters
- Efficient string slicing using Rust's char iterator
- No additional memory overhead
- Suitable for real-time animation

## Combining with Other Animations

You can combine text reveal effects with other animations:

```rust
// Reveal text while moving it
timeline.animate(label_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .reveal_text()
    .spawn();

timeline.animate(label_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .move_to(Vec3::new(3.0, 0.0, 0.0))
    .spawn();
```

## See Also

- [Text Write Effect Guide](./text-write-effect-guide.md) - Original typewriter effect
- [Example: text_write_effect_showcase.rs](../examples/text_write_effect_showcase.rs)
- [Example: text_reveal_effects_showcase.rs](../examples/text_reveal_effects_showcase.rs)
