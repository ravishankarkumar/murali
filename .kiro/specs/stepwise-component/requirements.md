# Requirements Document

## Introduction

This document covers the completion of the Stepwise component in the Murali animation library.
Phases 1–6 (model, layout, timeline engine, rendering, transitions, signal) are already implemented.
This spec covers Phases 7–11: the Script API, ConnectionBuilder, StepContent trait, debug overlay,
and polish (easing, colors, animation smoothing).

The Stepwise component converts a user-authored sequence of labeled steps into a time-driven visual
animation. The goal is a script-first API that hides internal complexity while remaining fully
extensible and debuggable.

## Glossary

- **Script_API**: The `stepwise(|s| { ... })` closure-based builder that is the primary public interface.
- **ScriptBuilder**: The struct passed into the Script_API closure; accumulates steps and connections.
- **ConnectionBuilder**: The fluent builder returned by `ScriptBuilder::connect(a, b)` that allows optional route customization.
- **StepwiseModel**: The internal data model containing steps, transitions, and sequence.
- **StepContent**: A trait that allows arbitrary Tattva-based content to be embedded inside a step node.
- **Direction**: An enum (`Up`, `Down`, `Left`, `Right`) used to specify waypoints in a custom route.
- **Route**: An ordered `Vec<Direction>` that defines the path a transition edge follows.
- **Debug_Overlay**: A text-based overlay rendered on top of the animation showing per-step state.
- **TimelineEngine**: The existing engine that maps `progress: f32` to `StepwiseState`.
- **StepwiseState**: The single source of truth for all rendering decisions.
- **StepwiseRenderer**: The existing renderer that reads `StepwiseState` and updates scene tattvas.
- **Easing_Function**: A mathematical function that maps linear `t` to a non-linear output for smoother animation.
- **Tattva**: A scene node in the Murali engine, identified by `TattvaId`.
- **Scene**: The Murali engine scene that owns all tattvas and updaters.

---

## Requirements

### Requirement 1: Script API — Closure Builder

**User Story:** As an animator, I want to describe a stepwise animation by listing steps inside a closure, so that I do not need to manually construct `StepwiseModel`, `sequence`, or `Transition` vectors.

#### Acceptance Criteria

1. THE Script_API SHALL accept a closure of the form `stepwise(|s| { ... })` and return a fully constructed `StepwiseModel`.
2. WHEN `s.step("Label")` is called inside the closure, THE ScriptBuilder SHALL append a new `Step` with the given label and return a `usize` index identifying that step.
3. WHEN steps are added without explicit `connect` calls, THE ScriptBuilder SHALL auto-generate a linear sequence `[0, 1, 2, ..., n-1]` and corresponding `Transition` entries for each adjacent pair.
4. THE Script_API SHALL produce a `StepwiseModel` whose `sequence` contains every step index exactly once.
5. IF the closure adds zero steps, THEN THE Script_API SHALL return a `StepwiseModel` with empty `steps`, `transitions`, and `sequence`.

---

### Requirement 2: Script API — Explicit Connections

**User Story:** As an animator, I want to explicitly connect two steps, so that I can control which transitions exist without relying on auto-generation.

#### Acceptance Criteria

1. WHEN `s.connect(a, b)` is called, THE ScriptBuilder SHALL record a directed transition from step `a` to step `b`.
2. WHEN at least one explicit `connect` call is made, THE ScriptBuilder SHALL use only the explicitly declared transitions and SHALL NOT auto-generate transitions.
3. WHEN at least one explicit `connect` call is made, THE ScriptBuilder SHALL derive the `sequence` from the declared connections in topological order.
4. IF `s.connect(a, b)` is called with an index that does not correspond to a step added in the same closure, THEN THE ScriptBuilder SHALL panic with a descriptive message identifying the invalid index.

---

### Requirement 3: ConnectionBuilder — Fluent Route Customization

**User Story:** As an animator, I want to attach a custom waypoint route to a connection, so that the transition edge follows a non-straight path.

#### Acceptance Criteria

1. WHEN `s.connect(a, b)` is called, THE ScriptBuilder SHALL return a `ConnectionBuilder` that holds a mutable reference to the transition being built.
2. WHEN `.route([Up, Right, Down])` is called on a `ConnectionBuilder`, THE ConnectionBuilder SHALL store the provided `Vec<Direction>` on the corresponding `Transition`.
3. THE `Transition` struct SHALL contain an `Option<Vec<Direction>>` field named `route`; a value of `None` indicates a straight-line path.
4. WHEN no `.route(...)` call is made on a `ConnectionBuilder`, THE ConnectionBuilder SHALL leave the `route` field as `None`.
5. THE `Direction` enum SHALL define exactly four variants: `Up`, `Down`, `Left`, `Right`.

---

### Requirement 4: StepContent Trait — Extensible Node Content

**User Story:** As an animator, I want to embed custom visual content inside a step node, so that steps can display diagrams, nested animations, or other Tattva-based visuals.

#### Acceptance Criteria

1. THE `StepContent` trait SHALL declare a method `fn create(&self, scene: &mut Scene, position: Vec3) -> TattvaId` that creates the content tattva and returns its id.
2. THE `StepContent` trait SHALL declare a method `fn update(&self, scene: &mut Scene, id: TattvaId, state: &StepState)` that updates the tattva in response to state changes.
3. WHEN a `Step` has a `content` field set to `Some(Box<dyn StepContent>)`, THE `create_view` function SHALL call `content.create(scene, position)` and store the returned `TattvaId` in the corresponding `StepView`.
4. WHEN a `Step` has a `content` field of `None`, THE `create_view` function SHALL render only the default rectangle and label for that step.
5. WHEN `StepwiseRenderer::update` processes a step with custom content, THE StepwiseRenderer SHALL call `content.update(scene, id, step_state)` for that step.
6. THE `StepContent` trait SHALL require `Send + Sync` bounds so that implementations can be used inside updater closures.

---

### Requirement 5: Debug Overlay

**User Story:** As an animator, I want to enable a debug overlay that shows the current state of every step, so that I can verify the timeline engine output matches the visual output.

#### Acceptance Criteria

1. THE Debug_Overlay SHALL display one line per step in the format `[✓] Label`, `[→] Label (t)`, or `[ ] Label` for `Completed`, `Active { t }`, and `Pending` states respectively.
2. WHEN the debug overlay is enabled, THE Debug_Overlay SHALL update every frame to reflect the current `StepwiseState`.
3. THE Debug_Overlay SHALL be opt-in; WHEN no debug flag is set, THE StepwiseRenderer SHALL not create or update any debug overlay tattvas.
4. THE Debug_Overlay SHALL be positioned in a fixed screen-space region that does not overlap the main step visualization.
5. WHEN `StepwiseState::signal` is `Some`, THE Debug_Overlay SHALL display the signal's `from`, `to`, and `t` values on a separate line.

---

### Requirement 6: Easing Functions

**User Story:** As an animator, I want step and transition animations to use easing curves, so that motion feels smooth and natural rather than linear.

#### Acceptance Criteria

1. THE Easing_Function module SHALL provide at least the following functions: `ease_in_out(t: f32) -> f32`, `ease_in(t: f32) -> f32`, `ease_out(t: f32) -> f32`.
2. FOR ALL valid inputs `t` in `[0.0, 1.0]`, each Easing_Function SHALL return a value in `[0.0, 1.0]`.
3. FOR ALL valid inputs `t` in `[0.0, 1.0]`, `ease_in_out(0.0)` SHALL equal `0.0` and `ease_in_out(1.0)` SHALL equal `1.0` (boundary preservation).
4. WHEN `StepwiseRenderer::update` computes visual properties for an `Active { t }` step, THE StepwiseRenderer SHALL apply `ease_in_out` to `t` before computing scale and opacity.
5. WHEN `StepwiseRenderer::update` computes the signal position for a `Drawing { t }` transition, THE StepwiseRenderer SHALL apply `ease_in_out` to `t` before interpolating position.

---

### Requirement 7: Visual Polish — Colors and Scale

**User Story:** As an animator, I want the default step colors and scale transitions to be visually distinct and aesthetically consistent, so that the animation communicates progression clearly without custom configuration.

#### Acceptance Criteria

1. THE StepwiseRenderer SHALL render `Pending` steps with an opacity of `0.25` and a neutral dark fill color.
2. THE StepwiseRenderer SHALL render `Active` steps with an opacity of `1.0` and an accent fill color distinct from the pending and completed colors.
3. THE StepwiseRenderer SHALL render `Completed` steps with an opacity of `0.75` and a muted fill color that is visually lighter than the active color.
4. WHEN a step transitions from `Pending` to `Active`, THE StepwiseRenderer SHALL interpolate scale from `0.9` to `1.15` using the eased `t` value.
5. WHEN a step transitions from `Active` to `Completed`, THE StepwiseRenderer SHALL set scale to `1.0` and opacity to `0.75`.
6. THE signal dot SHALL use a color that contrasts with both the step fill and the transition line color.

---

### Requirement 8: Round-Trip Model Consistency

**User Story:** As a developer, I want the Script_API output to be equivalent to a manually constructed `StepwiseModel`, so that both authoring paths produce identical animation behavior.

#### Acceptance Criteria

1. FOR ALL sequences of `s.step(label)` calls with no explicit connections, THE Script_API SHALL produce a `StepwiseModel` such that `TimelineEngine::compute(model, p)` returns the same `StepwiseState` as computing on an equivalent manually constructed model for all `p` in `[0.0, 1.0]`.
2. THE `StepwiseModel` produced by the Script_API SHALL have `transitions.len() == steps.len() - 1` when no explicit connections are provided and `steps.len() >= 1`.
3. THE `StepwiseModel` produced by the Script_API SHALL have `sequence.len() == steps.len()`.
