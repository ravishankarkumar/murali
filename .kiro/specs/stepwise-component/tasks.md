# Implementation Plan: Stepwise Component (Phases 7–11)

## Overview

Incremental implementation of the Script API, StepContent trait, easing functions, debug overlay,
and visual polish on top of the existing `StepwiseModel → TimelineEngine → StepwiseState → StepwiseRenderer` pipeline.

## Tasks

- [x] 1. Create `easing.rs` with pure easing functions
  - Create `murali/src/frontend/collection/storytelling/stepwise/easing.rs`
  - Implement `ease_in_out(t: f32) -> f32` as smoothstep `3t² - 2t³`
  - Implement `ease_in(t: f32) -> f32` as `t²`
  - Implement `ease_out(t: f32) -> f32` as `1 - (1-t)²`
  - All functions must clamp input to `[0.0, 1.0]` before computing
  - _Requirements: 6.1, 6.2, 6.3_

  - [x] 1.1 Write property test for easing range preservation
    - **Property 6: Easing functions are range-preserving**
    - Generate `f32` values in `[0.0, 1.0]` via proptest; assert all three functions return values in `[0.0, 1.0]`
    - Tag: `// Feature: stepwise-component, Property 6: Easing functions are range-preserving`
    - **Validates: Requirements 6.2**

- [x] 2. Extend `model.rs` with `Direction`, `route`, `StepContent`, and `content`
  - Add `pub enum Direction { Up, Down, Left, Right }` to `model.rs`
  - Add `pub route: Option<Vec<Direction>>` field to `Transition`; default `None` for existing construction sites
  - Add `pub trait StepContent: Send + Sync` with methods `create` and `update` (import `Scene`, `Vec3`, `TattvaId`, `StepState`)
  - Add `pub content: Option<Box<dyn StepContent>>` field to `Step`; default `None` for existing construction sites
  - Update `stepwise.rs` example to add `content: None` and `route: None` to all existing `Step` and `Transition` literals
  - _Requirements: 3.3, 3.5, 4.1, 4.2, 4.6_

- [x] 3. Create `script.rs` with `ScriptBuilder`, `ConnectionBuilder`, and `stepwise()`
  - Create `murali/src/frontend/collection/storytelling/stepwise/script.rs`
  - Implement `pub fn stepwise<F: FnOnce(&mut ScriptBuilder)>(f: F) -> StepwiseModel`
  - Implement `ScriptBuilder` with fields `steps`, `explicit_connections`, `has_explicit_connections`
  - Implement `ScriptBuilder::step(&mut self, label: &str) -> usize`
  - Implement `ScriptBuilder::connect(&mut self, from: usize, to: usize) -> ConnectionBuilder<'_>`; panic with `"stepwise: connect() called with invalid step index {idx}; only {n} steps have been added"` on invalid index
  - Implement `ScriptBuilder::build(self) -> StepwiseModel`: auto-generate linear chain when no explicit connections; topological sort when explicit connections exist
  - Implement `ConnectionBuilder<'a>` with `pub fn route(self, directions: Vec<Direction>) -> Self`
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.4_

  - [x] 3.1 Write property test for sequential index assignment (Property 1)
    - **Property 1: Step index assignment is sequential**
    - Generate `Vec<String>` of 1–20 labels; verify returned indices are `0..n` and `model.steps[i].label` matches
    - Tag: `// Feature: stepwise-component, Property 1: Step index assignment is sequential`
    - **Validates: Requirements 1.2**

  - [x] 3.2 Write property test for auto-generated linear chain (Property 2)
    - **Property 2: Auto-generated model is a linear chain**
    - Generate `n` in `1..=20`; add n steps without connect; verify `sequence == [0..n]`, `transitions.len() == n-1`, each transition `i` is `{from: i, to: i+1}`
    - Tag: `// Feature: stepwise-component, Property 2: Auto-generated model is a linear chain`
    - **Validates: Requirements 1.3, 8.2, 8.3**

  - [x] 3.3 Write property test for sequence as permutation (Property 3)
    - **Property 3: Sequence is a permutation of step indices**
    - Generate n steps with random explicit connections forming a DAG; verify `sequence` contains every index in `0..n` exactly once
    - Tag: `// Feature: stepwise-component, Property 3: Sequence is a permutation of step indices`
    - **Validates: Requirements 1.4**

  - [x] 3.4 Write property test for explicit connections suppressing auto-generation (Property 4)
    - **Property 4: Explicit connections suppress auto-generation**
    - Generate n steps with k ≥ 1 explicit connections; verify `model.transitions` contains exactly those connections and no others
    - Tag: `// Feature: stepwise-component, Property 4: Explicit connections suppress auto-generation`
    - **Validates: Requirements 2.2**

  - [x] 3.5 Write property test for route storage (Property 5)
    - **Property 5: Route is stored on the transition**
    - Generate `Vec<Direction>` of 0–10 items; call `.route(directions.clone())`; verify `transition.route == Some(directions)`
    - Tag: `// Feature: stepwise-component, Property 5: Route is stored on the transition`
    - **Validates: Requirements 3.2**

  - [x] 3.6 Write property test for Script_API functional equivalence (Property 8)
    - **Property 8: Script_API output is functionally equivalent to manual construction**
    - Generate `Vec<String>` labels and sample `p` values in `[0.0, 1.0]`; compare `TimelineEngine::compute` output between Script_API model and manually constructed model
    - Tag: `// Feature: stepwise-component, Property 8: Script_API output is functionally equivalent to manual construction`
    - **Validates: Requirements 8.1**

- [x] 4. Checkpoint — ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 5. Extend `view.rs` with `content_id` on `StepView` and `debug_labels` on `StepwiseView`
  - Add `pub content_id: Option<TattvaId>` to `StepView`; set to `None` for steps without content
  - In `create_view`, check `step.content`; if `Some(content)`, call `content.create(scene, position)` and store the returned id; otherwise set `content_id: None`
  - Add `pub debug_labels: Option<Vec<TattvaId>>` to `StepwiseView`; set to `None` (debug label creation happens in renderer)
  - _Requirements: 4.3, 4.4_

- [x] 6. Update `render.rs` with easing, StepContent dispatch, updated colors, and debug overlay
  - Change `StepwiseRenderer` from a unit struct to `pub struct StepwiseRenderer { pub debug: bool }`
  - Import `crate::frontend::collection::storytelling::stepwise::easing`
  - Apply `ease_in_out(t)` to `t` before computing scale and opacity for `Active { t }` steps
  - Update visual constants: Pending → opacity `0.25`, scale `0.9`, fill `(0.15, 0.15, 0.18, 1.0)`; Active → opacity `1.0`, scale `lerp(0.9, 1.15, eased_t)`, fill `(0.25, 0.55, 0.95, 1.0)`; Completed → opacity `0.75`, scale `1.0`, fill `(0.35, 0.40, 0.45, 1.0)`
  - Apply `ease_in_out(t)` to signal position interpolation for `Drawing { t }` transitions
  - Add `StepContent::update` dispatch: when `step.content` is `Some` and `step_view.content_id` is `Some`, call `content.update(scene, content_id, step_state)`
  - Implement opt-in debug overlay: when `self.debug` is true, create/update `Label` tattvas (one per step + one for signal) with formatted state strings; when false, skip entirely
  - _Requirements: 4.5, 5.1, 5.2, 5.3, 5.4, 5.5, 6.4, 6.5, 7.1, 7.2, 7.3, 7.4, 7.5, 7.6_

  - [x] 6.1 Write property test for active step scale using eased interpolation (Property 7)
    - **Property 7: Active step scale uses eased interpolation**
    - Generate `t: f32` in `[0.0, 1.0]`; call `StepwiseRenderer::update` with a mock scene and `Active { t }` state; assert scale on rect tattva equals `lerp(0.9, 1.15, ease_in_out(t))`
    - Tag: `// Feature: stepwise-component, Property 7: Active step scale uses eased interpolation`
    - **Validates: Requirements 6.4, 7.4**

  - [x] 6.2 Write property test for debug overlay text format (Property 9)
    - **Property 9: Debug overlay text matches state for all steps**
    - Generate random `StepwiseState`; enable debug overlay; verify text for each step matches `[✓] label`, `[→] label (t)`, or `[ ] label` per state
    - Tag: `// Feature: stepwise-component, Property 9: Debug overlay text matches state for all steps`
    - **Validates: Requirements 5.1, 5.5**

- [x] 7. Update `mod.rs` to expose `script` and `easing` modules
  - Add `pub mod script;` and `pub mod easing;` to `murali/src/frontend/collection/storytelling/stepwise/mod.rs`
  - _Requirements: (all — makes new modules accessible)_

- [x] 8. Checkpoint — ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 9. Update `stepwise.rs` example to use the Script API
  - Rewrite `murali/examples/stepwise.rs` to use `stepwise(|s| { ... })` instead of manually constructing `StepwiseModel`
  - Use `ScriptBuilder::step` and `ScriptBuilder::connect` to reproduce the same five-step linear chain
  - Replace `StepwiseRenderer` unit struct usage with `StepwiseRenderer { debug: false }`
  - _Requirements: 1.1, 1.2, 1.3_

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Property tests use the `proptest` crate; add it to `murali/Cargo.toml` under `[dev-dependencies]` if not already present
- All `crate::` import paths follow the existing convention in the stepwise module
- `StepContent` implementations must be `Send + Sync` — enforced at compile time by the trait bound
