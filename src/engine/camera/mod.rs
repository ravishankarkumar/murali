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

/// Canonical world-space constants.
/// These define Murali's default coordinate canvas.
pub const DEFAULT_VIEW_WIDTH: f32 = 16.0;
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const DEFAULT_VIEW_HEIGHT: f32 = DEFAULT_VIEW_WIDTH / ASPECT_RATIO;

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
            } => Mat4::perspective_rh(fov_y_rad, aspect, near, far),
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

    /// Returns the visible world width (orthographic only).
    /// For perspective cameras, returns 0.0.
    pub fn view_width(&self) -> f32 {
        match self.projection {
            Projection::Orthographic { width, .. } => width,
            Projection::Perspective { .. } => 0.0,
        }
    }

    /// Sets the visible world width, maintaining 16:9 aspect ratio.
    /// Smaller values zoom in (objects appear larger).
    /// Larger values zoom out (more world is visible).
    /// No-op for perspective cameras.
    pub fn set_view_width(&mut self, width: f32) {
        if let Projection::Orthographic { width: w, height: h, .. } = &mut self.projection {
            *w = width.max(0.01);
            *h = width.max(0.01) / ASPECT_RATIO;
        }
    }

    /// Zoom in by a factor — objects appear `factor` times larger.
    /// Equivalent to `set_view_width(view_width() / factor)`.
    pub fn zoom_in(&mut self, factor: f32) {
        let w = self.view_width();
        self.set_view_width(w / factor.max(0.001));
    }

    /// Zoom out by a factor — objects appear `factor` times smaller.
    /// Equivalent to `set_view_width(view_width() * factor)`.
    pub fn zoom_out(&mut self, factor: f32) {
        let w = self.view_width();
        self.set_view_width(w * factor.max(0.001));
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
                width: DEFAULT_VIEW_WIDTH,
                height: DEFAULT_VIEW_HEIGHT,
                near: -100.0,
                far: 100.0,
            },
        }
    }
}
