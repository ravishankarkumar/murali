See [STYLE_GUIDE.md](./STYLE_GUIDE.md) for the canonical Murali example structure and preferred authored style.

The examples set is currently being consolidated. See [CONSOLIDATION_PLAN.md](./CONSOLIDATION_PLAN.md) for the keep / merge / retire plan that guides this cleanup.

All examples are now grouped by intent, but the run commands stay the same:

```bash
cargo run --example primitives_showcase
```

## Folder Map

- `examples/basics`
  Foundational Murali scenes: primitives, layout, shapes, styling, and motion.
- `examples/animation`
  Core animation semantics: appear/draw, text effects, morphing, and transform continuity.
- `examples/text_and_math`
  Text, formulas, matrices, tables, Fourier traces, and STEM-oriented authored scenes.
- `examples/graphs_and_fields`
  Graphs, plotted functions, vector fields, streamlines, noise-driven scenes, and map projections.
- `examples/dynamics`
  Updaters, particle systems, traced paths, swaying point fields, and physics-style motion.
- `examples/three_d`
  Parametric surfaces, wireframes, textured surfaces, and camera-driven 3D examples.
- `examples/ai_and_storytelling`
  AI teaching visuals, neural diagrams, agentic flows, and stepwise storytelling examples.
- `examples/branding_and_export`
  Capture/export workflows, blog-ready scenes, presentation templates, and logo examples.

## Start Here

`cargo run --example primitives_showcase`
Basic primitive/render smoke test.

`cargo run --example layout_playground`
Scene composition, anchors, stacks, and number-plane layout.

`cargo run --example animation_parity`
Core motion, visibility, follow hooks, and camera animation.

`cargo run --example stem_showcase`
Math and graphing objects for STEM lessons.

`cargo run --example semantic_animation_showcase`
Transform matching, morphing, equation continuity, and matrix-step animation.

`cargo run --example swaying_points`
Hundreds of VIBGYOR points react to a bouncing ball with local gravitational pull and drag.

`cargo run --example particle_nebula_showcase`
Layered particle belts create a bright core, drifting arcs, and a wide dust halo with subtle camera motion.

`cargo run --example textured_surface_showcase`
Earth texture mapped onto a spherical 3D parametric surface.

`cargo run --example ai_teaching_showcase`
AI teaching primitives in one scene.

`cargo run --example screenshot_markers`
Scene-level screenshot and GIF capture schedules driven by arrays of times.

## Canonical Examples

These are the examples we want to treat as the primary public learning path while the broader set is being compressed.

`cargo run --example primitives_showcase`
Foundational Murali primitives and rendering smoke test.

`cargo run --example layout_playground`
Scene composition, anchors, stacks, and layout thinking.

`cargo run --example animation_parity`
Core motion, visibility, follow hooks, and camera animation.

`cargo run --example semantic_animation_showcase`
Higher-level animation semantics such as morphing, transform matching, and continuity.

`cargo run --example stem_showcase`
Core text, math, graphing, and STEM-oriented authoring.

`cargo run --example vector_field_and_streamlines`
The recommended graph-and-field entrypoint while vector-field examples are consolidated.

`cargo run --example particle_nebula_showcase`
The recommended dynamics showcase.

`cargo run --example textured_surface_showcase`
The recommended 3D entrypoint.

`cargo run --example stepwise_showcase`
The recommended storytelling entrypoint for new work.

`cargo run --example screenshot_markers`
The recommended export/capture example.

## Advanced Examples

These remain valuable, but are not the first examples we should send new users to.

`cargo run --example styling_showcase`
Fills, strokes, dashes, and gradients.

`cargo run --example arrow_showcase`
Arrow variations and directional composition.

`cargo run --example map_projection_morph`
A richer long-form graphing and projection scene.

`cargo run --example formula_shape_formula_morph --features typst_embedded`
An advanced morph pipeline spanning formulas and shapes.

`cargo run --example fourier_formula_trace`
Fourier reconstruction of a sampled formula outline.

`cargo run --example swaying_points`
Dense updater-driven motion with many reacting points.

`cargo run --example traced_path_rolling_circle`
Cycloid-style traced-path example.

`cargo run --example blog_showcase`
Blog-ready composition example.

`cargo run --example murali_logo`
Branding/logo composition example.

## Legacy / Experimental

These examples remain in the repo for now, but they are not part of the preferred path.

`cargo run --example agentic_flow_chart`
Legacy `AgenticFlowChart` example. The feature remains available, but new storytelling work should prefer `stepwise_showcase`.

`cargo run --example agentic_flow_with_neural_node`
Advanced legacy agentic-flow example with embedded neural content.

`cargo run --example manim_sector_fill_demo`
Parity-style rendering demo kept mainly for comparison and regression value.

## Basics

`cargo run --example primitives_showcase`
Basic primitive/render smoke test.

`cargo run --example arrow_showcase`
Arrow primitive with various tip configurations and vector field patterns.

`cargo run --example layout_playground`
Scene composition, anchors, stacks, and number-plane layout.

`cargo run --example animated_motion`
Intent-first movement example with a clean canonical structure.

`cargo run --example shapes`
Hundreds of circles and rectangles appear with a stagger, remix their size/color/position, then bounce down off-screen.

`cargo run --example shapes_extended`
Showcase of new primitive shapes: rectangles, polygons, and ellipses.

`cargo run --example bezier_showcase`
Quadratic and cubic Bezier paths using the `Path` tattva.

`cargo run --example styling_showcase`
Fills, strokes, GPU dashes, and linear gradients.

## Animation

`cargo run --example animation_parity`
Core motion, visibility, follow hooks, and camera animation.

`cargo run --example semantic_animation_showcase`
Transform matching, morphing, equation continuity, and matrix-step animation.

`cargo run --example morph_and_move`
Simultaneous morphing and translation.

`cargo run --example morph_showcase`
Shape morphing between square, circle, rectangle, and triangle.

`cargo run --example write_effect_showcase`
Path drawing semantics with the new `draw()` API.

`cargo run --example unwrite_showcase`
Path removal semantics with `undraw()`.

`cargo run --example text_write_effect_showcase`
Text authoring with `typewrite_text()`.

`cargo run --example text_reveal_effects_showcase`
Side-by-side comparison of typewrite vs centered reveal text behavior.

`cargo run --example text_indicate_showcase`
Label indication pulses using the text-specific `.indicate()` animation.

## Text And Math

`cargo run --example stem_showcase`
Math and graphing objects for STEM lessons.

`cargo run --example latex_showcase`
LaTeX text-path regression scene.

`cargo run --example latex_matrix_showcase`
Bracketed matrices, determinants, and chained matrix expressions.

`cargo run --example latex_matrix_multiplication_morph`
Vectorized LaTeX matrix multiplication morph from a product expression into the resulting matrix.

`cargo run --example matrix_showcase`
Native matrix collection regression scene covering spacing, vectors, and highlights.

`cargo run --example typst_showcase`
Typst text-path regression scene.

`cargo run --example typst_to_typst_morph --features typst_embedded`
Vectorized Typst-to-Typst formula morph using path matching.

`cargo run --example typst_to_latex_morph --features typst_embedded`
Cross-pipeline vector morph from Typst-generated SVG paths to LaTeX-generated SVG paths.

`cargo run --example latex_to_latex_morph`
Vectorized LaTeX-to-LaTeX formula morph using SVG path output from `dvisvgm`.

`cargo run --example formula_morph_showcase`
High-level vector formula morph example.

`cargo run --example formula_shape_formula_morph --features typst_embedded`
Single-path bridge demo: Typst formula morphs into a shape, then into a LaTeX formula.

`cargo run --example fourier_formula_trace`
Epicycles reconstruct a sampled Typst formula outline and leave behind the traced curve.

`cargo run --example fourier_simple_trace`
Three epicycles build a simple looping trace over a full 30-second observation window.

`cargo run --example table_showcase`
Styled table composition and authored table animation.

`cargo run --example table_simple`
Minimal table example for quick onboarding.

`cargo run --example axes_and_labels`
Coordinate systems with axis labels.

## Graphs And Fields

`cargo run --example graph_on_axes`
Plotted function and sampled points on 2D axes.

`cargo run --example graph_draw_3d_camera`
Graph is progressively drawn while the camera moves in perspective.

`cargo run --example vector_field_showcase`
Radial, rotational, gradient, saddle, sine-wave, and magnitude-colored vector fields.

`cargo run --example stream_lines_showcase`
Streamline visualizations for several flow patterns.

`cargo run --example vector_field_and_streamlines`
Side-by-side comparison of vector field and streamline representations.

`cargo run --example noisy_circle`
Circular outline driven by polar Perlin noise.

`cargo run --example perlin_noise_horizon`
Generative horizon with a Perlin-noise top edge and shifting fill.

`cargo run --example map_projection_morph`
Stylized world coastlines and graticule morph through several projections over 30 seconds.

## Dynamics

`cargo run --example projectile_with_updaters`
Projectile motion with dynamic velocity vectors and traced path.

`cargo run --example force_field_with_updaters`
Single charged particle moving through an electric field.

`cargo run --example force_field_multiple_charges`
Multiple charges creating a combined electric field by superposition.

`cargo run --example traced_path_rolling_circle`
Rolling circle tracing a cycloid from a point on its circumference.

`cargo run --example swaying_points`
Dense multicolor point field reacting to a bouncing ball with local pull and drag.

`cargo run --example particle_belt`
Colored particles orbit in a stylized asteroid belt.

`cargo run --example particle_nebula_showcase`
Layered particle belts create a cinematic nebula-like system.

## Three D

`cargo run --example parametric_surface_showcase`
Introductory 3D parametric surface example.

`cargo run --example parametric_surface_animated`
Animated surface deformation in 3D.

`cargo run --example parametric_surface_advanced`
Multiple 3D surfaces in one composed scene.

`cargo run --example parametric_surface_wireframe`
Wireframe surface rendering examples.

`cargo run --example parametric_surface_wireframe_animated`
Animated wireframe surface example.

`cargo run --example parametric_surface_wireframe_showcase`
Comparative wireframe surface showcase.

`cargo run --example textured_surface_showcase`
Earth texture mapped onto a spherical parametric surface.

`cargo run --example manim_sector_fill_demo`
Sector-style fill animation demo for shape rendering parity.

## AI And Storytelling

`cargo run --example ai_teaching_showcase`
AI teaching primitives in one scene.

`cargo run --example neural_signal_flow`
Signal propagation through a neural-network path.

`cargo run --example neural_network_v2`
Deactivated-node behavior, multi-path propagation, and signal playback modes.

`cargo run --example agentic_flow_chart`
Agentic flow chart composition and authored transitions.

`cargo run --example agentic_flow_with_neural_node`
Flow chart pauses at a neural-network node, waits for neural signal loops, then resumes.

`cargo run --example agentic_flow_write_animation`
Write-on treatment for an agentic flow scene.

`cargo run --example agentic_reveal_showcase`
Reveal-first staging for agentic flow storytelling.

`cargo run --example stepwise`
Stepwise storytelling scene.

`cargo run --example stepwise_backpath`
Backtracking path behavior in a stepwise scene.

`cargo run --example stepwise_neural_node`
Stepwise flow with embedded neural-node content.

`cargo run --example stepwise_script_api`
Script-driven stepwise authoring.

## Branding And Export

`cargo run --example aiu_attention_template`
aiunderthehood-ready presentation template with branded defaults.

`cargo run --example export_aiu_attention`
Deterministic export example writing PNG frames and, if available, an MP4.

`cargo run --example blog_showcase`
A short blog-ready animation with neural networks, code blocks, and labels.

`cargo run --example screenshot_markers`
Scene-level screenshot and GIF capture schedules driven by arrays of times.

`cargo run --example murali_logo`
Murali-native logo concept built from plotted lines, curves, and a clean wordmark.

`cargo run --example murali_flute_logo`
Minimal flute-led Murali logo concept built from geometric primitives.

`cargo run --example murali_flute_feather_logo`
Flute-led logo with a peacock feather built from graph-like curves and concentric plotted forms.
