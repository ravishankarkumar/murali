use glam::{Vec2, Vec3, Vec4, vec2};

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Mesh, Project, ProjectionCtx, RenderPrimitive};

#[derive(Debug, Clone)]
pub struct SignalFlow {
    pub path_points: Vec<Vec3>,
    pub progress: f32,
    pub edge_color: Vec4,
    pub pulse_color: Vec4,
    pub node_color: Vec4,
    pub edge_thickness: f32,
    pub pulse_radius: f32,
    pub node_radius: f32,
    pub highlight_nodes: bool,
}

impl SignalFlow {
    pub fn new(path_points: Vec<Vec3>) -> Self {
        Self {
            path_points,
            progress: 0.0,
            edge_color: Vec4::new(0.98, 0.84, 0.32, 0.95),
            pulse_color: Vec4::new(1.0, 0.96, 0.72, 1.0),
            node_color: Vec4::new(0.98, 0.92, 0.58, 0.95),
            edge_thickness: 0.04,
            pulse_radius: 0.11,
            node_radius: 0.09,
            highlight_nodes: true,
        }
    }

    pub fn with_edge_color(mut self, color: Vec4) -> Self {
        self.edge_color = color;
        self
    }

    pub fn with_pulse_color(mut self, color: Vec4) -> Self {
        self.pulse_color = color;
        self
    }

    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn current_position(&self) -> Option<Vec3> {
        if self.path_points.is_empty() {
            return None;
        }
        if self.path_points.len() == 1 {
            return self.path_points.first().copied();
        }

        let segment_count = self.path_points.len() - 1;
        let scaled = self.progress.clamp(0.0, 1.0) * segment_count as f32;
        let current_idx = scaled.floor() as usize;
        let segment_t = (scaled - current_idx as f32).clamp(0.0, 1.0);

        if current_idx >= segment_count {
            return self.path_points.last().copied();
        }

        let start = self.path_points[current_idx];
        let end = self.path_points[current_idx + 1];
        Some(start.lerp(end, segment_t))
    }
}

impl Project for SignalFlow {
    fn project(&self, ctx: &mut ProjectionCtx) {
        if self.path_points.len() < 2 {
            return;
        }

        let segment_count = self.path_points.len() - 1;
        let scaled = self.progress.clamp(0.0, 1.0) * segment_count as f32;
        let current_idx = scaled.floor() as usize;
        let segment_t = (scaled - current_idx as f32).clamp(0.0, 1.0);

        for idx in 0..segment_count {
            let start = self.path_points[idx];
            let end = self.path_points[idx + 1];

            if idx < current_idx {
                ctx.emit(RenderPrimitive::Line {
                    start,
                    end,
                    thickness: self.edge_thickness,
                    color: self.edge_color,
                    dash_length: 0.0,
                    gap_length: 0.0,
                    dash_offset: 0.0,
                });
            } else if idx == current_idx && self.progress < 1.0 {
                ctx.emit(RenderPrimitive::Line {
                    start,
                    end: start.lerp(end, segment_t),
                    thickness: self.edge_thickness,
                    color: self.edge_color,
                    dash_length: 0.0,
                    gap_length: 0.0,
                    dash_offset: 0.0,
                });
            }
        }

        if self.highlight_nodes {
            let active_node_count = if self.progress >= 1.0 {
                self.path_points.len()
            } else {
                (current_idx + 1).min(self.path_points.len())
            };

            for point in self.path_points.iter().take(active_node_count) {
                let mesh = Mesh::circle(self.node_radius, 24, self.node_color)
                    .as_ref()
                    .translated(*point);
                ctx.emit(RenderPrimitive::Mesh(mesh));
            }
        }

        if let Some(pulse_pos) = self.current_position() {
            let mesh = Mesh::circle(self.pulse_radius, 28, self.pulse_color)
                .as_ref()
                .translated(pulse_pos);
            ctx.emit(RenderPrimitive::Mesh(mesh));
        }
    }
}

impl Bounded for SignalFlow {
    fn local_bounds(&self) -> Bounds {
        if self.path_points.is_empty() {
            return Bounds::from_center_size(Vec2::ZERO, vec2(0.01, 0.01));
        }

        let mut min = self.path_points[0].truncate();
        let mut max = self.path_points[0].truncate();
        for point in &self.path_points {
            min = min.min(point.truncate());
            max = max.max(point.truncate());
        }
        let pad = self.pulse_radius.max(self.node_radius) + self.edge_thickness;
        Bounds::new(min - vec2(pad, pad), max + vec2(pad, pad))
    }
}
