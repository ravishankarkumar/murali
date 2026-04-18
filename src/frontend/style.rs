pub use crate::projection::style::{ColorSource, StrokeParams};

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fill: Option<ColorSource>,
    pub stroke: Option<StrokeParams>,
}

impl Style {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_fill(mut self, color: impl Into<ColorSource>) -> Self {
        self.fill = Some(color.into());
        self
    }

    #[must_use]
    pub fn with_stroke(mut self, params: StrokeParams) -> Self {
        self.stroke = Some(params);
        self
    }

    #[must_use]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let fill = match (&self.fill, &other.fill) {
            (Some(f1), Some(f2)) => Some(f1.lerp(f2, t)),
            (None, Some(f)) => Some(f.clone()),
            (Some(f), None) => Some(f.clone()),
            (None, None) => None,
        };

        let stroke = match (&self.stroke, &other.stroke) {
            (Some(s1), Some(s2)) => Some(s1.lerp(s2, t)),
            (None, Some(s)) => Some(s.clone()),
            (Some(s), None) => Some(s.clone()),
            (None, None) => None,
        };

        Self { fill, stroke }
    }
}