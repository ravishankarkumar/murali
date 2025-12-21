use glam::{Vec3, Quat};
use std::sync::Arc;
use parking_lot::RwLock;

/// Shared transform handle. 
/// Moving this does NOT trigger re-projection (expensive), 
/// it only triggers a matrix update in the ECS (cheap).
pub struct DrawableProps {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub opacity: f32,
}

impl Default for DrawableProps {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            opacity: 1.0,
        }
    }
}

pub type SharedProps = Arc<RwLock<DrawableProps>>;