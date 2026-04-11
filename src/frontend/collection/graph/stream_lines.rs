// src/frontend/collection/graph/stream_lines.rs
//! Streamline visualization for vector fields
//! Shows flow paths that are tangent to the vector field at every point

use glam::{Vec2, Vec3, Vec4};
use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use std::sync::Arc;

/// Streamlines show the paths that particles would follow in a vector field
/// Each streamline is a curve that is tangent to the vector field at every point
pub struct StreamLines {
    /// Starting points for streamlines
    pub start_points: Vec<Vec2>,
    /// Function that returns a vector for a given position
    pub field_fn: Arc<dyn Fn(Vec2) -> Vec2 + Send + Sync>,
    /// Maximum number of steps to trace each streamline
    pub max_steps: usize,
    /// Step size for integration (smaller = more accurate but more points)
    pub step_size: f32,
    /// Color of the streamlines
    pub color: Vec4,
    /// Line thickness
    pub thickness: f32,
    /// Optional function to color streamlines based on position or magnitude
    pub color_fn: Option<Arc<dyn Fn(Vec2, f32) -> Vec4 + Send + Sync>>,
    /// Bounds to constrain streamlines
    pub bounds: Option<(Vec2, Vec2)>,
}

impl StreamLines {
    /// Create new streamlines from a set of starting points
    pub fn new<F>(start_points: Vec<Vec2>, field_fn: F) -> Self
    where
        F: Fn(Vec2) -> Vec2 + Send + Sync + 'static,
    {
        Self {
            start_points,
            field_fn: Arc::new(field_fn),
            max_steps: 1000,
            step_size: 0.05,
            color: Vec4::new(0.5, 0.7, 1.0, 0.8),
            thickness: 0.03,
            color_fn: None,
            bounds: None,
        }
    }

    /// Create streamlines from a grid of starting points
    pub fn from_grid<F>(
        x_range: (f32, f32),
        y_range: (f32, f32),
        x_count: usize,
        y_count: usize,
        field_fn: F,
    ) -> Self
    where
        F: Fn(Vec2) -> Vec2 + Send + Sync + 'static,
    {
        let mut start_points = Vec::new();
        let dx = (x_range.1 - x_range.0) / (x_count - 1) as f32;
        let dy = (y_range.1 - y_range.0) / (y_count - 1) as f32;

        for i in 0..x_count {
            for j in 0..y_count {
                let x = x_range.0 + i as f32 * dx;
                let y = y_range.0 + j as f32 * dy;
                start_points.push(Vec2::new(x, y));
            }
        }

        Self::new(start_points, field_fn)
    }

    /// Set the color
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    /// Set a function to color streamlines based on position and magnitude
    pub fn with_color_fn<F>(mut self, color_fn: F) -> Self
    where
        F: Fn(Vec2, f32) -> Vec4 + Send + Sync + 'static,
    {
        self.color_fn = Some(Arc::new(color_fn));
        self
    }

    /// Set the line thickness
    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the step size for integration
    pub fn with_step_size(mut self, step_size: f32) -> Self {
        self.step_size = step_size;
        self
    }

    /// Set the maximum number of steps
    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }

    /// Set bounds to constrain streamlines
    pub fn with_bounds(mut self, min: Vec2, max: Vec2) -> Self {
        self.bounds = Some((min, max));
        self
    }

    /// Check if a point is within bounds
    fn in_bounds(&self, pos: Vec2) -> bool {
        if let Some((min, max)) = self.bounds {
            pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y
        } else {
            true
        }
    }

    /// Trace a single streamline using Euler integration
    fn trace_streamline(&self, start: Vec2) -> Vec<Vec2> {
        let mut points = vec![start];
        let mut current = start;

        for _ in 0..self.max_steps {
            // Get the vector at the current position
            let vector = (self.field_fn)(current);
            let magnitude = vector.length();

            // Stop if the vector is too small (stagnation point)
            if magnitude < 1e-6 {
                break;
            }

            // Normalize and scale by step size
            let step = vector.normalize() * self.step_size;
            let next = current + step;

            // Stop if we go out of bounds
            if !self.in_bounds(next) {
                break;
            }

            points.push(next);
            current = next;
        }

        points
    }
}

impl Bounded for StreamLines {
    fn local_bounds(&self) -> Bounds {
        if let Some((min, max)) = self.bounds {
            Bounds::new(min, max)
        } else {
            // Calculate bounds from start points
            let mut min_x = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut min_y = f32::INFINITY;
            let mut max_y = f32::NEG_INFINITY;

            for point in &self.start_points {
                min_x = min_x.min(point.x);
                max_x = max_x.max(point.x);
                min_y = min_y.min(point.y);
                max_y = max_y.max(point.y);
            }

            Bounds::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
        }
    }
}

impl Project for StreamLines {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // Trace each streamline
        for start_point in &self.start_points {
            let points = self.trace_streamline(*start_point);

            // Draw the streamline as connected line segments
            for i in 0..points.len().saturating_sub(1) {
                let start = points[i];
                let end = points[i + 1];

                // Calculate color
                let color = if let Some(ref color_fn) = self.color_fn {
                    let vector = (self.field_fn)(start);
                    let magnitude = vector.length();
                    color_fn(start, magnitude)
                } else {
                    self.color
                };

                ctx.emit(RenderPrimitive::Line {
                    start: Vec3::new(start.x, start.y, 0.0),
                    end: Vec3::new(end.x, end.y, 0.0),
                    thickness: self.thickness,
                    color,
                    dash_length: 0.0,
                    gap_length: 0.0,
                    dash_offset: 0.0,
                });
            }
        }
    }
}

/// Helper function to create evenly spaced starting points along a line
pub fn line_start_points(start: Vec2, end: Vec2, count: usize) -> Vec<Vec2> {
    let mut points = Vec::new();
    for i in 0..count {
        let t = i as f32 / (count - 1) as f32;
        points.push(start.lerp(end, t));
    }
    points
}

/// Helper function to create starting points in a circle
pub fn circle_start_points(center: Vec2, radius: f32, count: usize) -> Vec<Vec2> {
    let mut points = Vec::new();
    for i in 0..count {
        let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Vec2::new(x, y));
    }
    points
}
