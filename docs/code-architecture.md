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

* **Sangh<T>:** The atomic unit of truth. It wraps a specific math state (`T`) and a shared transform (`Props`).
* **Responsibility:** Owns the "Meaning." If you want to change the range of an Axis or the text of a Label, you do it here.
* **Dirty Tracking:** If `T` changes, `dirty = true`. If only `Props` (position) change, the object is NOT dirty (no re-projection needed).

### Layer B: The Projection Layer (`src/projection/`)

**Nature:** Pure, Stateless, Backend-agnostic.

* **The Project Trait:** `fn project(&self, ctx: &mut ProjectionCtx)`.
* **Responsibility:** Converts a Sangh's math state into a list of **RenderPrimitives** (Lines, Meshes, Glyphs).
* **The Boundary:** This layer does not know about WGPU, Buffers, or the ECS World. It only knows about Geometry.

### Layer C: The Backend Layer (`src/backend/`)

**Nature:** Dense, GPU-optimized, Volatile.

* **The Sync Boundary:** The "Garbage Collector" of the engine. It takes Primitives from the Projector and updates the ECS.
* **ECS (Hecs):** Holds the raw data for the GPU. Entities here are **disposable**.
* **Renderer (WGPU):** Only looks at the ECS. It draws batches of lines and triangles.

---

## 3. Directory Mapping

| New Directory | Old Directory/Concept | Description |
| --- | --- | --- |
| `src/frontend/` | `sangh/`, `tattva/` | Math objects (Circle, Axes, Equation). |
| `src/projection/` | `projection/` | The `Project` trait and `RenderPrimitive` enums. |
| `src/backend/` | `renderer/`, `ecs/`, `core/` | WGPU pipelines, ECS World, and Sync logic. |
| `src/engine/` | `app.rs`, `scene/`, `timeline/` | The glue: Time, Animation, and Window events. |
| `src/resource/` | `assets/`, `latex/`, `typst/` | IO, Font loading, and Rasterization logic. |

---

## 4. The Data Flow (The "Murali Loop")

1. **Animate:** The `Timeline` mutates a `Sangh` or its `Props`.
2. **Check Dirty:** The `Scene` iterates Sanghs.
3. **Project:** If a Sangh is `dirty`, the `Project` trait runs. It outputs new `RenderPrimitives`.
4. **Sync:** The `SyncBoundary` compares new Primitives to the ECS. It spawns/kills ECS entities and updates GPU Buffers.
5. **Draw:** The `Renderer` queries the ECS and submits a single, optimized Command Encoder to the GPU.

---

## 5. Non-Negotiable Rules

1. **No Tattva Meshes:** A Sangh must never return a `wgpu::Buffer`. It returns a `RenderPrimitive::Mesh { vertices: Vec<V> }`.
2. **No Logic in ECS:** Systems must not "calculate" math in the ECS. The ECS is a passive mirror of the Semantic layer.
3. **Transform vs. Geometry:** Moving a Sangh (position) is cheap. Changing its shape (projection) is expensive. Keep them separate.
4. **Determinism:** Given the same `SceneTime`, the `Project` layer must produce the exact same `RenderPrimitives`.

---

## 6. Checklist: "Am I doing this right?"

* **I want to add a new shape:** Create a struct in `semantic/collection/` and implement `Project`.
* **I want to animate a property:** Add that property to the Sangh's state and mark it `dirty` when it changes.
* **I want to optimize rendering:** Go to `backend/renderer/` and improve the WGSL shaders or the batching logic.
* **I'm getting a "Mismatched Type" error:** You likely have two versions of `Scene`. Ensure `engine/scene.rs` is the only one.