use glam::{Vec2, Vec4};

#[derive(Debug, Clone)]
pub enum ColorSource {
    Solid(Vec4),
    LinearGradient {
        start: Vec2,
        end: Vec2,
        stops: Vec<(f32, Vec4)>,
    },
}

impl ColorSource {
    #[must_use]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (ColorSource::Solid(v1), ColorSource::Solid(v2)) => {
                ColorSource::Solid(v1.lerp(*v2, t))
            }

            // Fallback for mismatched types (e.g., solid ↔ gradient)
            (_, _) => {
                if t < 0.5 {
                    self.clone()
                } else {
                    other.clone()
                }
            }
        }
    }

    pub fn solid(color: Vec4) -> Self {
        Self::Solid(color)
    }

    pub fn linear_gradient(start: Vec2, end: Vec2, stops: Vec<(f32, Vec4)>) -> Self {
        Self::LinearGradient { start, end, stops }
    }
}

impl Default for ColorSource {
    fn default() -> Self {
        Self::Solid(Vec4::ONE)
    }
}

impl From<Vec4> for ColorSource {
    fn from(v: Vec4) -> Self {
        Self::Solid(v)
    }
}

impl From<&Vec4> for ColorSource {
    fn from(v: &Vec4) -> Self {
        Self::Solid(*v)
    }
}

#[derive(Debug, Clone)]
pub struct StrokeParams {
    pub thickness: f32,
    pub color: Vec4,
    pub dash_length: f32,
    pub gap_length: f32,
    pub dash_offset: f32,
}

impl Default for StrokeParams {
    fn default() -> Self {
        Self {
            thickness: 0.05,
            color: Vec4::ONE,
            dash_length: 0.0,
            gap_length: 0.0,
            dash_offset: 0.0,
        }
    }
}

impl StrokeParams {
    #[must_use]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            thickness: self.thickness + (other.thickness - self.thickness) * t,
            color: self.color.lerp(other.color, t),
            dash_length: self.dash_length + (other.dash_length - self.dash_length) * t,
            gap_length: self.gap_length + (other.gap_length - self.gap_length) * t,
            dash_offset: self.dash_offset + (other.dash_offset - self.dash_offset) * t,
        }
    }
}