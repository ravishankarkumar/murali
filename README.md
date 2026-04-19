# Murali

Murali is a Rust-based animation engine for semantic graphics and mathematical scenes. It is built around deterministic timelines, a frontend scene model, CPU-side projection, and a GPU-backed runtime.

For full documentation and showcases, visit [muraliengine.com](https://muraliengine.com)

## Goals

- Predictable, explicit animation behavior
- World-space authoring instead of pixel-first APIs
- Clear separation between authored scene state and render/runtime state
- A modern GPU path built on `wgpu`

## Current Shape

- `src/frontend/` contains user-facing tattvas, animations, layout helpers, and scene authoring APIs
- `src/projection/` contains backend-neutral render primitives and meshes
- `src/backend/` contains the sync boundary, ECS cache, and renderer
- `src/engine/` contains scene ownership, app lifecycle, timeline stepping, export, and config
- `examples/` contains runnable examples and feature showcases
- `docs/` contains the longer-form documentation site

## Getting Started

Requirements:

- Rust toolchain
- A working graphics environment for preview
- `ffmpeg` if you want video export

Run an example in preview mode:

```bash
cargo run --example primitives_showcase
```

Some useful examples:

```bash
cargo run --example animated_motion
cargo run --example axes_and_labels
cargo run --example agentic_flow_chart
cargo run --example export_aiu_attention
```

## Preview And Export Config

Murali looks for the nearest `murali.toml` next to a `Cargo.toml`. If no config file is present, sensible defaults are used.

Example config:

```toml
[preview]
fps = 60

[export]
fps = 60
width = 1920
height = 1080
```

A sample file is included at [murali.toml.example](./murali.toml.example).

## Documentation

- Project overview: [Project Overview](https://muraliengine.com)
- Scene and app docs: [Scene and App](https://muraliengine.com/docs/scene-and-app)
- Internal architecture: [Architecture Overview](https://muraliengine.com/docs/architecture/overview)


## Examples

### Shapes 

[![Watch the video](./resources/shapes.png)](https://youtu.be/rzQZHta2PQM)

### Animation showcase

[![Watch the video](./resources/animation_showcase.png)](https://youtu.be/W8WQQbSo70Y)

## Status

Murali is under active development. The repository already includes:

- scene and timeline infrastructure
- preview and headless export paths
- text, LaTeX, and Typst support
- primitives, layout helpers, tables, graph tattvas, and utility tattvas
- write/unwrite, transform, text, and surface animation building blocks

## License

Murali is licensed under the Apache License, Version 2.0.
