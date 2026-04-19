# Examples Consolidation Plan

This document tracks the planned reduction and reorganization of the examples set.

The goal is not to remove useful coverage. The goal is to make the public examples surface:

- easier to navigate
- easier to maintain
- more intentional as part of Murali's product experience

We will prefer one strong example per concept over multiple overlapping examples.

## Target Shape

We want three layers:

1. Canonical examples
   These are the examples new users should run first and the ones docs should prefer.
2. Advanced examples
   These show deeper or more specialized capabilities, but still remain public.
3. Legacy / internal examples
   These are retained temporarily for regression coverage, parity checks, or older APIs, but are not part of the recommended learning path.

Target size:

- 10-12 canonical examples
- 8-10 advanced examples
- everything else merged, archived, or removed

## Canonical Keepers

- `examples/basics/primitives_showcase.rs`
- `examples/basics/layout_playground.rs`
- `examples/animation/animation_parity.rs`
- `examples/animation/semantic_animation_showcase.rs`
- `examples/text_and_math/stem_showcase.rs`
- `examples/graphs_and_fields/vector_field_and_streamlines.rs`
- `examples/dynamics/particle_nebula_showcase.rs`
- `examples/three_d/textured_surface_showcase.rs`
- `examples/stepwise_showcase.rs`
- `examples/branding_and_export/screenshot_markers.rs`

## Advanced Keepers

- `examples/basics/styling_showcase.rs`
- `examples/basics/arrow_showcase.rs`
- `examples/graphs_and_fields/map_projection_morph.rs`
- `examples/text_and_math/formula_shape_formula_morph.rs`
- `examples/text_and_math/fourier_formula_trace.rs`
- `examples/dynamics/swaying_points.rs`
- `examples/dynamics/traced_path_rolling_circle.rs`
- `examples/branding_and_export/blog_showcase.rs`
- `examples/branding_and_export/murali_logo.rs`

## Legacy / Internal Keepers

These should stay available for now, but should not be presented as the preferred path:

- `examples/ai_and_storytelling/agentic_flow_with_neural_node.rs`
- `examples/ai_and_storytelling/agentic_flow_chart.rs`
- `examples/three_d/manim_sector_fill_demo.rs`

## Merge Plan

### Text and path animation

Merge:

- `examples/animation/write_effect_showcase.rs`
- `examples/animation/unwrite_showcase.rs`
- `examples/animation/text_write_effect_showcase.rs`
- `examples/animation/text_reveal_effects_showcase.rs`
- `examples/animation/text_indicate_showcase.rs`

Into:

- `examples/animation/text_and_path_animation_showcase.rs`

### Morphing

Merge:

- `examples/animation/morph_showcase.rs`
- `examples/animation/morph_and_move.rs`

Into:

- `examples/animation/morph_showcase.rs`

### Vector fields and streamlines

Keep:

- `examples/graphs_and_fields/vector_field_and_streamlines.rs`

Absorb useful material from:

- `examples/graphs_and_fields/vector_field_showcase.rs`
- `examples/graphs_and_fields/stream_lines_showcase.rs`

### Stepwise

Keep:

- `examples/stepwise_showcase.rs`

Retire after migration:

- `examples/ai_and_storytelling/stepwise.rs`
- `examples/ai_and_storytelling/stepwise_script_api.rs`
- `examples/ai_and_storytelling/stepwise_backpath.rs`
- `examples/ai_and_storytelling/stepwise_neural_node.rs`

### Tables

Merge:

- `examples/text_and_math/table_showcase.rs`
- `examples/text_and_math/table_simple.rs`

Into:

- `examples/text_and_math/table_showcase.rs`

### Force fields

Merge:

- `examples/dynamics/force_field_with_updaters.rs`
- `examples/dynamics/force_field_multiple_charges.rs`

Into:

- `examples/dynamics/force_field_showcase.rs`

### Parametric surfaces

Merge:

- `examples/three_d/parametric_surface_showcase.rs`
- `examples/three_d/parametric_surface_animated.rs`
- `examples/three_d/parametric_surface_advanced.rs`

Into:

- `examples/three_d/parametric_surface_showcase.rs`

### Wireframe surfaces

Merge:

- `examples/three_d/parametric_surface_wireframe.rs`
- `examples/three_d/parametric_surface_wireframe_animated.rs`
- `examples/three_d/parametric_surface_wireframe_showcase.rs`

Into:

- `examples/three_d/parametric_surface_wireframe_showcase.rs`

## Retirement Candidates

These should be removed after merged replacements land and are verified:

- `examples/ai_and_storytelling/agentic_flow_chart.rs`
- `examples/ai_and_storytelling/agentic_flow_write_animation.rs`
- `examples/ai_and_storytelling/agentic_reveal_showcase.rs`
- `examples/ai_and_storytelling/stepwise.rs`
- `examples/ai_and_storytelling/stepwise_script_api.rs`
- `examples/ai_and_storytelling/stepwise_backpath.rs`
- `examples/ai_and_storytelling/stepwise_neural_node.rs`
- `examples/text_and_math/table_simple.rs`
- `examples/graphs_and_fields/vector_field_showcase.rs`
- `examples/graphs_and_fields/stream_lines_showcase.rs`
- `examples/animation/morph_and_move.rs`
- `examples/animation/unwrite_showcase.rs`
- `examples/animation/text_write_effect_showcase.rs`
- `examples/animation/text_reveal_effects_showcase.rs`
- `examples/animation/text_indicate_showcase.rs`
- `examples/dynamics/force_field_with_updaters.rs`
- `examples/dynamics/force_field_multiple_charges.rs`
- `examples/three_d/parametric_surface_animated.rs`
- `examples/three_d/parametric_surface_advanced.rs`
- `examples/three_d/parametric_surface_wireframe.rs`
- `examples/three_d/parametric_surface_wireframe_animated.rs`

Possible additional retirements after review:

- `examples/text_and_math/fourier_simple_trace.rs`
- `examples/basics/shapes_extended.rs`
- `examples/branding_and_export/aiu_attention_template.rs`
- `examples/branding_and_export/export_aiu_attention.rs`

## README Direction

`examples/README.md` should move toward:

- `Start Here`
- `Canonical Examples`
- `Advanced Examples`
- `Legacy / Experimental`
- `Regression / Internal` if needed

It should stop presenting overlapping examples as equal first-class entry points.

## Rollout Strategy

1. Save the plan in-repo and update `examples/README.md`.
2. Create merged showcase files.
3. Update the README to point to merged examples.
4. Delete superseded examples only after the replacements are tested.
