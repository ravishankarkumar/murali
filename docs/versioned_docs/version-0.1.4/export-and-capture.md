---
sidebar_position: 11
---

# Export And Capture

Murali supports two main workflows:

- preview a scene interactively while authoring
- export frames, video, screenshots, or GIF captures for deliverables

The most important thing to know is that preview and export are different modes. Preview opens a window for interactive inspection. Export renders deterministically from `scene_time` and writes output to disk.

## Preview Vs Export

Preview is for authoring:

```rust
App::new().with_preview().run(scene)?;
```

or from the CLI:

```bash
cargo run --example basics_showcase -- --preview
```

Export is the default behavior if you do not opt into preview:

```rust
App::new().run(scene)?;
```

or:

```bash
cargo run --example basics_showcase -- --export
```

Use preview when you are iterating on timing, layout, or camera choices. Use export when you want reproducible assets.

## Basic Export Controls

`App` exposes a few high-level helpers:

```rust
App::new()
    .with_video_export()
    .with_frames_export(true)
    .run(scene)?;
```

What these mean:

- `.with_preview()` switches into interactive preview mode
- `.with_video_export()` ensures video export is enabled
- `.with_frames_export(true)` enables frame export in addition to video export

`with_frames_export(true)` does not mean "frames only." By default, both video and frames are enabled unless you explicitly override render options.

## Render Options

Use `RenderOptions` when you want to control output behavior without building a full `ExportSettings` value yourself.

```rust
use murali::engine::render::RenderOptions;

let options = RenderOptions {
    video: Some(true),
    frames: Some(true),
    fps: Some(60),
    resolution: Some((1920, 1080)),
    output: Some("render_output/demo.mp4".to_string()),
};

App::new()
    .with_render_options(options)
    .run(scene)?;
```

Use this path when you mainly care about:

- output resolution
- frame rate
- output video path
- whether frames and/or video should be written

## ExportSettings

Use `ExportSettings` when you want more direct control over the export pipeline:

```rust
use glam::Vec4;
use murali::engine::export::{ExportSettings, export_scene};

let settings = ExportSettings {
    width: 1920,
    height: 1080,
    fps: 60,
    duration_seconds: 6.0,
    output_dir: "render_output/frames".into(),
    basename: "demo".to_string(),
    video_path: Some("render_output/demo.mp4".into()),
    gif_path: Some("render_output/demo.gif".into()),
    capture_gif_dir: Some("render_output/capture_gifs".into()),
    clear_color: Vec4::new(0.05, 0.10, 0.15, 1.0),
};

export_scene(scene, &settings)?;
```

Reach for `ExportSettings` when you want to control:

- exact duration
- exact frame output directory and basename
- video path
- top-level GIF output
- capture GIF directory
- clear color

## Screenshots At Authored Times

If you want screenshots from specific authored moments, schedule them on the `Scene`:

```rust
scene.capture_screenshots([0.5, 1.0, 2.0]);
```

If you want named output files:

```rust
scene.capture_screenshots_named([
    (0.5, Some("captures/intro.png")),
    (1.0, None),
    (2.0, Some("captures/final.png")),
]);
```

This is usually the nicest authoring model for tutorials, blog posts, and docs assets because the capture schedule lives with the scene itself.

## GIF Capture Groups

You can also schedule small GIF capture groups from authored times:

```rust
scene.capture_gif("overview", [0.5, 1.0, 2.0]);
scene.capture_gif("focus", [2.5, 2.7, 2.9]);
```

Those captures are assembled into GIFs when `capture_gif_dir` is configured in `ExportSettings`.

This is useful when you want:

- short explainer GIFs from a longer scene
- multiple derived assets from one authored scene
- social/documentation snippets without reauthoring the animation

## ScreenshotMarker

`ScreenshotMarker` is a utility tattva for capture-oriented scenes, but most authored capture workflows should prefer the scene-level helpers above first:

- `scene.capture_screenshots(...)`
- `scene.capture_screenshots_named(...)`
- `scene.capture_gif(...)`

Use `ScreenshotMarker` when you explicitly want capture intent represented as a tattva in the scene graph.

## Deterministic Export

Export is deterministic in the sense that it advances the scene frame by frame from authored time rather than relying on interactive preview timing.

That makes export the right mode for:

- final deliverables
- reproducible screenshots
- asset generation in CI or scripted workflows
- checking whether authored timing really lands as expected

## Practical Advice

- use preview for iteration speed
- use export when checking the final timing and framing
- use scene-level capture helpers for doc/blog assets
- use `RenderOptions` for lightweight output control
- use `ExportSettings` for production-style exports

## Related Docs

- [Scene and App](./scene-and-app)
- [Examples](./examples/showcase)
- [Utility Tattvas](./tattvas/utility)
