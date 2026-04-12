---
sidebar_position: 1
---

# Introduction

Murali v0.0.1 is the initial proof-of-concept release. It establishes the rendering foundation: a wgpu backend, a world-space coordinate system, and the ability to place basic shapes and text in a scene.

There is no animation system in this version. Scenes are static — you define what's in the scene and it renders a single frame or a preview window.

## Installation

```toml
[dependencies]
murali = { git = "https://github.com/ravishankarkumar/murali", tag = "v0.0.1" }
```

## Your first scene

```rust
use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::circle::Circle;
use glam::{Vec3, Vec4};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Circle::new(1.0, 64, Vec4::new(0.2, 0.6, 1.0, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
```

## What's in v0.0.1

- wgpu rendering backend
- World-space coordinate system (origin at center, 16:9 aspect ratio)
- Basic primitives: Circle, Square, Rectangle, Line
- Embedded Typst for LaTeX rendering
- Preview window via winit

## What's not in v0.0.1

- No animation or timeline system
- No morphing
- No video export
- No axes or graph primitives
- No layout helpers (`to_edge`, anchors)

See v0.1.0 for the first release with animations.
