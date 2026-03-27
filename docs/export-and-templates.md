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
