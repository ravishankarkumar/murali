use glam::{Vec2, Vec3, Vec4, vec2};

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Mesh, Project, ProjectionCtx, RenderPrimitive};

#[derive(Debug, Clone)]
pub struct LabeledPoint {
    pub point: Vec2,
    pub class: usize,
}

#[derive(Clone)]
pub struct DecisionBoundaryPlot {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub points: Vec<LabeledPoint>,
    pub grid_resolution: usize,
    pub class_a_color: Vec4,
    pub class_b_color: Vec4,
    pub boundary_color: Vec4,
    pub point_radius: f32,
    pub classifier: fn(Vec2) -> f32,
}

impl DecisionBoundaryPlot {
    pub fn new(x_range: (f32, f32), y_range: (f32, f32), classifier: fn(Vec2) -> f32) -> Self {
        Self {
            x_range,
            y_range,
            points: Vec::new(),
            grid_resolution: 24,
            class_a_color: Vec4::new(0.24, 0.61, 0.93, 1.0),
            class_b_color: Vec4::new(0.93, 0.39, 0.45, 1.0),
            boundary_color: Vec4::new(0.96, 0.96, 0.98, 1.0),
            point_radius: 0.08,
            classifier,
        }
    }

    pub fn project_with_points(&self, ctx: &mut ProjectionCtx, points: &[LabeledPoint]) {
        let nx = self.grid_resolution.max(2);
        let ny = self.grid_resolution.max(2);
        let dx = (self.x_range.1 - self.x_range.0) / nx as f32;
        let dy = (self.y_range.1 - self.y_range.0) / ny as f32;

        for ix in 0..nx {
            for iy in 0..ny {
                let cx = self.x_range.0 + (ix as f32 + 0.5) * dx;
                let cy = self.y_range.0 + (iy as f32 + 0.5) * dy;
                let v = (self.classifier)(vec2(cx, cy));
                let color = if v >= 0.0 {
                    self.class_a_color
                } else {
                    self.class_b_color
                };
                let mesh = Mesh::square(dx.max(dy) * 0.95, color)
                    .as_ref()
                    .translated(Vec3::new(cx, cy, 0.0));
                ctx.emit(RenderPrimitive::Mesh(mesh));
            }
        }

        let steps = 120;
        let mut prev: Option<Vec2> = None;
        for ix in 0..=steps {
            for iy in 0..=steps {
                let x =
                    self.x_range.0 + (self.x_range.1 - self.x_range.0) * ix as f32 / steps as f32;
                let y =
                    self.y_range.0 + (self.y_range.1 - self.y_range.0) * iy as f32 / steps as f32;
                let v = (self.classifier)(vec2(x, y));
                if v.abs() < 0.04 {
                    if let Some(p) = prev {
                        ctx.emit(RenderPrimitive::Line {
                            start: Vec3::new(p.x, p.y, 0.0),
                            end: Vec3::new(x, y, 0.0),
                            thickness: 0.025,
                            color: self.boundary_color,
                            dash_length: 0.0,
                            gap_length: 0.0,
                            dash_offset: 0.0,
                        });
                    }
                    prev = Some(vec2(x, y));
                }
            }
        }

        for point in points {
            let color = if point.class == 0 {
                self.class_a_color
            } else {
                self.class_b_color
            };
            let mesh = Mesh::circle(self.point_radius, 20, color)
                .as_ref()
                .translated(Vec3::new(point.point.x, point.point.y, 0.0));
            ctx.emit(RenderPrimitive::Mesh(mesh));
        }
    }
}

impl Project for DecisionBoundaryPlot {
    fn project(&self, ctx: &mut ProjectionCtx) {
        self.project_with_points(ctx, &self.points);
    }
}

impl Bounded for DecisionBoundaryPlot {
    fn local_bounds(&self) -> Bounds {
        Bounds::new(
            vec2(self.x_range.0, self.y_range.0),
            vec2(self.x_range.1, self.y_range.1),
        )
    }
}
