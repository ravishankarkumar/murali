# Murali development roadmap

This roadmap is ordered by **architectural dependency**.

---

## Phase 1: Rendering & Text Backends (Mostly Done)

1. **wgpu setup** ✅  
   - Window + surface
   - Swapchain
   - Basic render loop

2. **Typst integration** ✅  
   - Embedded Typst compilation
   - SVG → RGBA rasterization
   - Text rendered as textured quads

---

## Phase 2: Space & Coordinates (Critical Foundation)

3. **Coordinate system & camera**✅ 
   - World space (math space) definition
   - Origin at center
   - Fixed 16:9 aspect ratio
   - World → View → Clip transform pipeline

<!-- 4. **Typst text sizing & positioning in world space**
   - Text size defined in world units (not pixels)
   - Raster resolution decoupled from layout
   - Screen preview matches video export -->

4. **Latex text sizing & positioning in world space** ✅
   - Text size defined in world units (not pixels)
   - Raster resolution decoupled from layout
   - Screen preview matches video export (video export configurable)

5. **Normal text integration**✅
    - Lightweight glyph-based text backend
    - Single-line text (numbers, labels, short strings)
    - World-space sizing (same invariants as LaTeX/Typst)
    - Shares anchoring, bounding boxes, and layout logic
    - Intended for performance-sensitive, non-math text
    - Anchoring not yet done ❌

6. **Coordinate axes tattva**
   - X and Y axes
   - Tick marks
   - Numeric labels (using Typst)
   - Acts as a validation tool for world scaling

7. **Anchors & alignment**
   - Bounding boxes in world space
   - Anchors: LEFT, RIGHT, UP, DOWN, CENTER
   - Relative placement (`next_to`, `align_to`, etc.)
   - Required for manim-like layout ergonomics

---

## Phase 3: Time & Motion

8. **Time abstraction**
   - Global timeline / playhead
   - `t`, `dt`, and duration-based animations
   - Deterministic playback (preview == render)
   - Animation scheduling and composition

9. **Movement of Tattvas**
   - Position, rotation, scale animations
   - Driven purely by time functions
   - No frame-dependent logic

10. **Hooks on Tattvas / DrawableInstances**
    - Observe position, size, or state of another tattva
    - Enables constraints and reactive layout
    - Example: label following a moving object

11. **Creation & removal animations**
    - Manim-style `Create`, `FadeIn`, `FadeOut`
    - Progressive stroke reveals
    - Time-based, not frame-based

---

## Phase 4: Structure & Morphing

12. **Scene graph vs flat list decision**
    - Decide if tattvas can have children
    - Transform propagation semantics
    - Grouping and hierarchical animations
    - Must be finalized before complex morphing

13. **Tattva → Tattva morphing**
    - Shape correspondence
    - Topology-aware morphs
    - Not just vertex interpolation

14. **Text ↔ Tattva morphing**
    - Text turning into shapes and vice versa
    - Requires semantic glyph/shape mapping

15. **Mathematical equation morphing**
    - Equation-to-equation transforms
    - Symbol-level continuity (Typst-driven)

16. **Matrix & structured math support**
    - Matrix layouts
    - Step-by-step transformations
    - Algebra-friendly animations

---

## Phase 5: Math Visualization

17. **2D mathematical graphs**
    - Axes-aware plotting
    - Parametric and functional graphs

18. **NumberPlane / Grid tattva**
    - Full vertical & horizontal grid lines
    - Major and minor spacing
    - Optional axes overlay
    - Used as background for graphs

19. **3D function surfaces**
    - Surface meshes
    - Camera-aware rendering
    - Lighting (basic)

---

## Phase 6: Assets & Materials

20. **glTF integration**
    - Import external meshes
    - Transform and animate them as tattvas

21. **Materials & surface properties**
    - Colors
    - Transparency
    - Future lighting models

---

## Phase 7: Output & Polish

22. **Configuration & environment diagnostics**
    - Defining default config ✅
    - Adding support for external config ✅
    - Adding "murali doctor" for environment diagnostics

23. **Frame rate & output format settings**
    - Fixed timestep rendering
    - Resolution independence

24. **Video output**
    - Offscreen rendering
    - Frame capture
    - Encoder integration
    - Deterministic execution (same input → same frames)

25. **API ergonomics**
    - Clean, minimal, composable API
    - Manim-like expressiveness without magic

26. **API & code cleanup**
    - Remove legacy paths
    - Simplify abstractions

27. **Color theme and color palette**
    - Choosing color palette
    - Adding support for themes
    - Adding a couple of themes

28. **Murali logo animation & branding**
    - Design Murali logo using Murali itself
    - Simple logo reveal animation
    - Reusable as splash / intro for demos and videos

29. **Examples & documentation**
    - Canonical examples
    - Design philosophy docs
    - Tutorials for common animations

---

## Guiding Principles

- World space first, pixels last
- Time-driven, not frame-driven
- Semantic math > visual hacks
- Ergonomics without compromising correctness

 