use glam::{Vec2, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepwiseDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Layout parameters for a `Stepwise` tattva.
/// Shared between position calculation and bounds computation.
#[derive(Debug, Clone)]
pub struct StepwiseLayout {
    pub spacing:   f32,
    pub direction: StepwiseDirection,
}

impl Default for StepwiseLayout {
    fn default() -> Self {
        Self { spacing: 1.0, direction: StepwiseDirection::Horizontal }
    }
}

impl StepwiseLayout {
    pub fn horizontal(spacing: f32) -> Self {
        Self { spacing, direction: StepwiseDirection::Horizontal }
    }

    pub fn vertical(spacing: f32) -> Self {
        Self { spacing, direction: StepwiseDirection::Vertical }
    }

    /// World-space position for the node at `index`.
    pub fn position_for(&self, index: usize) -> Vec3 {
        let offset = index as f32 * self.spacing;
        match self.direction {
            StepwiseDirection::Horizontal => Vec3::new(offset, 0.0, 0.0),
            StepwiseDirection::Vertical   => Vec3::new(0.0, -offset, 0.0),
        }
    }

    /// Interpolated 2D position between two step indices (for signal dot).
    pub fn lerp_position(&self, from: usize, to: usize, t: f32) -> Vec2 {
        let a = self.position_for(from);
        let b = self.position_for(to);
        Vec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
    }
}
