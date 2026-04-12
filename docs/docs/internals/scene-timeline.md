---
sidebar_position: 2
---

# Scene & Timeline Architecture

The `Scene` and `Timeline` are the heart of the Murali engine's frontend. They work together to maintain a deterministic, time-driven "source of truth" that is eventually projected onto the GPU.

## The Scene as Authority

The `Scene` struct is the authoritative container for everything you see. It follows a **unidirectional flow** model:

1. **Scene** (State) -> 2. **Sync Boundary** (Project) -> 3. **Backend** (ECS/wgpu)

### Key Responsibilities
- **Tattva Ownership**: Stores all active elements in a `HashMap<TattvaId, Box<dyn TattvaTrait>>`.
- **Global Context**: Manages the `Camera`, `scene_time`, and updaters.
- **Identity Management**: Handles stable IDs for objects so animations can target them consistently.

## The Frame Lifecycle

Every frame in Murali follows a strict sequence to ensure determinism:

### 1. Engine Update
The `Engine` triggers `scene.update(dt)`.

### 2. Scene Update
Inside `Scene::update`:
- **Time Advancement**: Increment `scene_time`.
- **Timeline Ticking**: All active timelines evaluate their scheduled animations against the new `scene_time`.
- **Updater Pass**: Procedural `Updater` callbacks run, allowing for non-linear logic (like tracking or constraints).

### 3. Timeline & Animations
The `Timeline` manages `ScheduledAnimation` entries. For each animation:
- **`on_start`**: Called when the current time reaches the animation's start time.
- **`apply_at(scene, t)`**: Called every frame with a normalized $t \in [0, 1]$.
- **`on_finish`**: Called when the duration has elapsed.

### 4. Sync Boundary
After the scene is updated, the `Engine` iterates over all "dirty" Tattvas. It calls `tattva.project()` to generate `RenderPrimitives`, which are then used to update the GPU-side resources in the Backend.

## State vs. Props

Murali makes a clean distinction between what can be animated:

- **Shared Props**: Standard properties common to all Tattvas (Position, Rotation, Scale, Opacity). These are modified via `DrawableProps::write(tattva.props())`.
- **Type-Specific State**: Properties unique to a specific shape (e.g., the `trim_end` of a `Path` or the `radius` of a `Circle`). These are accessed by downcasting the Tattva to its concrete type inside an animation.

## Determinism & The Reset Pass

Because Murali is time-driven, the scene must be predictable even if you jump back and forth in time (Seek).

To ensure that objects start in the correct state at $t=0$ without manual user setup (like calling `scene.hide()` manually), the engine performs a **Reverse Reset Pass**:

- When the scene starts or a seek occurs, the timeline iterates through its animations in **reverse chronological order**.
- Each animation's `reset(scene)` is called.
- By going in reverse, the **earliest** animation for any given object has the "final say" on how that object should look $t=0$. For example, a `Write` animation starting at $t=1$ will ensure the object is correctly hidden at $t=0$.

## Contributing Guidelines

When adding new animations or scene features:
- **Think Deterministically**: Avoid state that depends on previous frame results. Always derive state from the current timeline progress.
- **Use Hooks**: Prefer `on_start` for one-time setups (like creating shadow paths) and `on_finish` for permanent state changes (like converting a Circle to a permanent Path).
- **Favor Declarative API**: Add helper methods to `AnimationBuilder` to make new animations easily accessible to users.
