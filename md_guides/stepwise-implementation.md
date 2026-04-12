# 🛠️ Stepwise — Implementation Plan (v1)

---

# 🧠 Guiding Principle

```text
Build small → test visually → iterate
```

At every step:

* system should compile ✅
* something should render ✅
* behavior should be understandable ✅

---

# 🧱 Phase 0 — Setup (Very Small)

## Goal

Create a clean space without touching old system.

---

### Tasks

* Create new module:

```text
stepwise/
  mod.rs
  model.rs
  timeline.rs
  render.rs
  script.rs
```

---

### Test

* Project compiles
* Nothing breaks

---

# 🟩 Phase 1 — Minimal Model (Steps only)

## Goal

Represent steps + sequence

---

### Implement

```rust
struct Step {
    label: String,
}

struct StepwiseModel {
    steps: Vec<Step>,
    sequence: Vec<usize>,
}
```

---

### ScriptBuilder (basic)

```rust
s.step("Observe");
s.step("Reason");
```

Auto-build:

```text
[0 → 1 → 2]
```

---

### Test

Render static text:

```text
Observe   Reason   Plan
```

👉 No animation yet

---

# 🟨 Phase 2 — Layout (very simple)

## Goal

Position steps

---

### Implement

```rust
struct StepLayout {
    position: Vec3,
}
```

---

### Strategy

* horizontal layout
* fixed spacing

---

### Test

* Steps appear spaced evenly
* Labels visible

---

# 🟥 Phase 3 — TimelineEngine (CORE)

## Goal

Replace your current animation logic

---

### Implement

```rust
enum StepState {
    Pending,
    Active { t: f32 },
    Completed,
}
```

---

### TimelineEngine::compute()

Input:

```rust
progress: f32
sequence: Vec<usize>
```

---

### Logic (simple first)

For N steps:

```text
segment_size = 1.0 / N
```

---

### Compute:

```text
which step is active
local t inside step
```

---

### Output:

```rust
StepwiseState {
    steps: Vec<StepState>
}
```

---

### Test (CRITICAL)

Print:

```text
progress = 0.3

Step 0 → Completed
Step 1 → Active (0.2)
Step 2 → Pending
```

👉 If this works → system is unlocked

---

# 🟪 Phase 4 — Render based on state

## Goal

Visual clarity of progression

---

### Implement

```rust
match state {
    Pending → dim
    Active → highlight
    Completed → full
}
```

---

### Test

At different progress:

* only 1 step active
* previous steps completed
* future steps dim

👉 MUST feel obvious

---

# 🟦 Phase 5 — Transitions (simple lines)

## Goal

Add edges between steps

---

### Implement

```rust
struct Transition {
    from: usize,
    to: usize,
}
```

---

### Add state

```rust
enum TransitionState {
    Hidden,
    Drawing { t: f32 },
    Completed,
}
```

---

### Timeline logic

* transition happens between steps
* use same segmentation logic

---

### Test

* line appears gradually between steps
* matches active step progression

---

# 🟧 Phase 6 — Signal (indicator / pulse)

## Goal

Make progression visible

---

### Implement

```rust
struct SignalState {
    from: usize,
    to: usize,
    t: f32,
}
```

---

### Render

* small dot moving along transition

---

### Test

* signal moves smoothly
* matches transition progress

---

# 🟫 Phase 7 — Script API (IMPORTANT)

## Goal

Hide complexity

---

### Implement

```rust
stepwise(|s| {
    s.step("Observe");
    s.step("Reason");
});
```

---

### Internally builds:

* model
* sequence
* transitions

---

### Test

User writes ONLY steps
Everything else auto-generated

---

# 🟨 Phase 8 — ConnectionBuilder (advanced control)

## Goal

Enable custom routing

---

### Implement

```rust
s.connect(a, b)
 .route([Up, Right, Down]);
```

---

### Internally:

* store route steps
* reuse routing logic later

---

### Test

* custom edge path renders correctly

---

# 🟩 Phase 9 — StepContent (Tattva support)

## Goal

Enable complex nodes

---

### Trait

```rust
trait StepContent {
    fn render(&self, ctx, state: &StepState);
}
```

---

### Test

* simple custom content inside step
* reacts to Active state

---

# 🟥 Phase 10 — Debug Overlay (HIGH VALUE)

## Goal

Remove confusion permanently

---

### Show:

```text
[✓] Observe
[→] Reason (0.4)
[ ] Plan
```

---

### Test

* always matches visual state

---

# 🚀 Phase 11 — Polish

---

### Add:

* easing functions
* better colors
* animation smoothing

---

# 🎯 Final Testing Checklist

---

## Must pass:

### ✅ Clarity

User instantly knows:

* current step
* progress

---

### ✅ Determinism

Same input → same animation

---

### ✅ Simplicity

User writes:

```rust
step("A")
step("B")
```

---

### ✅ Extensibility

Can add:

* custom step content
* custom routing

---

# 🧠 Development Strategy

---

## ❗ DO NOT:

* rewrite everything at once
* copy old architecture
* over-engineer early

---

## ✅ DO:

* build from scratch
* reuse only small utilities
* validate visually every phase

---

# 💥 Milestone Plan

---

## Day 1

* Phase 1–3 (model + timeline)
  👉 print state working

---

## Day 2

* Phase 4–6 (render + transitions + signal)
  👉 visible animation

---

## Day 3

* Phase 7–9 (API + extensibility)
  👉 usable system

---

# 🎯 Final Thought

> If Phase 3 (timeline) is clean, everything else becomes easy.
