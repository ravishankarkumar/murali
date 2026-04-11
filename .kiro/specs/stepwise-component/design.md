# Design Document: Stepwise Component (Phases 7–11)

## Overview

This document covers the design for completing the Stepwise component in the Murali animation library.
Phases 1–6 (model, layout, timeline engine, rendering, transitions, signal) are already implemented
and working. This design addresses Phases 7–11:

- **Phase 7**: Script API — `stepwise(|s| { s.step("...") })` closure builder in `script.rs`
- **Phase 8**: ConnectionBuilder — `s.connect(a, b).route([Up, Right, Down])` in `script.rs`
- **Phase 9**: StepContent trait — extensible node content in `model.rs`, `view.rs`, `render.rs`
- **Phase 10**: Debug overlay — opt-in text overlay in `render.rs`
- **Phase 11**: Polish — easing functions in `easing.rs`, updated colors/scale in `render.rs`

The existing architecture is sound: `StepwiseModel` → `TimelineEngine` → `StepwiseState` → `StepwiseRenderer`.
All new work layers on top of this pipeline without modifying its core logic.

---

## Architecture

```
User Script (script.rs)
    stepwise(|s| {
        let a = s.step("Observe");
        let b = s.step("Reason");
        s.connect(a, b).route([Right]);
    })
         │
         ▼
    ScriptBuilder::build()
         │
         ▼
    StepwiseModel  ◄── unchanged core data model
    (steps, transitions, sequence)
         │
         ▼
    TimelineEngine::compute(model, progress)  ◄── unchanged
         │
         ▼
    StepwiseState  ◄── unchanged single source of truth
         │
         ▼
    StepwiseRenderer::update(scene, view, state)
    ├── applies easing (easing.rs)  ◄── new
    ├── calls StepContent::update() ◄── new
    └── updates debug overlay       ◄── new (opt-in)
```

The data flow is strictly one-directional. No component modifies time or controls execution.

---

## Components and Interfaces

### script.rs — ScriptBuilder and ConnectionBuilder

`ScriptBuilder` is the struct passed into the `stepwise(|s| { ... })` closure. It accumulates
steps and connections, then builds a `StepwiseModel` on drop or via an explicit `build()` call.

```rust
pub fn stepwise<F>(f: F) -> StepwiseModel
where
    F: FnOnce(&mut ScriptBuilder),

pub struct ScriptBuilder {
    steps: Vec<Step>,
    explicit_connections: Vec<(usize, usize, Option<Vec<Direction>>)>,
    has_explicit_connections: bool,
}

impl ScriptBuilder {
    pub fn step(&mut self, label: &str) -> usize
    pub fn connect(&mut self, from: usize, to: usize) -> ConnectionBuilder<'_>
    fn build(self) -> StepwiseModel
}

pub struct ConnectionBuilder<'a> {
    builder: &'a mut ScriptBuilder,
    transition_index: usize,
}

impl<'a> ConnectionBuilder<'a> {
    pub fn route(self, directions: Vec<Direction>) -> Self
}
```

**Auto-generation rule**: If `has_explicit_connections` is false when `build()` is called,
`ScriptBuilder` generates `sequence = [0, 1, ..., n-1]` and `transitions = [(0,1), (1,2), ..., (n-2, n-1)]`.

**Explicit connection rule**: If any `connect()` call was made, only the explicitly declared
transitions are used. The sequence is derived via topological sort of the declared connection graph.

**Panic rule**: `connect(a, b)` panics if `a >= steps.len()` or `b >= steps.len()` with a message
identifying the invalid index and the current step count.

### easing.rs — Easing Functions

A small pure-function module with no dependencies beyond `f32`.

```rust
pub mod easing {
    pub fn ease_in_out(t: f32) -> f32  // smoothstep: 3t² - 2t³
    pub fn ease_in(t: f32) -> f32      // t²
    pub fn ease_out(t: f32) -> f32     // 1 - (1-t)²
}
```

All functions clamp input to `[0.0, 1.0]` and guarantee output in `[0.0, 1.0]`.

### model.rs — Extended Step and Transition

Two fields are added to existing structs:

```rust
pub struct Step {
    pub label: String,
    pub content: Option<Box<dyn StepContent>>,  // new
}

pub struct Transition {
    pub from: usize,
    pub to: usize,
    pub route: Option<Vec<Direction>>,  // new
}

pub enum Direction { Up, Down, Left, Right }

pub trait StepContent: Send + Sync {
    fn create(&self, scene: &mut Scene, position: Vec3) -> TattvaId;
    fn update(&self, scene: &mut Scene, id: TattvaId, state: &StepState);
}
```

`Send + Sync` bounds on `StepContent` are required because the model is moved into an updater
closure (see `updater.rs` — `UpdaterFn = Arc<dyn Fn(...) + Send + Sync>`).

### view.rs — Extended StepView

`StepView` gains an optional content tattva id:

```rust
pub struct StepView {
    pub rect: TattvaId,
    pub label: TattvaId,
    pub content_id: Option<TattvaId>,  // new
}
```

`create_view` checks `step.content` and calls `content.create(scene, position)` when present,
storing the returned id. When `content` is `None`, only the default rect and label are created
(existing behavior, unchanged).

### render.rs — Extended StepwiseRenderer

`StepwiseRenderer::update` is extended with:

1. **Easing**: `t` from `Active { t }` and `Drawing { t }` is passed through `ease_in_out` before
   computing scale, opacity, and signal position.
2. **StepContent dispatch**: When `step.content` is `Some` and `step_view.content_id` is `Some`,
   calls `content.update(scene, content_id, step_state)`.
3. **Debug overlay** (opt-in): `StepwiseRenderer` gains a `debug: bool` field. When true, it
   maintains a set of `Label` tattvas (one per step plus one for the signal) and updates their
   text each frame.

Updated visual constants:

| State     | Opacity | Scale                          | Fill color (Vec4)              |
|-----------|---------|--------------------------------|--------------------------------|
| Pending   | 0.25    | 0.9                            | `(0.15, 0.15, 0.18, 1.0)`     |
| Active    | 1.0     | lerp(0.9, 1.15, ease_in_out(t))| `(0.25, 0.55, 0.95, 1.0)`     |
| Completed | 0.75    | 1.0                            | `(0.35, 0.40, 0.45, 1.0)`     |

Signal dot color: `(1.0, 0.82, 0.25, 1.0)` — warm yellow, contrasts with both the blue active
fill and the grey transition line.

Debug overlay format per step:
- Completed: `[✓] Label`
- Active:    `[→] Label (0.42)`
- Pending:   `[ ] Label`

Signal line (when present): `signal: 0 → 1 @ 0.67`

---

## Data Models

### StepwiseModel (updated)

```rust
pub struct StepwiseModel {
    pub steps: Vec<Step>,           // Step now has content: Option<Box<dyn StepContent>>
    pub transitions: Vec<Transition>, // Transition now has route: Option<Vec<Direction>>
    pub sequence: Vec<usize>,
}
```

### StepwiseView (updated)

```rust
pub struct StepwiseView {
    pub steps: Vec<StepView>,           // StepView now has content_id: Option<TattvaId>
    pub transitions: Vec<TransitionView>,
    pub signal: SignalView,
    pub debug_labels: Option<Vec<TattvaId>>,  // None when debug is off
}
```

### StepwiseRenderer (updated)

```rust
pub struct StepwiseRenderer {
    pub debug: bool,
}
```

Previously a zero-size unit struct; gains a `debug` field. Default is `debug: false`.

---

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a
system — essentially, a formal statement about what the system should do. Properties serve as the
bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Step index assignment is sequential

*For any* sequence of `s.step(label)` calls, the returned indices SHALL be `0, 1, 2, ..., n-1` in
order, and `model.steps[i].label` SHALL equal the label passed to the i-th `s.step()` call.

**Validates: Requirements 1.2**

---

### Property 2: Auto-generated model is a linear chain

*For any* n ≥ 1 steps added without explicit connections, the produced `StepwiseModel` SHALL have
`sequence == [0, 1, ..., n-1]`, `transitions.len() == n - 1`, and each transition `i` SHALL be
`{ from: i, to: i+1 }`.

**Validates: Requirements 1.3, 8.2, 8.3**

---

### Property 3: Sequence is a permutation of step indices

*For any* `StepwiseModel` produced by the Script_API (with or without explicit connections), the
`sequence` field SHALL contain every step index in `0..steps.len()` exactly once.

**Validates: Requirements 1.4**

---

### Property 4: Explicit connections suppress auto-generation

*For any* set of steps where at least one explicit `connect(a, b)` call is made, the produced
`StepwiseModel.transitions` SHALL contain exactly the explicitly declared transitions and no others.

**Validates: Requirements 2.2**

---

### Property 5: Route is stored on the transition

*For any* `Vec<Direction>` passed to `.route(...)` on a `ConnectionBuilder`, the corresponding
`Transition.route` SHALL equal `Some(directions)` after `build()`.

**Validates: Requirements 3.2**

---

### Property 6: Easing functions are range-preserving

*For any* `t` in `[0.0, 1.0]`, each of `ease_in_out(t)`, `ease_in(t)`, and `ease_out(t)` SHALL
return a value in `[0.0, 1.0]`.

**Validates: Requirements 6.2**

---

### Property 7: Active step scale uses eased interpolation

*For any* `t` in `[0.0, 1.0]`, when `StepwiseRenderer::update` processes a step in state
`Active { t }`, the scale applied to that step's rect tattva SHALL equal
`lerp(0.9, 1.15, ease_in_out(t))`.

**Validates: Requirements 6.4, 7.4**

---

### Property 8: Script_API output is functionally equivalent to manual construction

*For any* sequence of n labels, the `StepwiseModel` produced by the Script_API SHALL produce
identical `StepwiseState` output from `TimelineEngine::compute` as an equivalent manually
constructed model, for all sampled `p` values in `[0.0, 1.0]`.

**Validates: Requirements 8.1**

---

### Property 9: Debug overlay text matches state for all steps

*For any* `StepwiseState`, when the debug overlay is enabled, the overlay text for step `i` SHALL
match the format `[✓] label` for `Completed`, `[→] label (t)` for `Active { t }`, and `[ ] label`
for `Pending`.

**Validates: Requirements 5.1, 5.5**

---

## Error Handling

**Invalid connect index**: `ScriptBuilder::connect(a, b)` panics immediately with a message of the
form `"stepwise: connect() called with invalid step index {idx}; only {n} steps have been added"`.
This is a programmer error (misuse of the API), not a runtime error, so panic is appropriate.

**Empty model**: `stepwise(|_s| {})` returns a valid empty `StepwiseModel`. `TimelineEngine::compute`
already handles the empty case (returns empty `StepwiseState`). No special error handling needed.

**StepContent panics**: If a `StepContent` implementation panics inside `create` or `update`, the
panic propagates naturally. The library does not attempt to catch or recover from content panics.

**Easing out-of-range input**: All easing functions clamp their input to `[0.0, 1.0]` before
computing, so out-of-range `t` values (e.g., from floating-point drift) are handled gracefully.

---

## Testing Strategy

### Unit Tests

Unit tests cover specific examples and edge cases:

- `stepwise(|_s| {})` returns empty model (Requirement 1.5)
- `connect` with invalid index panics with descriptive message (Requirement 2.4)
- `ConnectionBuilder` without `.route()` leaves `transition.route == None` (Requirement 3.4)
- `Step` with `content = None` produces `StepView.content_id == None` (Requirement 4.4)
- Debug overlay disabled by default — no debug tattvas created (Requirement 5.3)
- `ease_in_out(0.0) == 0.0` and `ease_in_out(1.0) == 1.0` (Requirement 6.3)
- Pending step opacity == 0.25 (Requirement 7.1)
- Active step opacity == 1.0 (Requirement 7.2)
- Completed step opacity == 0.75 and scale == 1.0 (Requirement 7.3, 7.5)

### Property-Based Tests

Using the `proptest` crate (already common in Rust ecosystems). Each property test runs a minimum
of 100 iterations.

**Tag format**: `// Feature: stepwise-component, Property {N}: {property_text}`

- **Property 1** — Generate `Vec<String>` of labels (1–20 items), verify index assignment and label storage.
- **Property 2** — Generate n in 1..=20, add n steps without connect, verify linear chain invariants.
- **Property 3** — Generate n steps with random explicit connections forming a DAG, verify sequence is a permutation.
- **Property 4** — Generate n steps with k explicit connections (k ≥ 1), verify transitions == explicit only.
- **Property 5** — Generate `Vec<Direction>` (0–10 items), call `.route()`, verify stored correctly.
- **Property 6** — Generate `f32` in `[0.0, 1.0]`, verify all three easing functions return values in `[0.0, 1.0]`.
- **Property 7** — Generate `t: f32` in `[0.0, 1.0]`, verify scale == `lerp(0.9, 1.15, ease_in_out(t))` for `Active { t }`.
- **Property 8** — Generate `Vec<String>` labels and `f32` progress values, verify Script_API model == manual model under `TimelineEngine::compute`.
- **Property 9** — Generate random `StepwiseState`, verify debug overlay text format matches state for every step.

### Integration / Smoke Tests

- Verify `Direction` enum has exactly four variants (compile-time, enforced by exhaustive match in tests).
- Verify `StepContent` trait requires `Send + Sync` (compile-time, verified by attempting to use a non-Send impl in an updater closure — this will fail to compile).
- Visual smoke test: run the `stepwise` example and verify it renders without panic.
