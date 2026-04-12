# Murali Architecture

## What Murali is now

Murali is a semantic animation engine with an ECS-backed render runtime.

The current shape of the system is:

- `src/frontend/`
  - authored meaning
  - user-facing objects
  - animation definitions
  - layout helpers
- `src/projection/`
  - pure CPU-side render descriptions
  - backend-neutral meshes and primitives
- `src/backend/`
  - sync boundary
  - ECS cache world
  - renderer and GPU resources
- `src/engine/`
  - scene ownership
  - timeline progression
  - app lifecycle
  - export path
- `src/resource/`
  - text, LaTeX, Typst, raster, and asset machinery

## Core principles

1. Semantic objects remain the source of truth.
2. Projection stays deterministic and backend-neutral.
3. ECS is a runtime cache and execution layer, not the authoring model.
4. Transform-only changes should stay cheaper than geometry/text rebuilds.
5. Preview and export should converge toward the same deterministic frame model.

## What the code already supports

### Semantic collections

Murali currently has these user-facing object families:

- `primitives`
- `text`
- `composite`
- `layout`
- `graph`
- `math`
- `ai`

That means Murali already has first-class support for:

- labels, LaTeX, Typst
- axes and number planes
- groups and stacks
- function graphs, parametric curves, scatter plots
- equations and matrices
- AI teaching diagrams like attention matrices, token sequences, transformer blocks, neural nets, and signal-flow overlays

## Current runtime model

The current frame model is:

1. timelines mutate semantic state and props
2. dirty flags indicate what kind of rebuild is needed
3. projection emits neutral render primitives
4. sync materializes ECS render entities
5. renderer draws mesh, text, and line data

Important current detail:

- typed dirty flags exist
- transform-only changes have a faster path
- semantic animation now includes transform matching, morph-style transitions, equation continuity, and matrix-step highlighting
- AI-specific motion has started through `SignalFlow` and propagation-style animation on network paths

## What is still unfinished

The architecture is good enough for ongoing feature work, but a few things are still clearly incomplete:

- export runtime is not fully production-ready yet
- sync is still centralized in `src/backend/sync.rs`
- true topology-aware morphing is not done
- hierarchy/scene-graph decisions are still lightweight rather than fully formalized
- glTF, materials, and 3D surfaces are not implemented yet

## Source of truth for planning

For implementation status and next steps, use:

- [ROADMAP_initial.md](/Users/ravishankar/personal-work/murali/ROADMAP_initial.md)
- [status-and-next-steps.md](/Users/ravishankar/personal-work/murali/implementation/status-and-next-steps.md)
