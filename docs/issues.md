# Stepwise — Known Issues & Future Improvements

This document tracks known issues, limitations, and future improvements for the Stepwise system.

⚠️ These are intentionally deferred to keep v1 simple and focused.

---

# 🧠 1. Timeline Engine

## 1.1 Transition Lookup Performance

### Problem

Current implementation may perform repeated sequence lookups.

### Impact

* O(n²) behavior in worst case
* Not an issue for small graphs, but will not scale

### Planned Fix

* Precompute `sequence_index_map: Vec<Option<usize>>`
* Use direct indexing instead of `.position()`

---

## 1.2 Transition Logic Assumptions

### Problem

Transitions assume:

```text
from → to are adjacent in sequence
```

### Impact

* Breaks for:

  * branching
  * non-linear flows
  * custom routing

### Planned Fix

* Explicit transition ordering
* Support non-linear graphs

---

## 1.3 Signal Follows Sequence (Not Transitions)

### Problem

Signal is derived from:

```text
sequence[i] → sequence[i+1]
```

Instead of actual transitions.

### Impact

* Incorrect behavior when:

  * custom transitions exist
  * graph != sequence

### Planned Fix

* Signal should follow **active transition**
* Not inferred from sequence

---

## 1.4 End State Handling

### Problem

At `progress = 1.0`:

* Last step may remain `Active { t: 1.0 }`

### Impact

* Visually incorrect final state

### Fix (Implemented)

* Force all steps/transitions → `Completed`
* Signal → `None`

---

## 1.5 Equal Time Distribution

### Problem

All steps get equal duration:

```text
segment_size = 1 / N
```

### Impact

* No control over pacing
* Some steps may need more time

### Planned Fix

* Weighted steps:

```rust
Step {
    weight: f32
}
```

---

# 🎨 2. Rendering Architecture

## 2.1 Immediate Mode Rendering (Incorrect)

### Problem

Initial design used:

```text
draw_rectangle(...)
```

### Impact

* Not compatible with Murali
* Would create objects every frame

### Resolution

* Move to **Tattva-based rendering**
* Create once, update per frame

---

## 2.2 Missing StepwiseView Layer

### Problem

No abstraction for:

```text
Step → Tattva mapping
```

### Impact

* Rendering logic becomes messy
* Hard to manage updates

### Planned Fix

Introduce:

```rust
struct StepwiseView {
    steps: Vec<StepView>,
    transitions: Vec<TransitionView>,
}
```

---

## 2.3 No Separation Between Create vs Update

### Problem

Creation and update logic are mixed

### Impact

* Hard to maintain
* Risk of duplicate objects

### Planned Fix

Split into:

```text
init() → create tattvas
update() → apply state
```

---

# 🧩 3. Step & Transition Behavior

## 3.1 Limited Step Representation

### Problem

Steps only support:

```rust
label: String
```

### Impact

* Cannot support:

  * diagrams
  * nested content
  * simulations

### Planned Fix

Introduce:

```rust
trait StepContent
```

---

## 3.2 Transition Rendering is Basic

### Problem

Only supports:

* straight lines

### Impact

* No expressive routing

### Planned Fix

* Reuse routing system from agentic_flow_chart
* Support:

  * custom paths
  * curves
  * orthogonal routing

---

## 3.3 No Styling System

### Problem

Visuals are hardcoded

### Impact

* No customization for:

  * colors
  * sizes
  * spacing

### Planned Fix

Introduce:

```rust
StepStyle
TransitionStyle
```

---

# 🧠 4. Architecture & API

## 4.1 Script API Not Implemented

### Problem

Users must manually define:

* model
* transitions
* sequence

### Planned Fix

Introduce:

```rust
stepwise(|s| {
    s.step("Observe");
    s.step("Reason");
});
```

---

## 4.2 No Separation Between Engine and Compiler

### Problem

Stepwise currently acts as:

* runtime engine

But should also support:

* timeline generation

### Planned Direction

Two modes:

```text
1. State-driven (current)
2. Timeline-generated (Murali-native)
```

---

## 4.3 Naming Consistency

### Problem

Legacy naming from:

* FlowChart
* AnimationEngine

### Planned Fix

Standardize:

```text
Flow → Stepwise
Node → Step
Edge → Transition
Animation → Timeline
Pulse → Signal
```

---

# 🚀 5. Future Enhancements

## 5.1 Weighted Steps

Control duration per step

---

## 5.2 Branching Support

```text
A → B → C
     ↘ D
```

---

## 5.3 Nested Stepwise

Step inside step

---

## 5.4 Interactive Controls

* pause
* scrub
* replay

---

## 5.5 Debug Overlay

Visual state debugging:

```text
[✓] Step 1
[→] Step 2 (0.42)
[ ] Step 3
```

---

# 🎯 Final Note

These issues are intentionally deferred.

Priority for v1:

```text
✔ Clear progression
✔ Clean architecture
✔ Working rendering
```

Everything else can be layered later.
