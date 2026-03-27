# Murali Concrete Implementation Plan

## Purpose

This is the working execution plan for the next phase of Murali.

It is ordered by practical product impact, not by historical milestone number.

The top priorities are:

1. fix text quality and correctness
2. make video export fast enough to be usable
3. add missing AI-teaching motion primitives
4. improve reliability and authoring ergonomics
5. continue toward stronger Manim parity

---

## Phase 1: Text System Repair

### Why this is first

Text is still the most visible polish-sensitive part of the product.

Murali is no longer in the state where text is completely broken:

- regular label text is meaningfully improved
- LaTeX renders and is usable
- Typst renders and is usable
- `murali doctor` and [text-setup.md](/Users/ravishankar/personal-work/murali/docs/text-setup.md) now exist

But text is still not fully finished.

If labels, equations, token sequences, and rendered text blocks look irregular, then:

- STEM scenes look untrustworthy
- AI teaching scenes look unfinished
- layout bounds become unreliable
- export quality is compromised even if rendering is otherwise correct

### Goal

Make regular text, LaTeX, and Typst visually stable, layout-aware, and predictable.

### Problems to solve

- irregular glyph sizing
- uneven baseline and spacing
- weak font consistency
- inaccurate text bounds
- mismatch between preview and export assumptions
- poor multi-object text alignment
- rendered text blocks still have approximate frontend bounds
- LaTeX and Typst need better integration with authored layout helpers

### Concrete work

#### 1. Audit and stabilize the label pipeline

Files to focus on:

- [font.rs](/Users/ravishankar/personal-work/murali/src/resource/text/font.rs)
- [layout.rs](/Users/ravishankar/personal-work/murali/src/resource/text/layout.rs)
- [atlas.rs](/Users/ravishankar/personal-work/murali/src/resource/text/atlas.rs)
- [mesh.rs](/Users/ravishankar/personal-work/murali/src/resource/text/mesh.rs)
- [sync.rs](/Users/ravishankar/personal-work/murali/src/backend/sync.rs)

Tasks:

- verify the coordinate system from font units -> atlas -> world mesh
- fix cap-height / baseline assumptions
- ensure glyph advance and bearing are handled consistently
- stop relying on fragile approximations where real font metrics should be used

#### 2. Make text bounds trustworthy across all text modes

Files to focus on:

- [label.rs](/Users/ravishankar/personal-work/murali/src/frontend/collection/text/label.rs)
- [latex.rs](/Users/ravishankar/personal-work/murali/src/frontend/collection/text/latex.rs)
- [typst.rs](/Users/ravishankar/personal-work/murali/src/frontend/collection/text/typst.rs)
- [equation.rs](/Users/ravishankar/personal-work/murali/src/frontend/collection/math/equation.rs)
- [matrix.rs](/Users/ravishankar/personal-work/murali/src/frontend/collection/math/matrix.rs)

Tasks:

- ensure `measure_label()` and actual rendered bounds match closely
- verify that label centering is true visual centering, not just arithmetic centering
- ensure matrix/equation layout uses the same metric model as rendering
- add better estimated bounds for `Latex` and `Typst`
- make `next_to`, stacks, and `align_to` work more reliably with rendered text blocks

#### 3. Unify text quality modes

Goal:

- preview should be reasonably fast
- export should be higher fidelity

Tasks:

- connect text resolution to render config more cleanly
- define preview text settings and export text settings explicitly
- avoid layout changing when raster resolution changes
- make LaTeX and Typst follow the same preview/export quality model where possible

#### 4. Build text regression scenes

Add canonical scenes for:

- dense axis labels
- token sequences with mixed word lengths
- equations with color styling
- matrices with multiple column widths
- side-by-side label alignment
- side-by-side regular text / LaTeX / Typst comparison
- a mixed text + math block scene

#### 5. Keep text setup and diagnostics current

Files to focus on:

- [text-setup.md](/Users/ravishankar/personal-work/murali/docs/text-setup.md)
- [doctor.rs](/Users/ravishankar/personal-work/murali/src/engine/doctor.rs)

Tasks:

- keep LaTeX and Typst setup guidance current
- extend diagnostics when new external/runtime dependencies are introduced
- keep examples aligned with the documented workflow

### Exit criteria

- text spacing is visually stable
- text bounds are believable enough for layout
- matrix/equation text looks consistent
- LaTeX and Typst are layout-usable, not just render-usable
- preview and export use the same layout with different raster fidelity

---

## Phase 2: Efficient Video Rendering

### Why this is second

The tool is not practical for production if one second of output takes many minutes.

### Goal

Make export deterministic and dramatically faster.

### Current issues

- export runtime is not fully working yet
- frame capture path stalls on macOS
- export currently depends on a slow per-frame render/readback path
- text and resource work may be repeated too often

### Concrete work

#### 1. Fix export runtime correctness first

Files to focus on:

- [export.rs](/Users/ravishankar/personal-work/murali/src/engine/export.rs)
- [renderer.rs](/Users/ravishankar/personal-work/murali/src/backend/renderer/renderer.rs)
- [device.rs](/Users/ravishankar/personal-work/murali/src/backend/renderer/device.rs)

Tasks:

- fix the macOS readback stall
- verify PNG sequence output
- verify ffmpeg assembly
- verify fixed-timestep export determinism

#### 2. Profile where export time is going

Measure:

- scene update time
- sync time
- text baking time
- GPU render time
- readback time
- image encoding time

We should identify whether the bottleneck is:

- renderer readback
- PNG writing
- repeated text/material rebuild
- too much ECS churn

#### 3. Avoid unnecessary rebuild work during export

Likely targets:

- cache text meshes more aggressively
- avoid rebaking static text every frame
- avoid respawning render entities when only transform/opacity changes
- ensure export path reuses exactly the same cached resources as preview where possible

#### 4. Improve output pipeline

Tasks:

- support raw frame dumps or faster image encoding options later
- consider piping frames directly to ffmpeg after PNG correctness is stable
- add export settings for fps, resolution, and quality presets

### Exit criteria

- PNG sequence export is reliable
- MP4 export is reliable
- one-second scenes render in a reasonable development-time budget
- export cost scales mostly with scene complexity, not accidental rebuilds

---

## Phase 3: Neural Network Flow Animation

### Why this is next

This is important for aiunderthehood.com and makes Murali more than a static diagram tool.

### Goal

Show information moving through a network in a way that is visually clear and reusable.

### What feature to add

We need a semantic primitive for animated signal propagation through edges.

In Manim terms, this is closest in spirit to a passing highlight / signal-travel effect.
The exact Murali feature does not need to copy the name. A better Murali name would be:

- `SignalFlow`
- `Propagate`
- `FlowAlongEdges`

### First implementation target

For `NeuralNetworkDiagram`, support:

- highlight a path or set of edges
- animate a dot or pulse moving layer-to-layer
- optionally leave a fading trail
- optionally tint activated nodes

### Concrete work

Files to focus on:

- [neural_network_diagram.rs](/Users/ravishankar/personal-work/murali/src/frontend/collection/ai/neural_network_diagram.rs)
- [animation/mod.rs](/Users/ravishankar/personal-work/murali/src/frontend/animation/mod.rs)
- [animation/builder.rs](/Users/ravishankar/personal-work/murali/src/frontend/animation/builder.rs)

Tasks:

- expose semantic edge geometry from `NeuralNetworkDiagram`
- add an animation that moves an indicator along selected edges
- add reusable path sequencing across layers
- support multiple simultaneous signals
- build one canonical AI example around it

Current state:

- the first reusable version now exists via `SignalFlow`
- `NeuralNetworkDiagram` exposes semantic path geometry
- animation builder support exists through `.propagate()` / `.propagate_to(...)`
- [neural_signal_flow.rs](/Users/ravishankar/personal-work/murali/examples/neural_signal_flow.rs) is the first focused regression/demo scene

Next target:

- multiple simultaneous signals
- stronger edge/node activation styling
- helpers that derive common layer-to-layer routes automatically
- better integration with AI-specific authored scenes and templates

### Exit criteria

- a dot/pulse can travel through a network path
- the API is scene-author-friendly
- the effect is deterministic and export-safe

---

## Phase 4: Reliability And Authoring Polish

### Goal

Make Murali feel dependable to use repeatedly.

### Concrete work

#### 1. Public API cleanup

Tasks:

- reduce `add_tattva` boilerplate in examples
- add scene helpers for common creation patterns
- standardize animation builder naming
- make theme application easier at scene level

#### 2. Better examples as regression assets

Canonical examples should cover:

- layout
- animation
- STEM math
- AI teaching
- export
- semantic animation
- text stress cases

#### 3. Diagnostics

Continue improving `murali doctor` and related diagnostics for:

- font/resource availability
- latex toolchain availability
- ffmpeg availability
- GPU/export support status

Current state:

- `murali doctor` already exists

Next target:

- make diagnostics more specific and more actionable as export and text systems harden

#### 4. Docs cleanup

Keep only:

- current architecture docs
- export docs
- implementation status/plan docs
- example docs

### Exit criteria

- examples double as smoke tests
- environment issues are easier to diagnose
- authoring APIs are less repetitive

---

## Phase 5: Stronger Manim Parity

### Goal

Move from “usable” toward “compelling” parity.

### Priority order

#### 1. Better creation and disappearance animations

Add:

- `fade_in`
- `fade_out`
- stroke-based create/reveal where applicable
- better uncreate/remove semantics

#### 2. Better morphing

Current state:

- semantic crossfade + transform matching

Next target:

- topology-aware shape morphing
- text-to-shape and shape-to-text transitions

#### 3. Better equation continuity

Next target:

- richer symbolic tokenization
- superscript/subscript awareness
- better continuity for algebra steps

#### 4. Better matrix pedagogy

Add:

- row/column operations
- per-cell emphasis
- transformation-step presets

#### 5. Hierarchy formalization

Need a firmer model for:

- parent-child transforms
- group-local animation
- nested layouts

### Exit criteria

- common Manim-style teaching scenes map naturally to Murali
- semantic animation feels intentional rather than improvised

---

## Phase 6: Post-Parity Growth

These are important, but should come after text/export/reliability are solid.

- 3D function surfaces
- glTF import
- materials and lighting polish
- branding / logo animation
- tutorial content and onboarding docs

---

## Recommended execution order for the next coding sessions

### Session block A

Text repair only.

- audit font metrics
- fix text mesh generation
- fix label bounds
- add text regression scenes

### Session block B

Export correctness and speed.

- fix export stall
- verify PNG/MP4
- measure export bottlenecks
- reduce repeated rebuild work

### Session block C

Neural-network signal propagation.

- semantic edge path representation
- animated moving signal
- AI teaching demo scene

### Session block D

Reliability polish.

- API cleanup
- diagnostics
- example cleanup

### Session block E

Deeper parity.

- better create/fade helpers
- better morphing
- better equation/matrix transformations

---

## Bottom line

The next practical roadmap should be:

1. text quality
2. export speed and correctness
3. neural-network flow animation
4. reliability and authoring polish
5. deeper Manim parity

That order matches both product reality and aiunderthehood.com needs.
