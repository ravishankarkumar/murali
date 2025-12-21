use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use glam::{Vec3, Vec4};
use std::f32::consts::TAU;

pub struct Circle {
    pub radius: f32,
    pub color: Vec4,
    pub segments: u32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            color: Vec4::ONE, // White by default
            segments: 64,     // Enough to look smooth
        }
    }
}

impl Project for Circle {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // A circle is projected as a series of connected line segments
        for i in 0..self.segments {
            let angle_start = (i as f32 / self.segments as f32) * TAU;
            let angle_end = ((i + 1) as f32 / self.segments as f32) * TAU;

            let start = Vec3::new(
                self.radius * angle_start.cos(),
                self.radius * angle_start.sin(),
                0.0,
            );

            let end = Vec3::new(
                self.radius * angle_end.cos(),
                self.radius * angle_end.sin(),
                0.0,
            );

            ctx.add(RenderPrimitive::Line {
                start,
                end,
                thickness: 0.05, // You can make this a state variable later
                color: self.color,
            });
        }
    }
}