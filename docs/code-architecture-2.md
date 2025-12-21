# Murali Architectural Blueprint (v2.1)

## 1. The Core Philosophy

Murali is not a game engine; it is a **Deterministic Projection Engine**.

* **Sparse Semantics:** We store only what an object *is* (e.g., "a circle with radius 1.0").
* **Dense Execution:** We render using high-performance data structures (ECS/GPU).
* **The Bridge:** The transition from Semantics to Execution is a **Pure Projection**.

---

## 2. Layer Definitions & Responsibilities

### Layer A: The Semantic Layer (`src/frontend/`)

**Nature:** Sparse, Math-rich, Human-readable.

* **Tattva<T>:** The atomic unit of truth. It wraps a specific math state (`T`) and a shared transform (`Arc<DrawableProps>`).
* **Responsibility:** Owns the "Meaning." If you want to change the range of an Axis or the text of a Label, you do it here.
* **Dirty Tracking:** If `T` changes, `dirty = true`. If only `props` (position) change, the object is NOT dirty. No re-projection/re-meshing occurs.

### Layer B: The Projection Layer (`src/projection/`)

**Nature:** Pure, Stateless, Backend-agnostic.

* **The Project Trait:** `fn project(&self, ctx: &mut ProjectionCtx)`.
* **Responsibility:** Converts a Tattva's math state into a list of **RenderPrimitives** (Lines, Meshes, Glyphs).
* **The Boundary:** This layer is a "No-IO" zone. It does not know about WGPU, Buffers, or the ECS World. It only knows about Geometry.

### Layer C: The Backend Layer (`src/backend/`)

**Nature:** Dense, GPU-optimized, Volatile.

* **The Sync Boundary:** The bridge between `projection` and `ecs`. It processes Primitives from the Projector and updates the ECS.
* **ECS (Hecs):** Holds the raw data for the GPU. Entities here are **disposable cache entries**, not persistent identities.
* **Renderer (WGPU):** A passive system that queries the ECS and submits optimized Command Encoders to the GPU.

---

## 3. Directory Mapping

| New Directory | Logic Category | Responsibility |
| --- | --- | --- |
| `src/frontend/` | **The Truth** | `collection/` (Shapes, Text), `animation/`, `props.rs`. |
| `src/projection/` | **The Bridge** | `Project` trait and `RenderPrimitive` definitions. |
| `src/backend/` | **The Muscle** | `sync.rs`, `ecs/` components, and `renderer/` pipelines. |
| `src/resource/` | **The Machinery** | `assets/`, `latex/`, `typst/`, `text/` (Heavy IO/Compilers). |
| `src/engine/` | **The Glue** | `app.rs` loop, `scene.rs` container, `config/`, and `camera/`. |
| `src/math/` | **The Utils** | Pure math, `interpolation.rs`, and `transform.rs`. |

---

## 4. The Data Flow (The "Murali Loop")

1. **Animate:** The `Timeline` mutates a `Tattva` state or its `props`.
2. **Check Dirty:** The `Scene` iterates Tattvas.
3. **Project:** If a Tattva is `dirty`, the `Project` trait runs. It outputs new `RenderPrimitives`.
4. **Sync:** The `SyncBoundary` compares new Primitives to the ECS. It spawns/kills ECS entities and updates GPU Buffers.
5. **Draw:** The `Renderer` queries the ECS and draws everything in a single pass.

---

## 5. Non-Negotiable Rules

1. **No Tattva Meshes:** A Tattva must never store a `wgpu::Buffer`. It emits a `RenderPrimitive` which is then turned into a buffer by the Backend.
2. **Resource Isolation:** Code in `semantic` must never perform File IO. It requests resources (like fonts) via IDs/Paths that the `resource/` layer resolves.
3. **Transform vs. Geometry:** Moving a Tattva (position) is cheap. Changing its shape (projection) is expensive. **Keep them separate.**
4. **Determinism:** Given the same `SceneTime`, the `Project` layer must produce the exact same `RenderPrimitives`.

---

## 6. Checklist: "Am I doing this right?"

* **I want to add a new shape:** Create a struct in `semantic/collection/primitives/` and implement `Project`.
* **I want to animate a property:** Add that property to the Tattva's `state` and call `mark_dirty()` in its setter.
* **I want to change font rendering:** Go to `resource/text/` or the `text.wgsl` shader.
* **I'm seeing performance lag:** Check `backend/sync.rs` to see if too many objects are being marked `dirty` unnecessarily.
