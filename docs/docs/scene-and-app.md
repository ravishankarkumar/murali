---
sidebar_position: 4
---

# Scene & App

## Scene

`Scene` is the container for all tattvas, timelines, and camera state.

```rust
use murali::engine::scene::Scene;

let mut scene = Scene::new();
```

### Adding tattvas

```rust
// Returns a TattvaId for later reference
let id = scene.add_tattva(shape, position);
```

### Accessing tattvas

```rust
// Untyped mutable access
scene.get_tattva_any_mut(id)

// Untyped immutable access
scene.get_tattva_any(id)

// Typed access (if you know the concrete type)
scene.get_tattva_typed::<Circle>(id)
scene.get_tattva_typed_mut::<Circle>(id)
```

### Camera

```rust
scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);
```

A z-position of `10.0` is a good default for 2D scenes. Move closer for a tighter view.

### Layout helpers

```rust
use murali::frontend::layout::Direction;

// Push a tattva to the edge of the screen with padding
scene.to_edge(id, Direction::Up, 0.35);
```

### Timelines

```rust
scene.timelines.insert("main".to_string(), timeline);
```

You can have multiple timelines. They all run in parallel.

---

## App

`App` is the entry point that runs your scene — either as a live preview window or a video export.

```rust
use murali::App;

App::new()?
    .with_scene(scene)
    .run_app()
```

### Preview vs export

By default, `run_app` checks CLI args to decide whether to preview or export. You can force a mode:

```rust
// Always open preview window
App::new()?.with_scene(scene).with_preview().run_app()

// Always export video
App::new()?.with_scene(scene).with_video_export().run_app()
```

### Render options

```rust
use murali::engine::render::RenderOptions;

App::new()?
    .with_scene(scene)
    .with_render_options(RenderOptions { ... })
    .run_app()
```

### Preview controls

When in preview mode:

| Key / Action | Effect |
|---|---|
| Left drag | Orbit camera |
| Scroll | Zoom |
| Right drag | Pan |
