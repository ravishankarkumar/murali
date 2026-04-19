---
sidebar_position: 9
---

# AI Diagrams

AI diagram tattvas live under `murali::frontend::collection::ai`.

This family is for higher-level teaching visuals and model diagrams rather than generic geometric graphing.

## Included tattvas

- `AttentionMatrix` - heatmap-style token-to-token attention grid
- `SignalFlow` - animated pulse moving through one or more paths
- `NeuralNetworkDiagram` - layered node-and-edge network diagram
- `TokenSequence` - ordered token visualization
- `TransformerBlockDiagram` - transformer-style block composition
- `DecisionBoundaryPlot` - classifier-style 2D decision region plot
- `AgenticFlowChart` - routed flow-chart style diagrams (available, but not currently being actively evolved; prefer `Stepwise` for new storytelling-oriented flows)

## Examples

`AttentionMatrix` is useful when values matter cell-by-cell:

```rust
use murali::frontend::collection::ai::attention_matrix::AttentionMatrix;

scene.add_tattva(
    AttentionMatrix::new(
        vec![
            vec![1.0, 0.5, 0.1],
            vec![0.3, 0.9, 0.2],
            vec![0.1, 0.4, 0.8],
        ],
        Some(vec!["The".into(), "cat".into(), "sat".into()]),
    ),
    Vec3::ZERO,
);
```

`SignalFlow` is useful when motion along a route is the story:

```rust
use murali::frontend::collection::ai::signal_flow::SignalFlow;

scene.add_tattva(
    SignalFlow::new(vec![
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
    ]),
    Vec3::ZERO,
);
```

`NeuralNetworkDiagram` is useful when layer structure matters:

```rust
use murali::frontend::collection::ai::neural_network_diagram::NeuralNetworkDiagram;

scene.add_tattva(
    NeuralNetworkDiagram::new(vec![3, 5, 2])
        .with_labels(vec!["Input", "Hidden", "Output"]),
    Vec3::ZERO,
);
```

## Recommended doc shape

This section should eventually document each concrete AI tattva separately, because the authoring intent differs a lot between them. For now, this page serves as the family index so the docs match the shipped API surface.
