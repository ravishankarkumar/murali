# Murali

Murali is a **Rust-based, GPU-accelerated animation engine** inspired by Manim. It is designed for creating **precise, semantic mathematical graphics and animations** with a strong focus on correctness, determinism, and long-term maintainability.

## Why Murali when Manim exists?

Manim is an excellent tool and has played a major role in popularizing mathematical animations. Today, there are two prominent variants: the community-maintained Manim, which emphasizes stability and backward compatibility, and the original Manim by 3Blue1Brown, which is more experimental and leverages full OpenGL capabilities.

Despite its strengths, Manim has some limitations that motivated the creation of Murali.

### 1. GPU backend and future compatibility

Manim is built on top of OpenGL. While this works well on many platforms, OpenGL is effectively deprecated on macOS, where only older versions are supported and no future updates are planned by Apple.

For a niche, open-source tool, it is difficult to continuously chase evolving GPU standards across platforms. **WebGPU (via wgpu)** offers a more future-proof, cross-platform abstraction over modern graphics APIs (Vulkan, Metal, DX12), making it a better long-term foundation.

---

### 2. Predictability and intent

Manim is powerful, but it can sometimes behave in ways that are hard to predict unless you are a power user deeply familiar with its internal conventions. This can lead to frustration when the output does not match the author’s intent.

Murali aims to prioritize **explicitness and predictability** in its core abstractions, so that animations do what they are meant to do, without relying on undocumented behaviors or fragile tricks.

---

### 3. Strongly typed, engine-first design

Murali is written in **Rust**, a strongly typed language that enables:

- Clear and explicit APIs
- Better tooling support (autocomplete, static analysis, AI-assisted tooling)
- Fewer runtime surprises

An engine-first design also encourages clean separation between:
- Rendering
- Layout
- Time
- Semantics

---

### 4. Performance potential

While performance is not the primary goal, a Rust-based engine with a modern GPU backend has the potential to significantly improve rendering performance. Actual performance gains will be evaluated later through benchmarking once the engine matures.

---

### 5. Community interest

There has been growing interest within the community (including Manim’s own Discord) around alternative implementations in Rust. Murali explores this space with a focus on sound engine architecture rather than a direct feature-by-feature port.

---

## What this project is not

- Murali is **not** a line-by-line reimplementation of Manim in Rust.
- It does **not** aim to replicate all Manim APIs verbatim.
- It intentionally respects Rust’s idioms and engineering constraints.

Early development reflects some personal design opinions. As the project matures and gains adoption, these decisions are expected to evolve with community input.

---

## Project Status

Murali is under **active development**.

### Currently implemented
- WGPU-based rendering backend
- World-space rendering pipeline (in progress)
- Embedded **Typst** integration (Typst → SVG → RGBA)
- Text rendered as textured quads
- Multiple basic tattvas rendered in a scene

### Under active development
- Coordinate system & camera
- World-space text sizing and positioning
- Anchors and relative placement
- Time-driven animation system

See [`ROADMAP.md`](./ROADMAP.md) for the full development plan.

---

## Repository Structure

```text
murali/
├── engine/          # Core animation engine (rendering, scene, tattvas, time)
├── examples/        # Example scenes using the engine
├── ROADMAP.md       # Development roadmap
└── README.md
````

---

## Running Examples

### Run a basic scene with multiple tattvas

```bash
cargo run -p examples --example scene_many_tattvas
```

This example renders a scene containing multiple basic tattvas using the engine.

---

### Run the Typst text example

```bash
cargo run -p examples \
  --features engine/typst_embedded \
  --example typst_hello
```

or 

```bash
cargo run -p examples \
  --features engine/typst_embedded \
  --example typst_world_text
```

This example demonstrates:

* Embedded Typst compilation
* SVG → RGBA rasterization
* Rendering Typst text inside Murali


``` bash
cargo run --example primitives_showcase
cargo run --example axes_and_labels
cargo run --example animated_motion
```

---

## Design Principles

Murali is built around the following principles:

* **World space first, pixels last**
  All layout and animation operate in mathematical world units.

* **Time-driven, not frame-driven**
  Animations depend on time, not frame count, ensuring determinism.

* **Semantic math over visual hacks**
  Mathematical meaning is preserved through transformations and morphing.

* **Ergonomics without compromising correctness**
  Expressive APIs are built on top of solid engine foundations.

---

## Inspiration

Murali is inspired by:

* [Manim](https://www.manim.community/) — semantic math animation concepts
* GPU-first rendering engines
* Mathematical visualization tools

Murali does **not** aim to be a direct Manim clone. Instead, it rethinks similar ideas with a strong emphasis on engine architecture, determinism, and Rust’s type system.

---

## License

Murali is licensed under the Apache License, Version 2.0.
See [LICENSE](./LICENSE) for details.
