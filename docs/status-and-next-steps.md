# Murali Status And Next Steps

## What is done

### Text system status

Partially completed and significantly improved.

Done:

- regular label text is meaningfully improved
- LaTeX renders end-to-end
- Typst renders end-to-end
- built-in diagnostics exist through `cargo run -- doctor`
- text setup docs exist in [text-setup.md](/Users/ravishankar/personal-work/murali/docs/text-setup.md)
- dedicated LaTeX and Typst showcase examples exist

Not done yet:

- `Latex` and `Typst` bounds are still approximate at the frontend object level
- layout helpers do not yet treat rendered text blocks as reliably as plain labels
- text regression coverage still needs to be expanded
- preview/export quality policy across all text modes still needs to be unified

### Milestone 1: Architectural cleanup

Completed.

- projection owns CPU-side mesh data
- typed dirty flags exist
- transform-only fast path exists
- startup/runtime stability was improved

### Milestone 2: Layout and authored scene ergonomics

Completed.

- bounds and anchors
- `next_to`, `align_to`, `to_edge`
- `Group`, `HStack`, `VStack`
- improved `Axes`
- `NumberPlane`

### Milestone 3: Core animation parity

Completed.

- deterministic timeline hardening
- move/rotate/scale/fade/create
- follow hooks
- camera animation cleanup

### Milestone 4: Math and graphing power

Completed.

- `FunctionGraph`
- `ParametricCurve`
- `ScatterPlot`
- `Matrix`
- improved equation layout

### Milestone 5: AI teaching primitives

Completed.

- `NeuralNetworkDiagram`
- `TransformerBlockDiagram`
- `AttentionMatrix`
- `TokenSequence`
- `DecisionBoundaryPlot`

### AI teaching motion status

Started.

Done:

- first semantic signal-propagation primitive exists through `SignalFlow`
- `NeuralNetworkDiagram` now exposes semantic path geometry
- animation builder support exists through `.propagate()` and `.propagate_to(...)`
- dedicated regression/demo scene exists in [neural_signal_flow.rs](/Users/ravishankar/personal-work/murali/examples/neural_signal_flow.rs)

Not done yet:

- multiple simultaneous signals
- richer activation styling for nodes and edges
- automatic path derivation helpers
- deeper integration into larger AI teaching templates and scenes

### Milestone 6: Advanced semantic animation

Completed as a strong first pass.

- transform matching
- object morph-style transitions
- equation symbol continuity
- matrix-step animations

Important note:

- object morphing is currently transform-match plus crossfade, not full topology-aware morphing

### Milestone 7: Export, docs, examples, polish

Partially completed.

Done:

- theme presets
- aiunderthehood templates
- canonical example structure
- export API scaffolding
- offscreen render capture API
- export docs

Not done yet:

- runtime-verified deterministic video export

## What changed in the docs

Older architecture docs were deleted because they were stale and duplicated each other.

Those deleted files described:

- older naming (`Sangh`)
- older dirty-tracking assumptions
- earlier projection/render boundaries
- architecture snapshots from before the recent milestone work

Current docs to keep:

- [architecture.md](/Users/ravishankar/personal-work/murali/docs/architecture.md)
- [export-and-templates.md](/Users/ravishankar/personal-work/murali/docs/export-and-templates.md)
- [text-setup.md](/Users/ravishankar/personal-work/murali/docs/text-setup.md)
- [ROADMAP_initial.md](/Users/ravishankar/personal-work/murali/ROADMAP_initial.md)

## Immediate next steps

Use [concrete-implementation-plan.md](/Users/ravishankar/personal-work/murali/implementation/concrete-implementation-plan.md) as the active execution plan.

Current priority order:

1. text system repair
2. export correctness and speed
3. neural-network flow animation
4. reliability and API polish
5. deeper Manim parity

## Recommended order from here

1. finish text bounds and layout integration
2. finish and accelerate export
3. add AI signal propagation primitives
4. clean up API and diagnostics
5. deepen parity and runtime structure
6. add post-parity 3D/assets/tooling work

## Bottom line

Murali is no longer at the “architecture sketch” stage.

It already has:

- a real semantic collection model
- ECS-backed rendering
- usable authored layout
- usable animation
- strong STEM support
- strong AI-teaching support

The main thing separating it from production readiness is export hardening and final API/tooling polish.

More specifically:

- text is now usable, but not fully layout-trustworthy
- export is still the biggest production blocker
- AI-specific motion primitives are now started, but still early
