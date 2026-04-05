# Text Write & Unwrite Effects Guide

## Overview

Murali now supports **write** and **unwrite** effects for text, implementing a typewriter-style character reveal animation similar to Manim's text animations. Characters are revealed one by one as the animation progresses.

## How It Works

### Write Effect (Typewriter Style)
The write effect reveals text character by character:
- **Character Reveal**: Characters appear progressively from the start
- **Position Fixed**: Text stays in its original position and grows from left to right
- **Progress**: `char_reveal` goes from 0.0 (no characters) to 1.0 (all characters)
- **Smooth Animation**: Characters appear smoothly based on easing function

### Unwrite Effect
The unwrite effect hides text character by character:
- **Character Hide**: Characters disappear progressively from the end
- **Position Fixed**: Text stays in its original position and shrinks from right to left
- **Progress**: `char_reveal` goes from 1.0 (all characters) to 0.0 (no characters)
- **Smooth Animation**: Characters disappear smoothly based on easing function

## Supported Text Types

- **Label**: Simple text for UI, annotations, or labels
- **LaTeX**: Mathematical equations and complex text

## Text Properties

Both Label and LaTeX now have a `char_reveal` property:

```rust
pub struct Label {
    pub text: String,
    pub world_height: f32,
    pub color: Vec4,
    /// Character reveal progress: 0.0 = no characters, 1.0 = all characters
    pub char_reveal: f32,
}

pub struct Latex {
    pub source: String,
    pub world_height: f32,
    pub color: Vec4,
    /// Character reveal progress: 0.0 = no characters, 1.0 = all characters
    pub char_reveal: f32,
}
```

## Usage

### Basic Write Animation

```rust
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::collection::text::label::Label;
use murali::frontend::animation::Ease;
use glam::{Vec3, Vec4};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // Create a label
    let label = Label::new("Hello Murali!", 0.5)
        .with_color(Vec4::new(0.19, 0.64, 0.33, 1.0));
    let label_id = scene.add_tattva(label, Vec3::ZERO);

    // Animate with write effect - characters appear one by one
    timeline.animate(label_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .write_text()
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
```

### Unwrite Animation

```rust
// Reverse the write effect - characters disappear one by one
timeline.animate(label_id)
    .at(2.5)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .unwrite_text()
    .spawn();
```

### LaTeX Write Animation

```rust
use murali::frontend::collection::text::latex::Latex;

let latex = Latex::new("f(x) = x^2 + 2x + 1", 0.5)
    .with_color(Vec4::new(0.96, 0.80, 0.19, 1.0));
let latex_id = scene.add_tattva(latex, Vec3::ZERO);

// Animate with write effect
timeline.animate(latex_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .write_text()
    .spawn();
```

## Visual Progression

### Write Effect Example
```
0% written:   ""
25% written:  "Hello"
50% written:  "Hello Mur"
75% written:  "Hello Mural"
100% written: "Hello Murali!"

Position: Always at the same location, text grows from left to right
```

### Unwrite Effect Example
```
0% unwritten: "Hello Murali!"
25% unwritten: "Hello Mural"
50% unwritten: "Hello Mur"
75% unwritten: "Hello"
100% unwritten: ""

Position: Always at the same location, text shrinks from right to left
```

## Easing Functions

Both write and unwrite effects support all easing functions:
- `Ease::Linear` - Constant speed (recommended for text)
- `Ease::OutCubic` - Fast start, slow end
- `Ease::InCubic` - Slow start, fast end
- `Ease::InOutCubic` - Smooth both ways

## Animation Builder Methods

```rust
// Write text character by character
.write_text()

// Unwrite text character by character
.unwrite_text()
```

## Examples

- `text_write_effect_showcase.rs` - Comprehensive text write/unwrite demo

## Implementation Details

### Character Reveal Calculation

```rust
fn get_revealed_text(&self) -> String {
    let char_count = self.text.chars().count();
    let reveal_count = (char_count as f32 * self.char_reveal.clamp(0.0, 1.0)).ceil() as usize;
    self.text.chars().take(reveal_count).collect()
}
```

The `char_reveal` value is multiplied by the total character count to determine how many characters to show. This ensures smooth character-by-character reveal.

### Position Behavior

**Key Feature**: Text stays in its original position and grows/shrinks from left to right.

This is achieved by:
1. Only rendering the revealed portion of the text
2. Not applying any offset or transformation
3. The text naturally grows as more characters are revealed
4. The text naturally shrinks as characters are hidden

This creates the authentic Manim-style text write effect where:
- Text appears to be "written" from left to right
- Already written characters don't move
- The text position remains fixed throughout the animation

### Animation Implementation

**WriteText Animation:**
```rust
impl Animation for WriteText {
    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let eased_t = self.ease.eval(t);
        label.state.char_reveal = eased_t;  // 0.0 → 1.0
    }
}
```

**UnwriteText Animation:**
```rust
impl Animation for UnwriteText {
    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let eased_t = self.ease.eval(t);
        label.state.char_reveal = 1.0 - eased_t;  // 1.0 → 0.0
    }
}
```

## Performance

- Character reveal calculation: O(n) where n = number of characters
- Efficient string slicing using Rust's char iterator
- No additional memory overhead
- Suitable for real-time animation

## Compatibility

✅ Works with Label text
✅ Works with LaTeX text
✅ Works with all easing functions
✅ Works with all colors and styles
✅ Doesn't interfere with other animations
✅ Can be combined with position/rotation/scale animations

## Combining with Other Animations

You can combine text write effects with other animations:

```rust
// Write text while moving it
timeline.animate(label_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::Linear)
    .write_text()
    .spawn();

timeline.animate(label_id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .move_to(Vec3::new(3.0, 0.0, 0.0))
    .spawn();
```

## See Also

- [Write Effect Guide](./write-effect-guide.md) - Path write effects
- [Example: text_write_effect_showcase.rs](../examples/text_write_effect_showcase.rs)
