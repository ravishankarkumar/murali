// src/animation/mod.rs
//! Animation primitives and trait.
//!
//! IMPORTANT DESIGN NOTE:
//! ----------------------
//! Animations operate on DrawableProps via DrawableInstance.
//! Tattvas are immutable semantic objects; drawables own spatial state.

pub mod builder;
pub mod camera_animation;
pub mod camera_animation_builder;

use crate::scene::{Scene, TattvaId};
use glam::Vec3;

/// Easing enum with a few common curves.
#[derive(Copy, Clone, Debug)]
pub enum Ease {
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
}

impl Ease {
    /// Evaluate easing for t in [0,1].
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

/// Animation trait: deterministic animation evaluated at absolute scene time.
pub trait Animation: Send + Sync {
    /// Called once when animation starts.
    fn on_start(&mut self, scene: &mut Scene);

    /// Apply animation at absolute scene time.
    fn apply_at(&mut self, scene: &mut Scene, t_abs: f32);

    /// Optional hook when animation finishes.
    fn on_finish(&mut self, _scene: &mut Scene) {}
}

// ============================================================================
// Example animation: MoveTo
// ============================================================================

/// Translate a drawable to a target position.
pub struct MoveTo {
    pub target_id: TattvaId,
    pub start_time: f32,
    pub duration: f32,
    pub from: Option<Vec3>, // captured at on_start
    pub to: Vec3,
    pub ease: Ease,
}

impl MoveTo {
    pub fn new(
        target_id: TattvaId,
        start_time: f32,
        duration: f32,
        to: Vec3,
        ease: Ease,
    ) -> Self {
        Self {
            target_id,
            start_time,
            duration,
            from: None,
            to,
            ease,
        }
    }

    fn find_drawable_mut<'a>(
        &'a self,
        scene: &'a mut Scene,
    ) -> Option<&'a mut crate::renderer::renderer::DrawableInstance> {
        scene
            .drawables
            .iter_mut()
            .find(|d| d.tattva_id == Some(self.target_id))
    }
}

impl Animation for MoveTo {
    fn on_start(&mut self, scene: &mut Scene) {
        if let Some(drawable) = self.find_drawable_mut(scene) {
            self.from = Some(drawable.props.position);
        } else {
            // Missing drawable → no-op
            self.from = Some(self.to);
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t_abs: f32) {
        let elapsed = (t_abs - self.start_time).clamp(0.0, self.duration);
        let t = if self.duration > 0.0 {
            elapsed / self.duration
        } else {
            1.0
        };

        let k = self.ease.eval(t);
        let from = self.from.unwrap_or(self.to);
        let new_pos = from.lerp(self.to, k);

        if let Some(drawable) = self.find_drawable_mut(scene) {
            drawable.props.position = new_pos;
        }
    }

    fn on_finish(&mut self, _scene: &mut Scene) {
        // nothing extra for now
    }
}
