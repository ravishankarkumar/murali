---
sidebar_position: 6
---

# Timelines

A **Timeline** is how you schedule when animations happen in Murali. It's the conductor that orchestrates all the changes in your scene over time.

## What is a Timeline?

Think of a timeline as a musical score:
- It tells each "instrument" (tattva) when to play (start time)
- It specifies how long each note lasts (duration)
- It controls how the note is played (easing)
- It defines what happens (animation verb)

**Key insight:** Timelines don't store state—they schedule mutations. The scene holds the actual state, and the timeline tells it when and how to change.

## Creating a Timeline

```rust
use murali::engine::timeline::Timeline;

let mut timeline = Timeline::new();
```

That's it! A timeline starts empty, and you add animations to it.

## Adding Animations

The most common way to add animations is with the fluent builder API:

```rust
timeline
    .animate(tattva_id)           // What to animate
    .at(0.0)                      // When to start (seconds)
    .for_duration(2.0)            // How long it takes
    .ease(Ease::OutCubic)         // How it moves
    .move_to(Vec3::new(3.0, 0.0, 0.0))  // What changes
    .spawn();                     // Add to timeline
```

**Important:** Don't forget `.spawn()` at the end! Without it, the animation isn't added to the timeline.

## Timeline Lifecycle

### 1. Build Phase

You create animations and add them to the timeline:

```rust
let mut timeline = Timeline::new();

timeline.animate(id1).at(0.0).for_duration(2.0).move_to(...).spawn();
timeline.animate(id2).at(1.5).for_duration(1.5).scale_to(...).spawn();
timeline.animate(id3).at(2.0).for_duration(1.0).fade_to(...).spawn();
```

### 2. Play Phase

You give the timeline to the scene:

```rust
scene.play(timeline);
```

### 3. Runtime Phase

Each frame:
1. Scene time advances by `dt` (e.g., 1/60 second)
2. Timeline checks which animations should be active
3. Active animations apply their changes to tattvas
4. Scene state is updated
5. Renderer draws the new state

## One Timeline vs Many Timelines

### Single Timeline (Recommended)

Most scenes use a single timeline:

```rust
let mut timeline = Timeline::new();

// Add all your animations
timeline.animate(id1).at(0.0).for_duration(2.0).move_to(...).spawn();
timeline.animate(id2).at(1.0).for_duration(1.5).scale_to(...).spawn();
timeline.animate(id3).at(2.0).for_duration(1.0).appear().spawn();

// Play it
scene.play(timeline);  // Internally uses the name "main"
```

**This is the preferred API** for most use cases.

### Multiple Named Timelines

You can have multiple timelines for organization:

```rust
let mut main_timeline = Timeline::new();
let mut background_timeline = Timeline::new();
let mut ui_timeline = Timeline::new();

// Add animations to each...

scene.play_named("main", main_timeline);
scene.play_named("background", background_timeline);
scene.play_named("ui", ui_timeline);
```

**Important limitations:**
- All timelines share the same `scene_time`
- They progress together, not independently
- You can't pause one timeline while others continue
- You can't play timelines at different speeds

**When to use multiple timelines:**
- Organizing complex scenes by layer (foreground, background, UI)
- Separating concerns (content animations vs camera movements)
- Managing different "tracks" that you want to edit separately
- Code organization in large projects

**Current status:** This is an advanced feature that works but has limitations. Treat multiple timelines as organizational lanes for one shared scene clock, not as independent playback systems. Future versions may add more explicit control.

## Timeline Time vs Scene Time

**Scene time** is the authoritative clock. It's a single `f32` that represents "where we are" in the animation.

**Timeline time** is just how animations are scheduled relative to scene time.

```rust
// At scene_time = 1.5:
timeline.animate(id).at(0.0).for_duration(2.0).move_to(...).spawn();
// This animation is active (started at 0.0, ends at 2.0)

timeline.animate(id).at(2.0).for_duration(1.0).scale_to(...).spawn();
// This animation hasn't started yet (starts at 2.0)
```

All timelines advance with scene time. There's no separate "timeline time" that can drift or be controlled independently.

If you are choosing between one timeline and many, use one timeline by default. Reach for multiple named timelines when separating concerns makes a large scene easier to edit and reason about.

## Callbacks

Sometimes you need to run custom code at specific times.

### call_at

Run code once at a specific time:

```rust
timeline.call_at(2.0, |scene| {
    println!("Reached t=2.0!");
    scene.hide(some_id);
});
```

**Use cases:**
- Discrete events (show/hide objects)
- State changes
- Logging or debugging
- Cleanup

### call_during

Run code continuously over a duration:

```rust
timeline.call_during(1.0, 2.0, |scene, t| {
    // t goes from 0.0 to 1.0 over the duration
    // This runs every frame between scene_time 1.0 and 3.0
    
    let angle = t * std::f32::consts::TAU;
    let position = Vec3::new(
        angle.cos() * 3.0,
        angle.sin() * 3.0,
        0.0
    );
    scene.set_position_3d(circle_id, position);
});
```

**Use cases:**
- Complex motion paths (circles, spirals, custom curves)
- Dependent motion (one object following another)
- Procedural animations
- Custom interpolation logic

**Note:** The `t` parameter is normalized from 0.0 to 1.0, regardless of the actual duration.

## Advanced Timeline Features

### Morphing Groups

Morph multiple tattvas at once:

```rust
timeline.morph_matching_staged(
    source_ids,      // Vec<TattvaId>
    target_ids,      // Vec<TattvaId>
    &mut scene,
    1.0,             // start time
    2.0,             // duration
    Ease::InOutCubic,
);
```

This automatically stages (hides) the target tattvas and morphs them from the sources.

### Signal Playback

For procedural or signal-driven animations:

```rust
use murali::engine::timeline::SignalPlayback;

// Play once
let playback = SignalPlayback::once(0.0, 2.0, Ease::OutCubic);
timeline.play_signal(tattva_id, playback);

// Round trip (there and back)
let playback = SignalPlayback::round_trip(0.0, 2.0, Ease::InOutQuad);
timeline.play_signal(tattva_id, playback);

// Loop multiple times
let playback = SignalPlayback::looped(0.0, 1.0, 5, Ease::Linear);
timeline.play_signal(tattva_id, playback);
```

### Wait Until

Ensure the scene runs until a specific time, even if all animations finish earlier:

```rust
timeline.wait_until(10.0);
```

This is useful for adding a pause at the end of your animation before it loops or exits.

### End Time

Get when the timeline finishes:

```rust
let end = timeline.end_time();
println!("Animation ends at t={}", end);
```

This considers all scheduled animations and any `wait_until` calls.

## Sequencing Patterns

### Sequential (One After Another)

```rust
let mut timeline = Timeline::new();
let mut current_time = 0.0;

// Animation 1
timeline.animate(id1).at(current_time).for_duration(2.0).move_to(...).spawn();
current_time += 2.0;

// Animation 2 (starts when 1 ends)
timeline.animate(id2).at(current_time).for_duration(1.5).scale_to(...).spawn();
current_time += 1.5;

// Animation 3 (starts when 2 ends)
timeline.animate(id3).at(current_time).for_duration(1.0).appear().spawn();
```

### Parallel (All at Once)

```rust
let mut timeline = Timeline::new();

// All start at the same time
timeline.animate(id1).at(0.0).for_duration(2.0).move_to(...).spawn();
timeline.animate(id2).at(0.0).for_duration(2.0).scale_to(...).spawn();
timeline.animate(id3).at(0.0).for_duration(2.0).fade_to(...).spawn();
```

### Staggered (Overlapping)

```rust
let mut timeline = Timeline::new();
let stagger_delay = 0.2;

for (i, id) in tattva_ids.iter().enumerate() {
    timeline
        .animate(*id)
        .at(i as f32 * stagger_delay)
        .for_duration(1.0)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
}
```

### Overlapping (Start Before Previous Ends)

```rust
let mut timeline = Timeline::new();

// Animation 1: 0.0 to 2.0
timeline.animate(id1).at(0.0).for_duration(2.0).move_to(...).spawn();

// Animation 2: 1.5 to 3.0 (overlaps with 1)
timeline.animate(id2).at(1.5).for_duration(1.5).scale_to(...).spawn();

// Animation 3: 2.5 to 3.5 (overlaps with 2)
timeline.animate(id3).at(2.5).for_duration(1.0).appear().spawn();
```

## Common Patterns

### Intro → Content → Outro

```rust
let mut timeline = Timeline::new();

// Intro: Title appears
timeline.animate(title_id).at(0.0).for_duration(1.0).appear().spawn();

// Content: Main animation
timeline.animate(content_id).at(1.5).for_duration(3.0).draw().spawn();

// Outro: Everything fades out
timeline.animate(title_id).at(5.0).for_duration(1.0).fade_to(0.0).spawn();
timeline.animate(content_id).at(5.0).for_duration(1.0).fade_to(0.0).spawn();
```

### Build Up Then Transform

```rust
let mut timeline = Timeline::new();

// Build: Reveal all pieces
for (i, id) in piece_ids.iter().enumerate() {
    timeline
        .animate(*id)
        .at(i as f32 * 0.3)
        .for_duration(0.8)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
}

// Transform: Move pieces into final positions
let transform_start = piece_ids.len() as f32 * 0.3 + 1.0;
for (i, id) in piece_ids.iter().enumerate() {
    timeline
        .animate(*id)
        .at(transform_start)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .move_to(final_positions[i])
        .spawn();
}
```

### Synchronized Multi-Property Animation

```rust
let mut timeline = Timeline::new();

// Move and scale at the same time
timeline
    .animate(id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .move_to(Vec3::new(3.0, 0.0, 0.0))
    .spawn();

timeline
    .animate(id)
    .at(0.0)
    .for_duration(2.0)
    .ease(Ease::OutCubic)
    .scale_to(Vec3::splat(2.0))
    .spawn();

// Fade out while moving back
timeline
    .animate(id)
    .at(3.0)
    .for_duration(1.5)
    .ease(Ease::InCubic)
    .move_to(Vec3::ZERO)
    .spawn();

timeline
    .animate(id)
    .at(3.0)
    .for_duration(1.5)
    .ease(Ease::InCubic)
    .fade_to(0.0)
    .spawn();
```

## Timeline Best Practices

### Do's

✅ **Use descriptive timing constants**
```rust
const INTRO_START: f32 = 0.0;
const INTRO_DURATION: f32 = 1.5;
const CONTENT_START: f32 = INTRO_START + INTRO_DURATION + 0.5;
const CONTENT_DURATION: f32 = 3.0;
```

✅ **Group related animations**
```rust
// Title animations
timeline.animate(title_id).at(0.0).for_duration(1.0).appear().spawn();
timeline.animate(title_id).at(5.0).for_duration(1.0).fade_to(0.0).spawn();

// Content animations
timeline.animate(content_id).at(1.5).for_duration(2.0).draw().spawn();
timeline.animate(content_id).at(5.0).for_duration(1.0).undraw().spawn();
```

✅ **Use staggering for visual interest**
```rust
for (i, id) in ids.iter().enumerate() {
    timeline
        .animate(*id)
        .at(i as f32 * 0.2)
        .for_duration(1.0)
        .appear()
        .spawn();
}
```

✅ **Add pauses between sections**
```rust
// Section 1: 0.0 to 3.0
// Pause: 3.0 to 3.5
// Section 2: 3.5 to 6.0
```

### Don'ts

❌ **Don't forget `.spawn()`**
```rust
// This does nothing!
timeline.animate(id).at(0.0).for_duration(2.0).move_to(...);
// Missing .spawn()
```

❌ **Don't use magic numbers**
```rust
// Bad
timeline.animate(id).at(2.347).for_duration(1.823).move_to(...).spawn();

// Good
const REVEAL_TIME: f32 = 2.35;
const REVEAL_DURATION: f32 = 1.8;
timeline.animate(id).at(REVEAL_TIME).for_duration(REVEAL_DURATION).move_to(...).spawn();
```

❌ **Don't make timings too tight**
```rust
// Bad - no breathing room
timeline.animate(id1).at(0.0).for_duration(1.0).appear().spawn();
timeline.animate(id2).at(1.0).for_duration(1.0).appear().spawn();

// Better - add small gaps
timeline.animate(id1).at(0.0).for_duration(1.0).appear().spawn();
timeline.animate(id2).at(1.3).for_duration(1.0).appear().spawn();
```

❌ **Don't try to control timeline playback speed**
```rust
// This doesn't exist (yet)
// timeline.set_speed(0.5);  // ❌ Not supported
```

## Debugging Timelines

### Print Timeline Info

```rust
let end_time = timeline.end_time();
println!("Timeline ends at: {:.2}s", end_time);
```

### Add Debug Callbacks

```rust
timeline.call_at(1.0, |_scene| {
    println!("Checkpoint 1 reached");
});

timeline.call_at(2.5, |_scene| {
    println!("Checkpoint 2 reached");
});

timeline.call_at(5.0, |_scene| {
    println!("Animation complete");
});
```

### Visualize Timing

```rust
// Print a simple timeline visualization
println!("Timeline:");
println!("0.0s: Title appears");
println!("1.5s: Content draws");
println!("3.0s: Transform begins");
println!("5.0s: Fade out");
println!("6.0s: End");
```

## Troubleshooting

**Animations don't play:**
- Did you call `scene.play(timeline)`?
- Did you forget `.spawn()` at the end of animations?
- Are start times reasonable (not negative, not too large)?

**Animations happen in wrong order:**
- Check your `.at(time)` values
- Make sure you're not reusing the same time for sequential animations
- Print `timeline.end_time()` to verify total duration

**Animation feels wrong:**
- Try different easing functions
- Adjust duration (too fast or too slow?)
- Add small delays between animations for breathing room

**Multiple timelines don't work as expected:**
- Remember: all timelines share scene_time
- They can't run at different speeds
- Consider using a single timeline with organized sections instead

## What's Next?

- **[Animations](./animations)** - Learn all available animation verbs
- **[Scene and App](./scene-and-app)** - Understand scene management
- **[Updaters](./updaters)** - For frame-by-frame custom logic
- **[Camera](./camera)** - Animate the camera
