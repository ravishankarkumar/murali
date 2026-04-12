# 📘 Stepwise — Design Document (v1)

---

# 🧠 1. Purpose

**Stepwise** is a system for:

> **Explaining processes step-by-step through time-based visual progression**

It converts:

```text
sequence + progress → visual explanation
```

---

# 🎯 2. Design Goals

### ✅ Primary

* Script-first API (feels like writing a story)
* Clear progression (no hidden magic)
* Deterministic behavior
* Easy to debug

---

### ✅ Secondary

* Full customizability (optional)
* Extensible components (Tattva support)
* Clean separation of concerns

---

### ❌ Non-goals (v1)

* Complex graph layouts
* AI-driven layouts
* Physics-based animations

---

# 🏗️ 3. Core Architecture

```text
User Script
    ↓
Model (Steps + Transitions + Sequence)
    ↓
Timeline Engine (Stepwise)
    ↓
StepwiseState (single source of truth)
    ↓
Components render themselves
    ↓
Renderer (draws primitives)
```

---

# 🧩 4. Core Concepts

---

## 4.1 Step

Represents a unit of explanation.

```rust
struct Step {
    label: String,
    content: Option<Box<dyn StepContent>>,
}
```

---

## 4.2 Transition

Represents movement between steps.

```rust
struct Transition {
    from: usize,
    to: usize,
    route: Option<Vec<Direction>>,
}
```

---

## 4.3 Sequence

Defines order of progression.

```rust
type Sequence = Vec<usize>;
```

---

## 4.4 Progress

```rust
progress: f32 // 0.0 → 1.0
```

---

# 🧠 5. Timeline Engine (Stepwise)

---

## Responsibility

> Convert global progress → StepwiseState

---

## Input

* steps
* transitions
* sequence
* progress

---

## Output

```rust
struct StepwiseState {
    steps: Vec<StepState>,
    transitions: Vec<TransitionState>,
    signal: Option<SignalState>,
}
```

---

# 🧾 6. State Definitions

---

## 6.1 StepState

```rust
enum StepState {
    Pending,
    Active { t: f32 },
    Completed,
}
```

---

## 6.2 TransitionState

```rust
enum TransitionState {
    Hidden,
    Drawing { t: f32 },
    Completed,
}
```

---

## 6.3 SignalState

```rust
struct SignalState {
    from: usize,
    to: usize,
    t: f32,
}
```

---

# 🎨 7. Rendering Model

---

## Principle

> Components read state and render themselves

---

## StepContent Trait

```rust
trait StepContent {
    fn render(&self, ctx: &mut Context, state: &StepState);
}
```

---

## Transition Renderer

```rust
trait TransitionRenderer {
    fn render(&self, ctx: &mut Context, state: &TransitionState);
}
```

---

## Signal Renderer

```rust
trait SignalRenderer {
    fn render(&self, ctx: &mut Context, state: &SignalState);
}
```

---

# 🔥 8. Data Flow (CRITICAL)

```text
Stepwise → produces StepwiseState
Everything else → reads StepwiseState
```

---

## Rules

* No component modifies time
* No component controls execution
* No callbacks between components

---

# 🎬 9. Script API (Public Interface)

---

## Basic

```rust
stepwise(|s| {
    s.step("Observe");
    s.step("Reason");
    s.step("Plan");
    s.step("Act");
    s.step("Reflect");
});
```

---

## Intermediate

```rust
stepwise(|s| {
    let a = s.step("Observe");
    let b = s.step("Reason");

    s.connect(a, b);
});
```

---

## Advanced

```rust
stepwise(|s| {
    let a = s.step("Observe");
    let b = s.step("Reason");

    s.connect(a, b)
     .route([Up, Right, Down]);
});
```

---

# ⚙️ 10. Layout System (v1)

---

## Responsibility

```text
Steps → positions
```

---

## Strategy (v1)

* Linear horizontal layout
* Equal spacing
* No complex routing

---

# 🧭 11. Routing System (v1)

---

## Default

```text
straight line
```

---

## Optional

```rust
.route([Up, Right, Down])
```

---

# 🧠 12. Signal System

---

Represents active progression.

* moves along transitions
* derived from timeline state
* rendered independently

---

# 🚀 13. Extensibility (Tattva Support)

---

## StepContent enables:

* text
* diagrams
* simulations
* nested Stepwise

---

Example:

```rust
step("Neural Network")
    .with_content(NeuralNetViz)
```

---

# ⚠️ 14. Key Constraints

---

## Single Source of Truth

```text
StepwiseState
```

---

## No Hidden Logic

All behavior must be derivable from state.

---

## Determinism

Same input → same output

---

# 🧪 15. Debug Strategy

---

Expose debug view:

```text
Step 0: Completed
Step 1: Active (0.42)
Step 2: Pending
```

---

# 🎯 16. Success Criteria

---

* User can understand progression instantly
* Minimal API surface
* Easy to extend
* Easy to debug

---

# 💥 17. Summary

> Stepwise is a timeline-driven system where:
>
> * Stepwise defines **when**
> * Components define **how**
> * State defines **everything**
