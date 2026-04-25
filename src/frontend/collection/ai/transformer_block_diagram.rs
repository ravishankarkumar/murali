use glam::{Vec2, Vec3, Vec4, vec2};

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};

#[derive(Debug, Clone)]
pub struct TransformerBlockDiagram {
    pub width: f32,
    pub block_height: f32,
    pub gap: f32,
    pub line_thickness: f32,
    pub frame_color: Vec4,
    pub accent_color: Vec4,
    pub text_color: Vec4,
}

impl TransformerBlockDiagram {
    pub fn new() -> Self {
        Self {
            width: 3.0,
            block_height: 0.65,
            gap: 0.22,
            line_thickness: 0.03,
            frame_color: Vec4::new(0.86, 0.90, 0.95, 1.0),
            accent_color: Vec4::new(0.45, 0.78, 0.98, 1.0),
            text_color: Vec4::new(0.95, 0.97, 0.99, 1.0),
        }
    }

    fn block_names(&self) -> [&'static str; 4] {
        ["Multi-Head Attention", "Add & Norm", "MLP", "Add & Norm"]
    }

    fn draw_box(&self, ctx: &mut ProjectionCtx, center: Vec2, size: Vec2, color: Vec4) {
        let left = center.x - size.x * 0.5;
        let right = center.x + size.x * 0.5;
        let top = center.y + size.y * 0.5;
        let bottom = center.y - size.y * 0.5;
        for (a, b) in [
            (Vec3::new(left, bottom, 0.0), Vec3::new(right, bottom, 0.0)),
            (Vec3::new(right, bottom, 0.0), Vec3::new(right, top, 0.0)),
            (Vec3::new(right, top, 0.0), Vec3::new(left, top, 0.0)),
            (Vec3::new(left, top, 0.0), Vec3::new(left, bottom, 0.0)),
        ] {
            ctx.emit(RenderPrimitive::Line {
                start: a,
                end: b,
                thickness: self.line_thickness,
                color,
                dash_length: 0.0,
                gap_length: 0.0,
                dash_offset: 0.0,
            });
        }
    }
}

impl Project for TransformerBlockDiagram {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let names = self.block_names();
        let total_height = names.len() as f32 * self.block_height
            + (names.len().saturating_sub(1) as f32) * self.gap;
        let top = total_height * 0.5 - self.block_height * 0.5;
        let inner_width = self.width * 0.86;

        for (idx, name) in names.iter().enumerate() {
            let y = top - idx as f32 * (self.block_height + self.gap);
            self.draw_box(
                ctx,
                vec2(0.0, y),
                vec2(inner_width, self.block_height),
                if idx % 2 == 0 {
                    self.accent_color
                } else {
                    self.frame_color
                },
            );

            let label_h = self.block_height * 0.32;
            let layout = crate::resource::text::layout::measure_label(name, label_h);
            let final_h = if layout.width > inner_width * 0.92 {
                label_h * (inner_width * 0.92 / layout.width)
            } else {
                label_h
            };

            ctx.emit(RenderPrimitive::Text {
                content: (*name).to_string(),
                height: final_h,
                color: self.text_color,
                offset: Vec3::new(0.0, y, 0.0),
                rotation: 0.0,
            });
            if idx + 1 < names.len() {
                let next_y = top - (idx + 1) as f32 * (self.block_height + self.gap);
                ctx.emit(RenderPrimitive::Line {
                    start: Vec3::new(0.0, y - self.block_height * 0.5, 0.0),
                    end: Vec3::new(0.0, next_y + self.block_height * 0.5, 0.0),
                    thickness: self.line_thickness,
                    color: self.frame_color,
                    dash_length: 0.0,
                    gap_length: 0.0,
                    dash_offset: 0.0,
                });
            }
        }

        ctx.emit(RenderPrimitive::Text {
            content: "Input Tokens".to_string(),
            height: 0.22,
            color: self.text_color,
            offset: Vec3::new(0.0, top + self.block_height * 0.5 + 0.45, 0.0),
            rotation: 0.0,
        });
        ctx.emit(RenderPrimitive::Text {
            content: "Output States".to_string(),
            height: 0.22,
            color: self.text_color,
            offset: Vec3::new(0.0, -top - self.block_height * 0.5 - 0.45, 0.0),
            rotation: 0.0,
        });
    }
}

impl Bounded for TransformerBlockDiagram {
    fn local_bounds(&self) -> Bounds {
        let total_height = self.block_names().len() as f32 * self.block_height
            + (self.block_names().len().saturating_sub(1) as f32) * self.gap
            + self.block_height * 1.8;
        Bounds::from_center_size(Vec2::ZERO, vec2(self.width, total_height))
    }
}
