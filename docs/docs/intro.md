---
sidebar_position: 1
---

# Introduction

Murali is a Rust-powered, GPU-accelerated math animation engine inspired by Manim. It lets you create precise, time-driven animations for mathematical concepts — rendered via wgpu with support for LaTeX, Typst, parametric surfaces, and more.

## Why Murali?

Manim is great, but it has real limitations:

- Built on OpenGL, which is deprecated on macOS
- Frame-driven logic makes determinism hard
- Python's dynamic typing makes large scenes hard to reason about

Murali addresses these with a wgpu backend (Vulkan/Metal/DX12), a strictly time-driven animation model, and Rust's type system.

## Core concepts

- **Tattva** — any object in a scene (shape, text, composite). The word means "element" or "essence" in Sanskrit.
- **Scene** — holds all tattvas and timelines
- **Timeline** — schedules animations against a time axis
- **World space** — all coordinates are in mathematical units, not pixels

## Installation

Murali is currently available from GitHub. Add it to your `Cargo.toml`:

```toml
[dependencies]
murali = { git = "https://github.com/ravishankarkumar/murali" }
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

Run it:

```bash
cargo run --example your_scene
```

By default this opens a preview window. Pass `--video` to export frames instead.
