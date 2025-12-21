# Murali Architecture Design Document (v2.1)

> **Goal:**
> Build a semantic, math-first animation engine where *meaning* is primary, *rendering* is deterministic, and *performance* scales to dense visuals — without sacrificing correctness or ergonomics.

---

## 1. High-Level Mental Model & Directory Mapping

Murali is split into **three conceptual layers**, each mapped to specific physical directories:

| Layer | Responsibility | Nature | Folder Path |
| --- | --- | --- | --- |
| **Tattva** | Semantic truth | Sparse, math-first | `src/frontend/` |
| **Projection** | Deterministic translation | Pure, stateless | `src/projection/` |
| **Backend** | Rendering cache & GPU | Dense, data-oriented | `src/backend/` |

**The "Machinery":** External tools (compilers, fonts, rasterizers) live in `src/resource/`.

---

## 2. Tattva: The Single Source of Truth (`src/frontend/`)

A **Tattva** represents *what the object means*, not how it is rendered. We categorize them to provide user-friendly clarity.

### Directory: `src/frontend/collection/`

* **`primitives/`**: Basic geometric building blocks (Circle, Square, Line).
* **`text/`**: Objects requiring resource backends (Label, Latex, Typst).
* **`composite/`**: High-level constructions (Axes, Grid).

### Structure (`src/frontend/mod.rs`)

```rust
pub struct Tattva<T> {
    pub id: TattvaId,
    pub state: T,                    // Math truth (e.g., Circle { radius: 1.0 })
    pub props: Arc<DrawableProps>,   // Shared visuals (Position, Scale, Rotation)
    dirty: bool,                     // Projection dirty flag
}

```

---

## 3. Dirty Tracking & Animation (`src/frontend/animation/`)

> **Projection runs only for dirty Tattvas.**

* Mutating `state` marks a Tattva **dirty** (requires a re-projection/re-mesh).
* Mutating `props` (via `Arc<DrawableProps>`) does **not** mark it dirty. This allows moving 10,000 objects every frame without re-computing their geometry.

---

## 4. Projection Layer (`src/projection/`)

The bridge that converts math state into backend-agnostic instructions.

* **`mod.rs`**: Defines the `Project` trait.
* **`primitives.rs`**: Defines the `RenderPrimitive` enum (Line, Quad, Mesh, GlyphRun).

### Rules

* Projection is **pure** and **stateless**.
* No access to ECS or Renderer. Same input must always produce the same output.

---

## 5. The Machinery (`src/resource/`)

All heavy lifting and file management are isolated here to keep the Semantic layer "pure."

* **`assets/`**: Raw binary files (Fonts, SVG templates).
* **`latex/`**: Tectonic compiler and SVG processing.
* **`text/`**: Font loading and glyph atlas packing.
* **`typst/`**: Typst compiler and SVG-to-Pixels rasterization.

---

## 6. Sync Boundary & Backend (`src/backend/`)

The interaction between semantics and rendering is forced through a single bottleneck.

### Sync Boundary (`src/backend/sync.rs`)

1. Iterate Tattvas in `Scene`.
2. Skip non-dirty Tattvas.
3. Run `project()` for dirty ones.
4. Update **ECS Components** (`src/backend/ecs/`) with new mesh handles or textures.

### Renderer (`src/backend/renderer/`)

* Consumes only ECS data.
* Manages GPU buffers, Shaders (`src/backend/renderer/shaders/`), and Pipelines.

---

## 7. Configuration & Orchestration (`src/engine/`)

The "Heart" that manages the lifecycle of the application.

* **`scene.rs`**: The container for all Tattvas.
* **`app.rs`**: The Winit event loop.
* **`config/`**: Policy-level settings (Preview vs. Export resolution).
* **`camera/`**: View logic (Orbit, Pan-Zoom).

---

## 8. Final Design Axioms

1. **Tattva owns meaning.**
2. **Geometry is expensive; Transforms are cheap.** (Only geometry marks dirty).
3. **The Backend is a Cache.** (ECS entities are replaceable).
4. **Resource isolation.** (Logic in `semantic` never touches files in `resource`).
5. **Deterministic loop.** (Timeline  Tattva  Project  Sync  Render).
