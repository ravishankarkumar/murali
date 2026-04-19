---
sidebar_position: 7
---

# Roadmap

## v0.1.4 (current)

- wgpu rendering backend (Vulkan / Metal / DX12)
- World-space coordinate system and camera
- Primitives: Circle, Square, Rectangle, Ellipse, Line, Polygon, Cube, Path
- Text: Label (glyph-based), Latex / Typst (compiled)
- Composite: Axes, NumberPlane
- Timeline animation system
- Animations: move, fade, scale, rotate, morph, create/write
- 8 easing functions
- Video export (frames + mp4)
- Orbit and pan/zoom camera controllers
- External config file support

## Upcoming

### Phase 2 — Layout
- Anchors and alignment (`next_to`, `align_to`)
- Bounding boxes in world space
- Relative placement helpers

### Phase 3 — Morphing
- Topology-aware tattva → tattva morphing
- Text ↔ shape morphing
- Mathematical equation morphing (symbol-level continuity)

### Phase 4 — Math visualization
- 2D parametric and functional graphs
- 3D function surfaces with lighting
- NumberPlane / grid tattva

### Phase 5 — Assets
- glTF mesh import
- Materials and transparency

### Phase 6 — Polish
- `murali doctor` environment diagnostics
- Color themes and palettes
- Murali logo animation
- Crates.io release
