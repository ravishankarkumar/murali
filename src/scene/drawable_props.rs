// src/scene/drawable_props.rs

use glam::{Quat, Vec3};
use glam::Mat4;
use std::sync::Arc;
use parking_lot::RwLock;

pub type SharedProps = Arc<RwLock<DrawableProps>>;

/// Runtime visual state of a drawable.
/// This is authoritative for rendering and animation.
#[derive(Debug, Clone)]
pub struct DrawableProps {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub visible: bool,
    pub opacity: f32,
}

impl Default for DrawableProps {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            visible: true,
            opacity: 1.0,
        }
    }
}

/// Builder-style helpers
impl DrawableProps {
    pub fn identity() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            visible: true,
            opacity: 1.0,
        }
    }

    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn at(mut self, pos: Vec3) -> Self {
        self.position = pos;
        self
    }

    pub fn scale_uniform(mut self, s: f32) -> Self {
        self.scale = Vec3::splat(s);
        self
    }

    pub fn scale(mut self, v: Vec3) -> Self {
        self.scale = v;
        self
    }

    pub fn rotate(mut self, q: Quat) -> Self {
        self.rotation = q;
        self
    }

    pub fn hide(mut self) -> Self {
        self.visible = false;
        self
    }

    pub fn show(mut self) -> Self {
        self.visible = true;
        self
    }

    pub fn opacity(mut self, v: f32) -> Self {
        self.opacity = v.clamp(0.0, 1.0);
        self
    }
}
