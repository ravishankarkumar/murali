---
sidebar_position: 3
---

# Tattvas

A tattva is any object that can be placed, animated, and rendered in a scene. The word comes from Sanskrit and means "element" or "essence".

Every tattva is added to the scene with `add_tattva(shape, position)` which returns a `TattvaId` for later reference:

```rust
let id = scene.add_tattva(Circle::new(1.0, 64, Vec4::new(0.2, 0.6, 1.0, 1.0)), Vec3::ZERO);
```

Tattvas are organized into categories based on their purpose:

- [Primitives](primitives) — basic shapes: circle, square, rectangle, line, polygon, arrow, path
- [Text](text) — label, latex, typst, code block
- [Composite](composite) — axes, number plane, 3D axes
- [Graphs](graphs) — function graphs, scatter plots, parametric surfaces, vector fields
- [Math](math) — equation layout, matrix with brackets
- [Layout](layout) — HStack, VStack for arranging groups of tattvas
- [Storytelling](storytelling) — stepwise reveal for step-by-step explanations
- [Utility](utility) — traced path, screenshot marker
