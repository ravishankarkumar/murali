use glam::{vec2, Vec2, Vec3, Vec4};

use crate::frontend::layout::{Bounded, Bounds};
use crate::resource::text::layout::measure_label;
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};

#[derive(Debug, Clone)]
pub struct TokenSequence {
    pub tokens: Vec<String>,
    pub token_height: f32,
    pub gap: f32,
    pub box_padding: Vec2,
    pub text_color: Vec4,
    pub box_color: Vec4,
    pub line_thickness: f32,
}

impl TokenSequence {
    pub fn new(tokens: Vec<impl Into<String>>, token_height: f32) -> Self {
        Self {
            tokens: tokens.into_iter().map(Into::into).collect(),
            token_height,
            gap: token_height * 0.45,
            box_padding: vec2(token_height * 0.35, token_height * 0.28),
            text_color: Vec4::new(0.97, 0.98, 0.99, 1.0),
            box_color: Vec4::new(0.42, 0.55, 0.86, 1.0),
            line_thickness: 0.02,
        }
    }

    fn token_size(&self, token: &str) -> Vec2 {
        let layout = measure_label(token, self.token_height);
        vec2(
            layout.width + self.box_padding.x * 2.0,
            layout.height + self.box_padding.y * 2.0,
        )
    }
}

impl Project for TokenSequence {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let sizes: Vec<Vec2> = self.tokens.iter().map(|t| self.token_size(t)).collect();
        let total_width = sizes.iter().map(|s| s.x).sum::<f32>() + self.gap * self.tokens.len().saturating_sub(1) as f32;
        let mut cursor = -total_width * 0.5;

        for (token, size) in self.tokens.iter().zip(sizes) {
            let center_x = cursor + size.x * 0.5;
            let left = center_x - size.x * 0.5;
            let right = center_x + size.x * 0.5;
            let top = size.y * 0.5;
            let bottom = -size.y * 0.5;

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
                    color: self.box_color,
                });
            }

            ctx.emit(RenderPrimitive::Text {
                content: token.clone(),
                height: self.token_height,
                color: self.text_color,
                offset: Vec3::new(center_x, 0.0, 0.0),
            });

            cursor += size.x + self.gap;
        }
    }
}

impl Bounded for TokenSequence {
    fn local_bounds(&self) -> Bounds {
        let sizes: Vec<Vec2> = self.tokens.iter().map(|t| self.token_size(t)).collect();
        let total_width = sizes.iter().map(|s| s.x).sum::<f32>() + self.gap * self.tokens.len().saturating_sub(1) as f32;
        let max_height = sizes.iter().map(|s| s.y).fold(0.0, f32::max);
        Bounds::from_center_size(Vec2::ZERO, vec2(total_width.max(0.1), max_height.max(0.1)))
    }
}
