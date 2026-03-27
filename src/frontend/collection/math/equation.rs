use glam::{Vec2, Vec3, Vec4};

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::resource::text::layout::measure_label;

#[derive(Debug, Clone)]
pub struct EquationPart {
    pub text: String,
    pub color: Vec4,
    pub key: Option<String>,
    pub opacity: f32,
    pub scale: f32,
    pub offset: Vec3,
}

impl EquationPart {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Vec4::ONE,
            key: None,
            opacity: 1.0,
            scale: 1.0,
            offset: Vec3::ZERO,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale.max(0.05);
        self
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }

    pub fn continuity_key(&self, index: usize) -> String {
        self.key
            .clone()
            .unwrap_or_else(|| format!("{}#{index}", self.text))
    }
}

#[derive(Debug, Clone)]
pub struct EquationPartLayout {
    pub index: usize,
    pub key: String,
    pub text: String,
    pub center: Vec3,
    pub width: f32,
    pub height: f32,
    pub color: Vec4,
    pub opacity: f32,
    pub scale: f32,
}

#[derive(Debug, Clone)]
pub struct EquationLayout {
    pub parts: Vec<EquationPart>,
    pub world_height: f32,
    pub gap: f32,
}

impl EquationLayout {
    pub fn new(parts: Vec<EquationPart>, world_height: f32) -> Self {
        Self {
            parts,
            world_height,
            gap: world_height * 0.35,
        }
    }

    fn part_width(&self, text: &str, scale: f32) -> f32 {
        measure_label(text, self.world_height * scale.max(0.05))
            .width
            .max(self.world_height * scale.max(0.05) * 0.4)
    }

    pub fn layout_snapshot(&self) -> Vec<EquationPartLayout> {
        let widths: Vec<f32> = self
            .parts
            .iter()
            .map(|p| self.part_width(&p.text, p.scale))
            .collect();
        let total_width =
            widths.iter().sum::<f32>() + self.gap * self.parts.len().saturating_sub(1) as f32;
        let mut cursor = -total_width * 0.5;
        let mut out = Vec::with_capacity(self.parts.len());

        for (index, (part, width)) in self.parts.iter().zip(widths).enumerate() {
            let base_center = Vec3::new(cursor + width * 0.5, 0.0, 0.0);
            let height = self.world_height * part.scale.max(0.05);
            out.push(EquationPartLayout {
                index,
                key: part.continuity_key(index),
                text: part.text.clone(),
                center: base_center + part.offset,
                width,
                height,
                color: part.color,
                opacity: part.opacity.clamp(0.0, 1.0),
                scale: part.scale.max(0.05),
            });
            cursor += width + self.gap;
        }

        out
    }
}

impl Project for EquationLayout {
    fn project(&self, ctx: &mut ProjectionCtx) {
        for part in self.layout_snapshot() {
            let mut color = part.color;
            color.w *= part.opacity;
            ctx.emit(RenderPrimitive::Text {
                content: part.text,
                height: part.height,
                color,
                offset: part.center,
            });
        }
    }
}

impl Bounded for EquationLayout {
    fn local_bounds(&self) -> Bounds {
        let layout = self.layout_snapshot();
        let mut min = Vec2::splat(f32::INFINITY);
        let mut max = Vec2::splat(f32::NEG_INFINITY);
        for part in layout {
            let b = Bounds::from_center_size(
                Vec2::new(part.center.x, part.center.y),
                Vec2::new(part.width, part.height),
            );
            min.x = min.x.min(b.min.x);
            min.y = min.y.min(b.min.y);
            max.x = max.x.max(b.max.x);
            max.y = max.y.max(b.max.y);
        }

        if !min.is_finite() || !max.is_finite() {
            Bounds::default()
        } else {
            Bounds::new(min, max)
        }
    }
}
