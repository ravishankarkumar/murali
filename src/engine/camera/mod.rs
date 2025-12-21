// src/camera/mod.rs
//! Camera definition for Murali.
//!
//! Design (LOCKED):
//! ----------------
//! - Single 3D camera model (always Vec3, Mat4).
//! - Orthographic projection by default (math-first).
//! - Perspective projection is opt-in.
//! - Camera is PURE state: no input, no movement logic.
//! - 2D scenes are constrained 3D scenes (Z used for layering).

pub mod controller;
use glam::{Mat4, Vec3};

/// Projection mode for the camera.
#[derive(Debug, Copy, Clone)]
pub enum Projection {
    /// Orthographic projection (default for math scenes)
    Orthographic {
        /// Visible width in world units
        width: f32,
        /// Visible height in world units
        height: f32,
        /// Near clipping plane
        near: f32,
        /// Far clipping plane
        far: f32,
    },

    /// Perspective projection (for true 3D scenes)
    Perspective {
        /// Vertical field of view (radians)
        fov_y_rad: f32,
        /// Aspect ratio (width / height)
        aspect: f32,
        /// Near clipping plane
        near: f32,
        /// Far clipping plane
        far: f32,
    },
}

impl Projection {
    /// Compute the projection matrix.
    pub fn matrix(&self) -> Mat4 {
        match *self {
            Projection::Orthographic {
                width,
                height,
                near,
                far,
            } => Mat4::orthographic_rh(
                -width / 2.0,
                 width / 2.0,
                -height / 2.0,
                 height / 2.0,
                 near,
                 far,
            ),

            Projection::Perspective {
                fov_y_rad,
                aspect,
                near,
                far,
            } => Mat4::perspective_rh(
                fov_y_rad,
                aspect,
                near,
                far,
            ),
        }
    }
}

/// Camera describing view + projection.
/// Owned by `Scene`.
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    /// Camera position in world space
    pub position: Vec3,

    /// Point the camera is looking at
    pub target: Vec3,

    /// Up direction (usually +Y)
    pub up: Vec3,

    /// Projection mode
    pub projection: Projection,
}

impl Camera {
    /// View matrix (world → view)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Projection matrix (view → clip)
    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.matrix()
    }

    /// Combined view-projection matrix (world → clip)
    pub fn view_proj_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Convenience: forward direction (normalized)
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    /// Convenience: right direction (normalized)
    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }
}

/// Canonical default camera:
/// - Orthographic
/// - 16:9 world
/// - Looking down -Z
/// - Origin centered
impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 10.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            projection: Projection::Orthographic {
                width: 16.0,
                height: 9.0,
                near: -100.0,
                far: 100.0,
            },
        }
    }
}
