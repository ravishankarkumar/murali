---
sidebar_position: 7
---

# Storytelling

Storytelling tattvas live under `murali::frontend::collection::storytelling`. They are higher-level constructs for building step-by-step explanations and narrative animations.

## Stepwise

`Stepwise` renders a flow of nodes that reveal progressively — useful for algorithm walkthroughs, decision trees, and step-by-step proofs.

It is a tattva: it implements `Project` and `Bounded`, so it's added to the scene like any other shape.

```rust
use murali::frontend::collection::storytelling::stepwise::Stepwise;
use murali::frontend::collection::storytelling::stepwise::model::{StepwiseModel, StepNode};

let model = StepwiseModel::new(vec![
    StepNode::new("Step 1", "Initialize"),
    StepNode::new("Step 2", "Process"),
    StepNode::new("Step 3", "Output"),
]);

let id = scene.add_tattva(
    Stepwise::new(model)
        .with_progress(0.0),  // 0.0 = nothing revealed, 1.0 = fully revealed
    Vec3::ZERO,
);
```

### Animating progress

Drive the reveal with a timeline animation or an updater:

```rust
// Via timeline
timeline
    .animate(id)
    .at(0.0)
    .for_duration(3.0)
    // use a custom signal or call_during to update progress
    .spawn();

// Via updater
scene.add_updater(id, |scene, id, _dt| {
    if let Some(t) = scene.get_tattva_typed_mut::<Stepwise>(id) {
        t.state.progress = (scene.scene_time / 3.0).clamp(0.0, 1.0);
        t.mark_dirty(DirtyFlags::GEOMETRY);
    }
});
```

### Signal progress

`with_signal_progress` controls a secondary animation within each node (e.g. a signal traveling along a connection):

```rust
Stepwise::new(model)
    .with_progress(1.0)
    .with_signal_progress(0.5)
```
