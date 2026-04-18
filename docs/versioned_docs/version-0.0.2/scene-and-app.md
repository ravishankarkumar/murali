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

Prefer the intent helpers when you only need to move, show, hide, or restyle a tattva:

```rust
scene.hide(id);
scene.show(id);
scene.set_opacity(id, 0.5);
scene.set_position_2d(id, glam::Vec2::new(1.0, 2.0));
scene.set_position_3d(id, glam::Vec3::new(1.0, 2.0, 0.5));
scene.set_scale(id, glam::Vec3::splat(1.25));
scene.set_rotation(id, glam::Quat::from_rotation_z(0.4));
```

Reach for low-level tattva access when you need to mutate the underlying state object or a specialized internal API:

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
scene.set_timeline("main", timeline);
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

By default, `run_app` exports video. Preview mode is entered when you explicitly opt into it, either in code or through the CLI.

In practice, most examples just do:

```rust
App::new()?.with_scene(scene).run_app()
```

That means they export unless preview is explicitly requested. You can force a mode when needed:

```rust
// Explicitly open preview window
App::new()?.with_scene(scene).with_preview().run_app()

// Always export video
App::new()?.with_scene(scene).with_video_export().run_app()
```

From the CLI, `--preview` forces preview mode and `--export` forces export mode. Since export is the default, `--preview` is the flag you usually use when you want an interactive window.

Preview mode is meant for checking animation feel while you work. It advances one animation frame at a time and tries not to run faster than the configured preview FPS.

If your machine is faster than that FPS, Murali waits and keeps preview playback at the target pace. If your machine is slower, Murali does not skip ahead to catch up. Preview will slow down, but you still get to see each frame in order.

This is intentionally different from games and streamed video. Those systems often drop or skip frames to stay locked to real time. Murali preview favors inspecting motion, easing, and visual changes frame by frame, so preserving every previewed frame is usually more useful than staying perfectly real-time under load.

Export is different: it always renders the full frame sequence at the chosen FPS, so the final video stays deterministic even if preview was slower on your machine.

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
