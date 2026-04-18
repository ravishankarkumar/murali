# Murali Terminology Guide

This document defines standard terminology usage across all Murali documentation.

## Core Terms

### Tattva
- **Definition:** Any visual object in a scene (shape, text, composite, etc.)
- **Etymology:** Sanskrit word meaning "element" or "essence"
- **Usage:**
  - Capitalize when referring to the concept: "A Tattva is any object..."
  - Lowercase in code: `scene.add_tattva(...)`
  - Plural: "tattvas" (lowercase)
  - Possessive: "tattva's properties"

### Scene
- **Definition:** Container for all tattvas, timelines, and camera
- **Usage:**
  - Capitalize when referring to the concept: "The Scene owns all tattvas"
  - Lowercase in code: `let mut scene = Scene::new()`
  - Think of it as: "A stage in a theater"

### Timeline
- **Definition:** Schedule of animations and callbacks over time
- **Usage:**
  - Capitalize when referring to the concept: "A Timeline defines how properties evolve"
  - Lowercase in code: `let mut timeline = Timeline::new()`
  - Singular vs plural:
    - "one timeline" (most common case)
    - "multiple timelines" (advanced scheduling)
    - "named timelines" (when using `play_named()`)

### World Space
- **Definition:** Coordinate system using mathematical units, not pixels
- **Usage:**
  - Always "world space" (lowercase, two words)
  - "world units" when referring to measurements
  - "world coordinates" when referring to positions

## Animation Terms

### Preview vs Export
- **Preview:** Interactive window mode for development
  - Command: `cargo run --example my_scene`
  - Purpose: Fast iteration, debugging
- **Export:** Headless video rendering mode
  - Command: `cargo run --example my_scene -- --export`
  - Purpose: Final output, deterministic frames

### Animation Methods

#### play() vs play_named() vs set_timeline()
- **`scene.play(timeline)`** - Recommended for single timeline (most common)
- **`scene.play_named("name", timeline)`** - For multiple timelines
- **`scene.set_timeline(timeline)`** - Low-level, advanced use only

**Recommendation in docs:** Always show `play()` first, mention `play_named()` for advanced cases.

## Capitalization Rules

### In Prose
- Capitalize when introducing concepts: "A Tattva is..."
- Capitalize in headings: "## Scene and Timeline"
- Lowercase when used as common nouns: "add tattvas to the scene"

### In Code Examples
- Always use actual Rust casing: `Scene`, `Timeline`, `TattvaId`
- Method names are lowercase: `add_tattva()`, `play()`, `animate()`

## Common Phrases

### Preferred
- "add a tattva to the scene"
- "build a timeline"
- "schedule animations"
- "world-space coordinates"
- "preview mode" / "export mode"
- "scene time" (the current time in the scene)

### Avoid
- "create a tattva" (use "add" instead - tattvas are added to scenes)
- "pixel coordinates" (always use world space)
- "frame-based animation" (Murali is time-based)
- "render mode" (use "preview" or "export")

## Cross-Reference Terms

When linking between docs:
- "See [Tattvas](./tattvas/)" - capitalize in link text
- "Learn about [animations](./animations)" - lowercase for action-oriented links
- "Read the [Scene and App](./scene-and-app) guide" - capitalize proper titles

## Consistency Checklist

When writing or reviewing docs, ensure:
- [ ] Tattva/Scene/Timeline capitalized when introducing concepts
- [ ] Code examples use correct Rust casing
- [ ] "preview" and "export" used consistently (not "render mode")
- [ ] "world space" / "world units" used instead of "pixels"
- [ ] `play()` shown before `play_named()` in examples
- [ ] Links use consistent capitalization

## Examples

### Good
```markdown
A **Tattva** is any visual object in your scene. You add tattvas using `scene.add_tattva()`.

The **Scene** owns all tattvas and timelines. Create a scene with `Scene::new()`.

Use **preview mode** for development and **export mode** for final output.
```

### Needs Improvement
```markdown
A **tattva** is any visual object. You create tattvas with `scene.add_tattva()`.

The **scene** owns everything. Use render mode to see your animation.
```

## Updates

This terminology guide should be updated when:
- New core concepts are introduced
- API naming changes
- User feedback indicates confusion
- Documentation patterns evolve
