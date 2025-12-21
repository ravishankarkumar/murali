
# Murali Architecture Design Document (v2)

> **Goal:**
> Build a semantic, math-first animation engine where *meaning* is primary, *rendering* is deterministic, and *performance* scales to dense visuals — without sacrificing correctness or ergonomics.

---

## 0. Guiding Principles (Non-Negotiable)

1. **Math first, pixels last**
2. **Preview == Export (determinism)**
3. **Semantic intent must never be inferred**
4. **Rendering backends are replaceable**
5. **ECS accelerates rendering, never decides meaning**
6. **If nothing changed, nothing should recompute**

---

## 1. High-Level Mental Model

Murali is split into **three conceptual layers**:

| Layer                 | Responsibility                      | Nature               |
| --------------------- | ----------------------------------- | -------------------- |
| **Sangh / Construct** | Semantic truth                      | Sparse, math-first   |
| **Projection**        | Deterministic translation           | Pure, stateless      |
| **ECS Backend**       | Rendering cache & GPU orchestration | Dense, data-oriented |

These layers **must not leak into each other**.

---

## 2. Sangh: The Single Source of Truth

A **Sangh** represents *what the object means*, not how it is rendered.

Examples:

* Axes
* LaTeX Equation
* Label
* Electric Field
* Particle System (semantic description)

### Structure

```rust
pub struct Sangh<T> {
    pub id: SanghId,
    pub state: T,                    // Math truth
    pub props: Arc<DrawableProps>,   // Shared transform (position/scale/rotation)
    dirty: bool,                     // Projection dirty flag
}
```

### Rules (Strict)

* Sangh **owns all math state**
* Sangh **never stores ECS entities**
* Sangh **never mutates ECS**
* Any mutation of `state` **must mark `dirty = true`**
* Transform-only changes via `props` **do not mark dirty**

### Example

```rust
axes.set_x_range(-5.0, 5.0); // marks dirty
axes.props.position.x += 2.0; // NOT dirty
```

This distinction is critical.

---

## 3. Dirty Tracking (Partial Projection)

### Motivation

In a scene with:

* Axes
* LaTeX equation
* 10,000 particles

If **only LaTeX changes**, we must *not* re-project particles.

### Design Rule

> **Projection runs only for dirty Sanghs.**

### Mechanism

```rust
for sangh in scene.sanghs.iter_mut() {
    if !sangh.dirty {
        continue;
    }

    sangh.project(&mut ctx);
    sangh.dirty = false;
}
```

Dirty is **semantic**, not rendering-related.

---

## 4. Projection Layer (Sangh → Render Description)

Projection is the **deterministic, pure conversion** from math state to renderable primitives.

### Trait

```rust
pub trait Project {
    fn project(&self, ctx: &mut ProjectionCtx);
}
```

### ProjectionCtx

```rust
pub struct ProjectionCtx<'a> {
    pub out: &'a mut Vec<RenderPrimitive>,
}
```

### RenderPrimitive (backend-agnostic)

```rust
enum RenderPrimitive {
    Line { start: Vec3, end: Vec3, thickness: f32, color: Color },
    Quad { size: Vec2, uv: UvRect, texture: TextureId },
    GlyphRun { glyphs: Vec<GlyphInstance> },
    ParticleBatch { instances: Vec<ParticleInstance> },
}
```

### Properties

* Projection is **pure**
* No ECS access
* No mutation
* Same input → same output
* Easy to test

---

## 5. ECS Backend (Render Cache + GPU Orchestrator)

ECS exists **only** to efficiently render large numbers of primitives.

### What ECS Is Responsible For

* Holding render instances
* Batching
* Instancing
* GPU buffer management
* Texture size limits
* Hardware constraints
* Culling / sorting

### What ECS Is *Not* Allowed to Do

* Decide semantics
* Modify math state
* Run animations
* Infer intent
* Propagate transforms logically

### ECS Components (Dumb by Design)

```rust
Transform
Opacity
GeometryHandle
Layer
InstanceData
GpuDirty
```

> ECS entities are **replaceable cache entries**, not identities.

---

## 6. Sync Boundary (Critical)

All interaction between semantics and rendering goes through **one explicit step**:

```rust
scene.sync_to_ecs();
```

### Sync Process

1. Iterate Sanghs
2. Skip non-dirty Sanghs
3. Run `project()`
4. Diff new primitives vs previous
5. Update ECS minimally
6. Enforce GPU limits (clamping, warnings)
7. Mark relevant ECS entities dirty for upload

This is where:

* LaTeX texture clamping happens
* Particle caps are enforced
* Renderer warnings are generated

---

## 7. Animation System (Timeline-Driven, Deterministic)

Animations operate on **Sangh state or DrawableProps**, never ECS.

### Animation Definition

```rust
pub struct Animation<T> {
    target: SanghId,
    duration: f32,
    curve: Easing,
    apply: Box<dyn Fn(&mut T, f32)>,
}
```

### Playback

```rust
timeline.tick(dt);
scene.sync_to_ecs();
```

### Consequences

* Deterministic
* Rewindable
* Preview == Export
* Debuggable
* Serializable

---

## 8. Groups, Constructs, and Composition

### Multiple Groups

* Each Sangh has its own `DrawableProps`
* Group movement = transform change
* Geometry unchanged → no re-projection

### Constructs

A **Construct** is a factory for Sanghs.

```rust
trait Construct {
    fn build(&self) -> Vec<Sangh<_>>;
}
```

Examples:

* Axes construct
* NumberPlane construct
* ElectricField construct
* AI Transformer construct

---

## 9. Particle Systems (Why ECS Is Needed)

### Semantic Layer

```rust
struct ParticleField {
    origin: Vec3,
    velocity_fn: Fn(Vec3) -> Vec3,
    count: usize,
}
```

### Projection

* Generates particle instances
* Marks Sangh dirty only if field parameters change

### ECS

* Stores thousands of instances
* Uses GPU instancing
* Updates transforms efficiently

Particles move every frame **without re-projection**, unless semantics change.

---

## 10. Why This Architecture Works

### Compared to ECS-First Designs

| Aspect          | ECS-First | Murali          |
| --------------- | --------- | --------------- |
| Source of truth | ECS       | Sangh           |
| Semantics       | Emergent  | Explicit        |
| Debugging       | Hard      | Straightforward |
| Determinism     | Fragile   | Guaranteed      |
| Math animation  | Awkward   | Natural         |

### Compared to Scene Graphs

* No hierarchy
* No recursive transforms
* No borrow hell
* Shared transforms via `Arc<DrawableProps>`

---

## 11. Design Axioms (Lock These In)

1. **Sangh owns meaning**
2. **Projection is pure**
3. **Dirty means semantic change**
4. **ECS is a cache**
5. **One sync boundary**
6. **Transforms ≠ geometry**
7. **Nothing re-computes unless it must**

---

## 12. Immediate Next Steps (Implementation Order)

1. Add `dirty: bool` to Sangh
2. Define `Project` trait
3. Implement `ProjectionCtx`
4. Refactor **Axes** to:

   * store only math state
   * implement `project()`
5. Add `scene.sync_to_ecs()`
6. Route renderer to consume ECS only

---

## 13. Long-Term Payoff

This architecture enables, cleanly:

* Axes with ticks & labels
* Equation morphing
* Symbol-level math animation
* Particle systems
* Vector fields
* AI diagrams
* Deterministic video export
* GPU scalability

Without turning Murali into a game engine.

---

### Final Statement

> **Murali is a semantic animation engine with a data-oriented renderer — not the other way around.**