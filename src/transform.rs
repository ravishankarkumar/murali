// src/transform.rs

use glam::{Vec3, Quat};

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub translate: Vec3,
    pub rotate: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            translate: Vec3::ZERO,
            rotate: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translate: Vec3::ZERO,
            rotate: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

