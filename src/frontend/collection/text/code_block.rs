use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use glam::Vec4;

/// A Tattva for displaying syntax-highlighted code blocks.
/// Leverages the Typst backend for high-quality rendering and highlighting.
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub code: String,
    pub language: String,
    pub world_height: f32,
    pub color: Vec4,
}

impl CodeBlock {
    pub fn new(code: impl Into<String>, language: impl Into<String>, world_height: f32) -> Self {
        Self {
            code: code.into(),
            language: language.into(),
            world_height,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    /// Builder-style color setter
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

impl Project for CodeBlock {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We wrap the code in a Typst raw block.
        // The Sync Boundary handles the actual compilation/rasterization.
        let escaped_code = self.code.replace("\\", "\\\\").replace("\"", "\\\"");

        let typst_source = format!(
            "#raw(\"{}\", lang: \"{}\", block: true)",
            escaped_code, self.language
        );

        ctx.emit(RenderPrimitive::Typst {
            source: typst_source,
            height: self.world_height,
            color: self.color,
            offset: glam::Vec3::ZERO,
        });
    }
}

impl Bounded for CodeBlock {
    fn local_bounds(&self) -> Bounds {
        // Estimate bounds based on line count and max width.
        // Typst's auto-paging will handle the actual layout, but we need
        // a reasonable bounding box for alignment/camera framing.
        let lines = self.code.lines().count().max(1) as f32;
        let max_line_len = self
            .code
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0) as f32;

        let char_width_ratio = 0.6; // Mono-space estimate
        let line_height_ratio = 1.3;

        let width = max_line_len * self.world_height * char_width_ratio;
        let height = lines * self.world_height * line_height_ratio;

        Bounds::from_center_size(glam::Vec2::ZERO, glam::vec2(width, height))
    }
}
