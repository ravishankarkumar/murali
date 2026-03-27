use glam::{vec2, vec3, Vec2, Vec4};
use crate::frontend::layout::{Bounded, Bounds};
use crate::math::bezier::{quadratic_bezier, cubic_bezier};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};

#[derive(Debug, Clone, Copy)]
pub enum PathSegment {
    MoveTo(Vec2),
    LineTo(Vec2),
    QuadTo(Vec2, Vec2),     // control, end
    CubicTo(Vec2, Vec2, Vec2), // control1, control2, end
}

/// A Tattva for drawing complex paths consisting of lines and Bézier curves.
#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub thickness: f32,
    pub color: Vec4,
    pub closed: bool,
}

impl Path {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            thickness: 0.05,
            color: Vec4::ONE,
            closed: false,
        }
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn move_to(mut self, p: Vec2) -> Self {
        self.segments.push(PathSegment::MoveTo(p));
        self
    }

    pub fn line_to(mut self, p: Vec2) -> Self {
        self.segments.push(PathSegment::LineTo(p));
        self
    }

    pub fn quad_to(mut self, ctrl: Vec2, end: Vec2) -> Self {
        self.segments.push(PathSegment::QuadTo(ctrl, end));
        self
    }

    pub fn cubic_to(mut self, ctrl1: Vec2, ctrl2: Vec2, end: Vec2) -> Self {
        self.segments.push(PathSegment::CubicTo(ctrl1, ctrl2, end));
        self
    }
}

impl Project for Path {
    fn project(&self, ctx: &mut ProjectionCtx) {
        if self.segments.is_empty() {
            return;
        }

        let mut current_point = vec2(0.0, 0.0);
        let mut first_point = None;

        for segment in &self.segments {
            match *segment {
                PathSegment::MoveTo(p) => {
                    current_point = p;
                    if first_point.is_none() {
                        first_point = Some(p);
                    }
                }
                PathSegment::LineTo(p) => {
                    ctx.emit(RenderPrimitive::Line {
                        start: vec3(current_point.x, current_point.y, 0.0),
                        end: vec3(p.x, p.y, 0.0),
                        thickness: self.thickness,
                        color: self.color,
                    });
                    current_point = p;
                }
                PathSegment::QuadTo(ctrl, end) => {
                    let steps = 16;
                    for i in 0..steps {
                        let t0 = i as f32 / steps as f32;
                        let t1 = (i + 1) as f32 / steps as f32;
                        let p0 = quadratic_bezier(current_point, ctrl, end, t0);
                        let p1 = quadratic_bezier(current_point, ctrl, end, t1);
                        ctx.emit(RenderPrimitive::Line {
                            start: vec3(p0.x, p0.y, 0.0),
                            end: vec3(p1.x, p1.y, 0.0),
                            thickness: self.thickness,
                            color: self.color,
                        });
                    }
                    current_point = end;
                }
                PathSegment::CubicTo(ctrl1, ctrl2, end) => {
                    let steps = 24;
                    for i in 0..steps {
                        let t0 = i as f32 / steps as f32;
                        let t1 = (i + 1) as f32 / steps as f32;
                        let p0 = cubic_bezier(current_point, ctrl1, ctrl2, end, t0);
                        let p1 = cubic_bezier(current_point, ctrl1, ctrl2, end, t1);
                        ctx.emit(RenderPrimitive::Line {
                            start: vec3(p0.x, p0.y, 0.0),
                            end: vec3(p1.x, p1.y, 0.0),
                            thickness: self.thickness,
                            color: self.color,
                        });
                    }
                    current_point = end;
                }
            }
        }

        if self.closed {
            if let Some(first) = first_point {
                if (current_point - first).length() > 0.001 {
                    ctx.emit(RenderPrimitive::Line {
                        start: vec3(current_point.x, current_point.y, 0.0),
                        end: vec3(first.x, first.y, 0.0),
                        thickness: self.thickness,
                        color: self.color,
                    });
                }
            }
        }
    }
}

impl Bounded for Path {
    fn local_bounds(&self) -> Bounds {
        if self.segments.is_empty() {
            return Bounds::default();
        }

        let mut min = vec2(f32::MAX, f32::MAX);
        let mut max = vec2(f32::MIN, f32::MIN);

        let mut update_bounds = |p: Vec2| {
            min = vec2(min.x.min(p.x), min.y.min(p.y));
            max = vec2(max.x.max(p.x), max.y.max(p.y));
        };

        for segment in &self.segments {
            match *segment {
                PathSegment::MoveTo(p) => {
                    update_bounds(p);
                }
                PathSegment::LineTo(p) => {
                    update_bounds(p);
                }
                PathSegment::QuadTo(ctrl, end) => {
                    // For simplicity, we just include control points in the bounds.
                    // A tighter bound would involve finding the extrema of the Bézier curve.
                    update_bounds(ctrl);
                    update_bounds(end);
                }
                PathSegment::CubicTo(ctrl1, ctrl2, end) => {
                    update_bounds(ctrl1);
                    update_bounds(ctrl2);
                    update_bounds(end);
                }
            }
        }

        Bounds::new(min, max)
    }
}
