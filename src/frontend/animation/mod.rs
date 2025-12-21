pub mod builder;
pub mod camera_animation;
pub mod camera_animation_builder;

use crate::engine::scene::Scene;
use crate::frontend::TattvaId;
use crate::frontend::props::DrawableProps;
use glam::Vec3;

/// Common easing curves for deterministic interpolation.
#[derive(Copy, Clone, Debug)]
pub enum Ease {
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
}

impl Ease {
    pub fn eval(&self, t: f32) -> f32 {
        match self {
            Ease::Linear => t,
            Ease::InQuad => t * t,
            Ease::OutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Ease::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

/// The core trait for all Frontend logic changes over time.
/// Every implementation must be deterministic.
pub trait Animation: Send + Sync {
    /// Initial capture of state (e.g., 'from' positions).
    fn on_start(&mut self, scene: &mut Scene);

    /// Apply interpolation at a normalized time [0, 1].
    /// The Timeline handles the conversion from absolute time to 0..1.
    fn apply_at(&mut self, scene: &mut Scene, t: f32);

    fn on_finish(&mut self, _scene: &mut Scene) {}
}

// ============================================================================
// Concrete Animation: MoveTo
// ============================================================================

/// Translates a Tattva's position property.
pub struct MoveTo {
    pub target_id: TattvaId,
    pub to: Vec3,
    pub duration: f32,
    pub ease: Ease,
    from: Option<Vec3>,
}

impl MoveTo {
    pub fn new(target_id: TattvaId, to: Vec3, duration: f32, ease: Ease) -> Self {
        Self {
            target_id,
            to,
            duration,
            ease,
            from: None,
        }
    }
}

impl Animation for MoveTo {
    fn on_start(&mut self, scene: &mut Scene) {
        // Capture initial position
        if let Some(tattva) = scene.get_tattva_any_mut(self.target_id) {
            let props = DrawableProps::read(tattva.props());
            self.from = Some(props.position);
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let k = self.ease.eval(t);
        let from = self.from.unwrap_or(self.to);
        let new_pos = from.lerp(self.to, k);

        if let Some(tattva) = scene.get_tattva_any_mut(self.target_id) {
            // let props = tattva.props();

            // Transform-only mutation.
            // This does NOT mark the Tattva dirty (no reprojection).
            let props = tattva.props();
            // let mut props = props.write();
            // let mut props = props.as_ref().write();
            let mut props = DrawableProps::write(props);
            props.position = new_pos;
        }
    }
}
