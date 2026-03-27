use glam::{vec2, Vec2, Vec3, Vec4};

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Mesh, Project, ProjectionCtx, RenderPrimitive};

#[derive(Debug, Clone)]
pub struct NeuralNetworkDiagram {
    pub layers: Vec<usize>,
    pub layer_spacing: f32,
    pub node_spacing: f32,
    pub node_radius: f32,
    pub node_color: Vec4,
    pub edge_color: Vec4,
    pub edge_thickness: f32,
    pub layer_labels: Option<Vec<String>>,
    pub activation: ActivationFunc,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivationFunc {
    None,
    ReLU,
    Sigmoid,
}

impl NeuralNetworkDiagram {
    pub fn new(layers: Vec<usize>) -> Self {
        Self {
            layers,
            layer_spacing: 1.8,
            node_spacing: 0.8,
            node_radius: 0.12,
            node_color: Vec4::new(0.36, 0.77, 0.98, 1.0),
            edge_color: Vec4::new(0.48, 0.56, 0.68, 1.0),
            edge_thickness: 0.015,
            layer_labels: None,
            activation: ActivationFunc::None,
        }
    }

    pub fn with_labels(mut self, labels: Vec<impl Into<String>>) -> Self {
        self.layer_labels = Some(labels.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn with_activation(mut self, func: ActivationFunc) -> Self {
        self.activation = func;
        self
    }

    fn layer_x(&self, idx: usize) -> f32 {
        let width = (self.layers.len().saturating_sub(1) as f32) * self.layer_spacing;
        -width * 0.5 + idx as f32 * self.layer_spacing
    }

    fn node_y(&self, count: usize, idx: usize) -> f32 {
        let height = (count.saturating_sub(1) as f32) * self.node_spacing;
        height * 0.5 - idx as f32 * self.node_spacing
    }

    pub fn node_position(&self, layer_idx: usize, node_idx: usize) -> Option<Vec3> {
        let count = *self.layers.get(layer_idx)?;
        if node_idx >= count {
            return None;
        }
        Some(Vec3::new(
            self.layer_x(layer_idx),
            self.node_y(count, node_idx),
            0.0,
        ))
    }

    pub fn path_points(&self, node_indices_per_layer: &[usize]) -> Option<Vec<Vec3>> {
        if node_indices_per_layer.len() != self.layers.len() || node_indices_per_layer.is_empty() {
            return None;
        }

        let mut points = Vec::with_capacity(node_indices_per_layer.len());
        for (layer_idx, node_idx) in node_indices_per_layer.iter().copied().enumerate() {
            points.push(self.node_position(layer_idx, node_idx)?);
        }
        Some(points)
    }

    fn project_activation(&self, ctx: &mut ProjectionCtx, x: f32, y: f32) {
        let size = 0.15;
        let color = Vec4::new(0.96, 0.74, 0.26, 1.0);
        let thickness = 0.02;

        match self.activation {
            ActivationFunc::ReLU => {
                // Draw L shape
                ctx.emit(RenderPrimitive::Line {
                    start: Vec3::new(x - size, y, 0.0),
                    end: Vec3::new(x, y, 0.0),
                    thickness,
                    color,
                });
                ctx.emit(RenderPrimitive::Line {
                    start: Vec3::new(x, y, 0.0),
                    end: Vec3::new(x + size, y + size, 0.0),
                    thickness,
                    color,
                });
            }
            ActivationFunc::Sigmoid => {
                // S curve approximation
                let steps = 8;
                for i in 0..steps {
                    let t0 = i as f32 / steps as f32;
                    let t1 = (i + 1) as f32 / steps as f32;
                    let x0 = x - size + t0 * 2.0 * size;
                    let x1 = x - size + t1 * 2.0 * size;
                    let y0 = y + (size * 2.0 / (1.0 + (-((t0 - 0.5) * 6.0)).exp())) - size;
                    let y1 = y + (size * 2.0 / (1.0 + (-((t1 - 0.5) * 6.0)).exp())) - size;

                    ctx.emit(RenderPrimitive::Line {
                        start: Vec3::new(x0, y0, 0.0),
                        end: Vec3::new(x1, y1, 0.0),
                        thickness,
                        color,
                    });
                }
            }
            ActivationFunc::None => {}
        }
    }
}

impl Project for NeuralNetworkDiagram {
    fn project(&self, ctx: &mut ProjectionCtx) {
        for layer_idx in 0..self.layers.len().saturating_sub(1) {
            let from_count = self.layers[layer_idx];
            let to_count = self.layers[layer_idx + 1];
            let x0 = self.layer_x(layer_idx);
            let x1 = self.layer_x(layer_idx + 1);
            for i in 0..from_count {
                for j in 0..to_count {
                    ctx.emit(RenderPrimitive::Line {
                        start: Vec3::new(x0, self.node_y(from_count, i), 0.0),
                        end: Vec3::new(x1, self.node_y(to_count, j), 0.0),
                        thickness: self.edge_thickness,
                        color: self.edge_color,
                    });
                }
            }
        }

        for (layer_idx, count) in self.layers.iter().copied().enumerate() {
            let x = self.layer_x(layer_idx);
            for node_idx in 0..count {
                let mesh = Mesh::circle(self.node_radius, 24, self.node_color)
                    .as_ref()
                    .translated(Vec3::new(x, self.node_y(count, node_idx), 0.0));
                ctx.emit(RenderPrimitive::Mesh(mesh));
            }

            // Draw layer label if present
            if let Some(labels) = &self.layer_labels {
                if let Some(label) = labels.get(layer_idx) {
                    let y_offset = self.node_y(count, 0) + self.node_spacing * 0.7;
                    ctx.emit(RenderPrimitive::Text {
                        content: label.clone(),
                        height: 0.22,
                        color: Vec4::new(0.9, 0.9, 0.9, 1.0),
                        offset: Vec3::new(x, y_offset, 0.0),
                    });
                }
            }

            // Draw activation icon if not None (only for hidden/output layers)
            if layer_idx > 0 && self.activation != ActivationFunc::None {
                let y_offset = self.node_y(count, count - 1) - self.node_spacing * 0.7;
                self.project_activation(ctx, x, y_offset);
            }
        }
    }
}

impl Bounded for NeuralNetworkDiagram {
    fn local_bounds(&self) -> Bounds {
        let width = (self.layers.len().saturating_sub(1) as f32) * self.layer_spacing + self.node_radius * 2.0;
        let max_nodes = self.layers.iter().copied().max().unwrap_or(1);
        let height = (max_nodes.saturating_sub(1) as f32) * self.node_spacing + self.node_radius * 2.0;
        Bounds::from_center_size(Vec2::ZERO, vec2(width.max(self.node_radius * 2.0), height.max(self.node_radius * 2.0)))
    }
}
