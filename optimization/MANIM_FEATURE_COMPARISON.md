# Murali vs Manim: Feature Comparison

## Overview

This document provides a comprehensive feature comparison between Murali and Manim (3Blue1Brown's animation engine). It identifies gaps in Murali's current implementation and prioritizes missing features based on impact and implementation complexity.

---

## Executive Summary

**Murali's Current Status:**
- ✅ Core rendering pipeline (wgpu-based)
- ✅ Basic shapes and primitives
- ✅ Text rendering (LaTeX, Typst, regular)
- ✅ Basic animations (move, rotate, scale, fade)
- ✅ AI/ML visualization templates
- ❌ Advanced morphing and transformations
- ❌ Tables and structured data
- ❌ Force fields and vector fields
- ❌ Complex shape morphing
- ❌ Advanced camera controls

**Manim's Strengths:**
- Mature ecosystem with extensive shape library
- Rich transformation and morphing capabilities
- Advanced mathematical visualization tools
- Extensive animation library
- Strong community and documentation

---

## Feature Categories

### 1. Basic Shapes & Primitives

#### Murali Implementation

| Category | Feature | Status | Notes |
|----------|---------|--------|-------|
| **2D Shapes** | Circle | ✅ | Basic circle with configurable segments |
| | Ellipse | ✅ | Configurable radii |
| | Rectangle | ✅ | Width/height based |
| | Square | ✅ | Size-based |
| | Polygon | ✅ | Vertex-based |
| | Line | ✅ | Start/end points, thickness, dashing |
| | Path | ✅ | Bézier curves and line segments |
| | Cube | ✅ | 3D cube primitive |
| **3D Shapes** | Axes3D | ✅ | 3D coordinate system |
| | ParametricCurve3D | ✅ | 3D parametric curves |
| **Composite** | Axes | ✅ | 2D axes with ticks |
| | NumberPlane | ✅ | Grid-based coordinate plane |
| | Group | ✅ | Basic grouping |
| | HStack/VStack | ✅ | Horizontal/vertical stacking |

#### Manim Implementation

| Category | Feature | Status | Notes |
|----------|---------|--------|-------|
| **2D Shapes** | Circle, Ellipse, Rectangle, Square | ✅ | Full featured |
| | Polygon, Triangle, RegularPolygon | ✅ | All polygon types |
| | Line, Arrow, DoubleArrow | ✅ | Multiple arrow types |
| | Arc, Sector, Annulus | ✅ | Advanced shapes |
| | Dot, SmallDot | ✅ | Point markers |
| | Wedge, Annular Sector | ✅ | Pie chart elements |
| **3D Shapes** | Cube, Sphere, Cylinder, Cone, Torus | ✅ | Full 3D library |
| | Surface, ParametricSurface | ✅ | 3D surfaces |
| | Line3D, Cylinder3D | ✅ | 3D primitives |
| **Composite** | VGroup, Group | ✅ | Hierarchical grouping |
| | Axes, NumberPlane, PolarPlane | ✅ | Coordinate systems |
| | Graph (networkx) | ✅ | Graph visualization |

**Gap Analysis:**
- Missing: Arrow types (single, double, curved)
- Missing: Sector/Wedge (pie chart elements)
- Missing: Annulus (ring shape)
- Missing: Dot/SmallDot (point markers)
- Missing: 3D surfaces and parametric surfaces
- Missing: Polar coordinate system

---

### 2. Text & Typography

#### Murali Implementation

| Feature | Status | Notes |
|---------|--------|-------|
| LaTeX rendering | ✅ | Via tectonic + dvisvgm |
| Typst rendering | ✅ | Embedded Typst compiler |
| Regular text (Label) | ✅ | Glyph-based, ASCII only |
| Text color | ✅ | Per-text color |
| Text sizing | ✅ | World-space sizing |
| Text reveal animation | ✅ | Character-by-character reveal |
| Code blocks | ✅ | Via Typst with syntax highlighting |
| Equation support | ✅ | Via LaTeX/Typst |
| Matrix rendering | ✅ | Via Typst |
| Multi-line text | ❌ | Not supported |
| Text alignment | ❌ | Limited support |
| Rich text formatting | ❌ | Not supported |
| Font selection | ❌ | Fixed fonts only |
| Text morphing | ❌ | Not supported |

#### Manim Implementation

| Feature | Status | Notes |
|---------|--------|-------|
| LaTeX rendering | ✅ | Full LaTeX support |
| Text rendering | ✅ | Pango-based text |
| MathTex | ✅ | Math-specific text |
| Paragraph | ✅ | Multi-line text |
| MarkupText | ✅ | Rich text formatting |
| Code | ✅ | Syntax-highlighted code |
| Text color | ✅ | Full color support |
| Text sizing | ✅ | Flexible sizing |
| Text alignment | ✅ | Full alignment options |
| Font selection | ✅ | Multiple fonts |
| Text morphing | ✅ | Text-to-text morphing |
| Subscripts/Superscripts | ✅ | Full support |

**Gap Analysis:**
- Missing: Multi-line text (Paragraph equivalent)
- Missing: Rich text formatting (MarkupText)
- Missing: Font selection
- Missing: Text morphing (text-to-text)
- Missing: Subscript/superscript support
- Missing: Text alignment options

---

### 3. Animations & Transformations

#### Murali Implementation

| Animation Type | Status | Notes |
|---|---|---|
| **Basic** | MoveTo | ✅ | Position animation |
| | RotateTo | ✅ | Rotation animation |
| | ScaleTo | ✅ | Scale animation |
| | FadeTo | ✅ | Opacity animation |
| | Create | ✅ | Fade-in creation |
| **Easing** | Linear, InQuad, OutQuad, InOutQuad | ✅ | Basic easing curves |
| | InCubic, OutCubic, InOutCubic | ✅ | Cubic easing |
| **Specialized** | PropagateSignal | ✅ | Signal flow animation |
| | RevealTo | ✅ | Reveal animation |
| | NoisePhaseTo | ✅ | Noise phase animation |
| | BeltPhaseTo | ✅ | Particle belt animation |
| | HorizonPhaseTo | ✅ | Horizon animation |
| | FollowAnchor | ✅ | Constraint-based animation |
| **Morphing** | Shape morphing | ❌ | Not implemented |
| | Text morphing | ❌ | Not implemented |
| | Topology-aware morphing | ❌ | Not implemented |
| **Transformations** | ReplacementTransform | ❌ | Not implemented |
| | TransformFromCopy | ❌ | Not implemented |
| | MorphPathPointCloud | ❌ | Not implemented |

#### Manim Implementation

| Animation Type | Status | Notes |
|---|---|---|
| **Basic** | Move, Rotate, Scale, Fade | ✅ | Full support |
| **Easing** | 20+ easing functions | ✅ | Comprehensive |
| **Transformations** | ReplacementTransform | ✅ | Morph one shape to another |
| | TransformFromCopy | ✅ | Transform with source copy |
| | MorphPathPointCloud | ✅ | Point cloud morphing |
| | ClockwiseTransform | ✅ | Directional morphing |
| **Specialized** | Write | ✅ | Stroke reveal animation |
| | DrawBorderThenFill | ✅ | Border then fill animation |
| | ShowPassingFlash | ✅ | Flash animation |
| | Broadcast | ✅ | Ripple effect |
| | Wiggle | ✅ | Wiggle animation |
| | Circumscribe | ✅ | Circle highlight |
| | Indicate | ✅ | Emphasis animation |
| | Flash | ✅ | Flash effect |
| **Composition** | AnimationGroup | ✅ | Animation composition |
| | Succession | ✅ | Sequential animations |
| | LaggedStart | ✅ | Staggered animations |

**Gap Analysis:**
- Missing: ReplacementTransform (shape morphing)
- Missing: TransformFromCopy
- Missing: MorphPathPointCloud
- Missing: Write animation (stroke reveal)
- Missing: DrawBorderThenFill
- Missing: Emphasis animations (Indicate, Circumscribe)
- Missing: Flash and ripple effects
- Missing: Animation composition (AnimationGroup, Succession)
- Missing: Lagged/staggered animations
- Missing: Advanced easing functions (20+ vs 7)

---

### 4. Mathematical Visualization

#### Murali Implementation

| Feature | Status | Notes |
|---------|--------|-------|
| **Graphs** | FunctionGraph | ✅ | 2D function plotting |
| | ParametricCurve | ✅ | 2D parametric curves |
| | ParametricCurve3D | ✅ | 3D parametric curves |
| | ScatterPlot | ✅ | Point scatter plots |
| **Coordinate Systems** | Axes | ✅ | 2D axes |
| | NumberPlane | ✅ | 2D grid plane |
| | Axes3D | ✅ | 3D axes |
| **Data Structures** | Matrix | ✅ | Matrix display |
| | Equation | ✅ | Equation display |
| **AI/ML** | NeuralNetworkDiagram | ✅ | Neural network visualization |
| | AttentionMatrix | ✅ | Attention heatmap |
| | TokenSequence | ✅ | Token sequence display |
| | TransformerBlockDiagram | ✅ | Transformer architecture |
| | DecisionBoundaryPlot | ✅ | Classification boundary |
| | SignalFlow | ✅ | Signal flow diagram |
| | AgenticFlowChart | ✅ | Agentic flow visualization |
| **Advanced** | Vector fields | ❌ | Not implemented |
| | Force fields | ❌ | Not implemented |
| | Streamlines | ❌ | Not implemented |
| | Heatmaps | ❌ | Not implemented |
| | 3D surfaces | ❌ | Not implemented |
| | Implicit surfaces | ❌ | Not implemented |

#### Manim Implementation

| Feature | Status | Notes |
|---------|--------|-------|
| **Graphs** | FunctionGraph | ✅ | Function plotting |
| | ParametricCurve | ✅ | Parametric curves |
| | ImplicitFunction | ✅ | Implicit function plotting |
| | PolarPlot | ✅ | Polar coordinate plotting |
| **Coordinate Systems** | Axes, NumberPlane, PolarPlane | ✅ | Multiple coordinate systems |
| **Data Structures** | Table, DecimalTable, MathTable | ✅ | Table support |
| | Matrix | ✅ | Matrix display |
| **Vector Fields** | VectorField | ✅ | Vector field visualization |
| | StreamLines | ✅ | Streamline visualization |
| **Advanced** | Surface, ParametricSurface | ✅ | 3D surfaces |
| | ImplicitSurface | ✅ | Implicit 3D surfaces |
| | Heatmap | ✅ | Heatmap visualization |
| | Graph (networkx) | ✅ | Graph/network visualization |

**Gap Analysis:**
- Missing: Vector fields
- Missing: Force fields
- Missing: Streamlines
- Missing: Heatmaps
- Missing: 3D surfaces
- Missing: Implicit surfaces
- Missing: Implicit function plotting
- Missing: Polar coordinate system
- Missing: Table support (Table, DecimalTable, MathTable)
- Missing: Graph/network visualization (networkx integration)

---

### 5. Advanced Features

#### Murali Implementation

| Feature | Status | Notes |
|---------|--------|-------|
| **Camera** | Basic camera | ✅ | Fixed camera |
| | Camera animation | ✅ | Frame, move, lookat |
| | 3D camera | ✅ | Basic 3D support |
| | Zoom | ❌ | Not implemented |
| | Pan | ❌ | Not implemented |
| | Orbit | ❌ | Not implemented |
| **Rendering** | 2D rendering | ✅ | Full 2D support |
| | 3D rendering | ✅ | Basic 3D support |
| | Lighting | ❌ | Not implemented |
| | Shadows | ❌ | Not implemented |
| | Transparency | ✅ | Alpha blending |
| **Effects** | Blur | ❌ | Not implemented |
| | Glow | ❌ | Not implemented |
| | Stroke effects | ❌ | Not implemented |
| | Particle effects | ✅ | ParticleBelt |
| **Interaction** | Interactive scenes | ❌ | Not implemented |
| | Real-time preview | ✅ | Live preview |
| | Export | ✅ | Video export |

#### Manim Implementation

| Feature | Status | Notes |
|---------|--------|-------|
| **Camera** | MovingCamera | ✅ | Dynamic camera |
| | ThreeDCamera | ✅ | 3D camera |
| | Zoom, Pan, Orbit | ✅ | Full camera control |
| **Rendering** | 2D rendering | ✅ | Cairo-based |
| | 3D rendering | ✅ | OpenGL-based |
| | Lighting | ✅ | Basic lighting |
| | Shadows | ✅ | Shadow support |
| **Effects** | Blur, Glow | ✅ | Post-processing effects |
| | Stroke effects | ✅ | Stroke customization |
| | Particle effects | ✅ | Particle systems |
| **Plugins** | Plugin system | ✅ | Extensible |
| **Export** | Video export | ✅ | Multiple formats |
| | GIF export | ✅ | GIF support |
| | SVG export | ✅ | SVG export |

**Gap Analysis:**
- Missing: MovingCamera (dynamic camera)
- Missing: Zoom, Pan, Orbit camera controls
- Missing: Lighting and shadows
- Missing: Post-processing effects (blur, glow)
- Missing: Plugin system
- Missing: GIF export
- Missing: SVG export

---

## Missing Features by Priority

### Tier 1: High Impact, Medium Effort

#### 1.1 Shape Morphing (ReplacementTransform)

**Impact:** Critical for educational animations
**Complexity:** High
**Estimated Effort:** 2-3 weeks

**What it does:**
- Smoothly transforms one shape into another
- Maintains visual continuity
- Handles topology changes

**Implementation Strategy:**
```rust
pub struct ReplacementTransform {
    pub from_id: TattvaId,
    pub to_id: TattvaId,
    pub ease: Ease,
    correspondence: Option<Vec<(usize, usize)>>,  // Vertex correspondence
}

impl Animation for ReplacementTransform {
    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        // Interpolate between vertex positions
        // Handle topology changes
    }
}
```

**Why it matters:**
- Enables mathematical transformations
- Core Manim feature
- Essential for shape-to-shape animations

---

#### 1.2 Vector Fields & Force Fields

**Impact:** High for physics/math visualization
**Complexity:** High
**Estimated Effort:** 2-3 weeks

**What it does:**
- Visualizes vector fields (arrows at grid points)
- Shows force fields with magnitude-based coloring
- Streamline visualization

**Implementation Strategy:**
```rust
pub struct VectorField {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub grid_size: (u32, u32),
    pub field_fn: Arc<dyn Fn(Vec2) -> Vec2 + Send + Sync>,
    pub color_fn: Option<Arc<dyn Fn(f32) -> Vec4 + Send + Sync>>,
}

impl Project for VectorField {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // Generate arrows for each grid point
        // Color by magnitude
    }
}
```

**Why it matters:**
- Essential for physics animations
- Visualizes gradients, flows, forces
- Commonly used in educational content

---

#### 1.3 Table Support (Table, DecimalTable, MathTable)

**Impact:** High for data visualization
**Complexity:** Medium
**Estimated Effort:** 1-2 weeks

**What it does:**
- Displays tabular data
- Supports row/column labels
- Cell highlighting and styling

**Implementation Strategy:**
```rust
pub struct Table {
    pub rows: Vec<Vec<String>>,
    pub row_labels: Option<Vec<String>>,
    pub col_labels: Option<Vec<String>>,
    pub cell_height: f32,
    pub cell_width: f32,
}

impl Project for Table {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // Generate grid of cells
        // Add labels
        // Create mesh for rendering
    }
}
```

**Why it matters:**
- Common in educational videos
- Data presentation
- Manim has extensive table support

---

### Tier 2: Medium Impact, Medium Effort

#### 2.1 Text Morphing

**Impact:** Medium for text-based animations
**Complexity:** High
**Estimated Effort:** 2-3 weeks

**What it does:**
- Morphs one text string into another
- Character-level correspondence
- Smooth interpolation

**Implementation Strategy:**
```rust
pub struct TextMorph {
    pub from_id: TattvaId,
    pub to_text: String,
    pub ease: Ease,
}

impl Animation for TextMorph {
    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        // Interpolate between character positions
        // Handle character additions/removals
    }
}
```

**Why it matters:**
- Enables text-based storytelling
- Smooth text transitions
- Educational emphasis

---

#### 2.2 Write Animation (Stroke Reveal)

**Impact:** High for visual appeal
**Complexity:** Medium
**Estimated Effort:** 1-2 weeks

**What it does:**
- Animates stroke drawing
- Reveals path progressively
- Customizable stroke width

**Implementation Strategy:**
```rust
pub struct Write {
    pub target_id: TattvaId,
    pub ease: Ease,
    pub stroke_width: f32,
}

impl Animation for Write {
    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        // Update path reveal progress
        // Render partial path
    }
}
```

**Why it matters:**
- Iconic Manim animation
- Highly visual and engaging
- Essential for shape drawing

---

#### 2.3 Animation Composition (Succession, LaggedStart)

**Impact:** Medium for animation control
**Complexity:** Low
**Estimated Effort:** 1 week

**What it does:**
- Sequence animations
- Stagger animations with delays
- Parallel animation groups

**Implementation Strategy:**
```rust
pub struct Succession {
    pub animations: Vec<Box<dyn Animation>>,
    pub current_index: usize,
}

pub struct LaggedStart {
    pub animations: Vec<Box<dyn Animation>>,
    pub lag_ratio: f32,
}
```

**Why it matters:**
- Better animation control
- Reduces boilerplate
- Enables complex sequences

---

### Tier 3: Medium Impact, Low Effort

#### 3.1 Additional Easing Functions

**Impact:** Low-medium for animation quality
**Complexity:** Low
**Estimated Effort:** 1-2 days

**What it does:**
- Adds more easing curves
- Exponential, elastic, bounce easing
- Custom easing support

**Implementation Strategy:**
```rust
pub enum Ease {
    // Existing
    Linear,
    InQuad, OutQuad, InOutQuad,
    InCubic, OutCubic, InOutCubic,
    
    // New
    InQuart, OutQuart, InOutQuart,
    InQuint, OutQuint, InOutQuint,
    InExpo, OutExpo, InOutExpo,
    InCirc, OutCirc, InOutCirc,
    InElastic, OutElastic, InOutElastic,
    InBounce, OutBounce, InOutBounce,
    InBack, OutBack, InOutBack,
}
```

**Why it matters:**
- Better animation feel
- Professional polish
- Minimal implementation cost

---

#### 3.2 Additional Shape Primitives

**Impact:** Low for basic shapes
**Complexity:** Low
**Estimated Effort:** 1-2 days per shape

**Missing shapes:**
- Arrow (single, double, curved)
- Sector/Wedge (pie chart)
- Annulus (ring)
- Dot/SmallDot (point markers)
- RegularPolygon (n-sided)
- Star

**Why it matters:**
- Completeness
- Common in diagrams
- Easy to implement

---

#### 3.3 Multi-line Text (Paragraph)

**Impact:** Medium for text support
**Complexity:** Medium
**Estimated Effort:** 1-2 weeks

**What it does:**
- Multi-line text support
- Text alignment (left, center, right)
- Line spacing control

**Implementation Strategy:**
```rust
pub struct Paragraph {
    pub text: String,
    pub width: f32,
    pub alignment: TextAlignment,
    pub line_spacing: f32,
}

pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}
```

**Why it matters:**
- Essential for text-heavy content
- Better text layout
- Educational content support

---

### Tier 4: Low Impact, High Effort (Research)

#### 4.1 3D Surfaces & Implicit Surfaces

**Impact:** Low-medium for 3D visualization
**Complexity:** Very High
**Estimated Effort:** 3-4 weeks

**What it does:**
- Renders 3D parametric surfaces
- Implicit surface visualization
- Lighting and shading

**Why it matters:**
- Advanced 3D visualization
- Niche use case
- High implementation complexity

---

#### 4.2 Lighting & Shadows

**Impact:** Low for visual quality
**Complexity:** High
**Estimated Effort:** 2-3 weeks

**What it does:**
- Basic lighting model
- Shadow rendering
- Material properties

**Why it matters:**
- Visual polish
- 3D realism
- Not critical for educational content

---

#### 4.3 Post-Processing Effects (Blur, Glow)

**Impact:** Low for visual effects
**Complexity:** Medium
**Estimated Effort:** 1-2 weeks

**What it does:**
- Blur effects
- Glow/bloom effects
- Color grading

**Why it matters:**
- Visual polish
- Special effects
- Not critical for core functionality

---

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-2)
1. Additional easing functions (1 day)
2. Basic shape primitives (Arrow, Dot, Sector) (2-3 days)
3. Animation composition (Succession, LaggedStart) (1 week)

### Phase 2: Core Features (Weeks 3-6)
1. Write animation (stroke reveal) (1-2 weeks)
2. Table support (1-2 weeks)
3. Vector fields (2-3 weeks)

### Phase 3: Advanced Features (Weeks 7-10)
1. Shape morphing (ReplacementTransform) (2-3 weeks)
2. Text morphing (2-3 weeks)
3. Multi-line text (Paragraph) (1-2 weeks)

### Phase 4: Polish (Weeks 11+)
1. 3D surfaces (3-4 weeks)
2. Lighting & shadows (2-3 weeks)
3. Post-processing effects (1-2 weeks)

---

## Feature Comparison Matrix

| Feature | Murali | Manim | Priority |
|---------|--------|-------|----------|
| Basic shapes | ✅ | ✅ | - |
| Text rendering | ✅ | ✅ | - |
| Basic animations | ✅ | ✅ | - |
| Shape morphing | ❌ | ✅ | Tier 1 |
| Vector fields | ❌ | ✅ | Tier 1 |
| Tables | ❌ | ✅ | Tier 1 |
| Write animation | ❌ | ✅ | Tier 2 |
| Text morphing | ❌ | ✅ | Tier 2 |
| Animation composition | ❌ | ✅ | Tier 2 |
| Easing functions | ✅ (7) | ✅ (20+) | Tier 3 |
| Multi-line text | ❌ | ✅ | Tier 3 |
| 3D surfaces | ❌ | ✅ | Tier 4 |
| Lighting | ❌ | ✅ | Tier 4 |
| Post-processing | ❌ | ✅ | Tier 4 |

---

## Recommendations

### Short-term (Next 2-3 months)
1. Implement Tier 1 features (shape morphing, vector fields, tables)
2. Add Tier 2 features (write animation, text morphing)
3. Expand easing functions

### Medium-term (3-6 months)
1. Multi-line text support
2. Animation composition
3. Additional shape primitives
4. Graph/network visualization

### Long-term (6+ months)
1. 3D surfaces and implicit surfaces
2. Lighting and shadows
3. Post-processing effects
4. Plugin system

---

## Conclusion

Murali has a solid foundation with core rendering and animation capabilities. The main gaps are:

1. **Shape morphing** - Critical for educational animations
2. **Vector/force fields** - Essential for physics visualization
3. **Tables** - Common in data visualization
4. **Write animation** - Iconic Manim feature
5. **Text morphing** - Enables text-based storytelling

Implementing these features would bring Murali to feature parity with Manim for most educational use cases. The roadmap prioritizes high-impact features with reasonable implementation complexity.

Murali's advantages over Manim:
- Modern GPU backend (wgpu vs OpenGL)
- Rust type safety
- Better performance potential
- Cleaner architecture

With focused development on the identified gaps, Murali can become a compelling alternative to Manim for mathematical animation.
