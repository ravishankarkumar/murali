# Export And Templates

Murali now has the structure for deterministic export, plus themes and AI-teaching templates.

## Export

Use [export_aiu_attention.rs](/Users/ravishankar/personal-work/murali/examples/export_aiu_attention.rs) as the reference:

```bash
cargo run --example export_aiu_attention
```

What it does:

- advances the scene on a fixed timestep
- renders each frame offscreen
- writes PNG frames to `renders/...`
- assembles an MP4 with `ffmpeg` when available

The main API lives in [export.rs](/Users/ravishankar/personal-work/murali/src/engine/export.rs).

### Project Config

Murali now looks for a `murali.toml` file next to the nearest `Cargo.toml` from the current working directory.

Example:

```toml
[export]
width = 1920
height = 1080
fps = 60
duration_seconds = 4.0
output_dir = "renders/frames"
basename = "lesson"
video_path = "renders/lesson.mp4"
gif_path = "renders/lesson.gif"
clear_color = [0.05, 0.10, 0.15, 1.0]
```

`video_path` may be either a file path or a directory.
If it is a directory, Murali will generate a filename automatically, usually from the running binary/example name, and fall back to a timestamped name if needed.

### Default Behavior

`App::run_app()` now exports video by default.

To force interactive preview instead, pass:

```bash
cargo run --example your_example -- --preview
```

Current status:

- compile-time path exists
- runtime export is still blocked by a macOS frame-readback stall in the current renderer path
- themes, templates, and example structure are usable now

## Diagnostics

Murali now includes a built-in doctor command:

```bash
cargo run -- doctor
```

It reports availability of:

- `latex`
- `dvisvgm`
- `ffmpeg`

## Themes

Theme presets live in [theme.rs](/Users/ravishankar/personal-work/murali/src/frontend/theme.rs).

Current presets:

- `Theme::ai_under_the_hood()`
- `Theme::classroom_light()`

These themes are semantic color packs for scene objects and export backgrounds.

## AI Under The Hood Templates

Reusable AI-teaching presets live in [templates.rs](/Users/ravishankar/personal-work/murali/src/frontend/collection/ai/templates.rs).

Current helpers:

- pre-styled `NeuralNetworkDiagram`
- pre-styled `TokenSequence`
- pre-styled `AttentionMatrix`
- pre-styled `TransformerBlockDiagram`

See [aiu_attention_template.rs](/Users/ravishankar/personal-work/murali/examples/aiu_attention_template.rs) for an interactive lesson scene.
