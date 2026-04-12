---
sidebar_position: 1
---

# Introduction

Murali is a modern alternative to Manim, built in Rust for performance, precision, and control. It is a GPU-accelerated engine for mathematical animation designed to create **mathematically precise, programmatically defined animations**.

Unlike traditional animation tools, Murali treats animations as deterministic functions of time. Every frame is derived from a well-defined timeline, making your visuals predictable, reproducible, and easy to reason about.

Powered by `wgpu`, Murali runs natively across Vulkan, Metal, and DirectX, and provides first-class support for LaTeX, Typst, and parametric geometry.

## Why Murali?

Manim is powerful, but it comes with trade-offs that become apparent as projects grow:

* Built on OpenGL, which is deprecated on macOS
* Frame-driven animation logic introduces implicit state and non-determinism
* Python’s dynamic typing makes large, complex scenes harder to maintain

Murali is designed to solve these at a foundational level:

* **Modern GPU backend** — built on `wgpu` (Vulkan / Metal / DX12)
* **Time-driven architecture** — animations are explicit functions of time, not frames
* **Rust type system** — enables safer, more maintainable large-scale animation code

The result is an engine that feels closer to **engineering a system** than scripting visuals.

## Core concepts

Murali introduces a small set of composable primitives:

* **Tattva** — any object in a scene (shape, text, or composite). Derived from Sanskrit, meaning “element” or “essence.”
* **Scene** — a container for all tattvas and their timelines
* **Timeline** — defines how properties evolve over time
* **World space** — all coordinates exist in mathematical units, not pixels

These abstractions allow you to describe animations declaratively, while retaining full programmatic control.

## Installation

Murali is currently available via GitHub. Add it to your `Cargo.toml`:

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

By default, this opens a preview window. Pass `--video` to export frames instead.

## Why timeline-based animation?

Murali is built around a timeline-based animation model for several key reasons:

1. **Determinism**  
   Animations are defined as explicit functions of time, making their behavior predictable and reproducible.

2. **Precise temporal control**  
   Elements can be scheduled with exact timing—ideal for synchronizing with audio, video, or pre-defined narratives.

3. **Random access (planned)**  
   In a timeline-based system, the state of a scene can be evaluated at any time instantly, without replaying previous frames.  
   This capability is not yet implemented and will be introduced in future iterations. The current focus is on feature completeness.

4. **Composability**  
   Animations combine naturally as independent functions of time, avoiding implicit state interactions.

## Why the name “Murali”?

The name *Murali* comes from the flute of Lord Krishna, which, in Hindu tradition, is said to produce music so pure that it touches the soul.

The goal of this project is similar: to make teaching feel effortless, so that ideas flow clearly from teacher to student. Murali aims to make mathematical expression not just precise, but deeply intuitive.
