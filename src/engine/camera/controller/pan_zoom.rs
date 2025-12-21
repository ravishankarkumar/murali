// src/camera/controller/pan_zoom.rs

use glam::{Vec2, Vec3};
use crate::engine::camera::{Camera, Projection};

pub struct PanZoomCameraController {
    pub pan_speed: f32,
    pub zoom_speed: f32,
}

impl PanZoomCameraController {
    pub fn new() -> Self {
        Self {
            pan_speed: 1.0,
            zoom_speed: 0.1,
        }
    }

    /// Mouse drag → pan (screen-space)
    pub fn pan(&self, delta: Vec2, cam: &mut Camera) {
        let right = cam.right();
        let up = cam.up;

        let offset = (-right * delta.x + up * delta.y) * self.pan_speed;

        cam.position += offset;
        cam.target += offset;
    }

    /// Mouse wheel → zoom
    pub fn zoom(&self, amount: f32, cam: &mut Camera) {
        match cam.projection {
            Projection::Orthographic {
                width,
                height,
                near,
                far,
            } => {
                let scale = (1.0 - amount * self.zoom_speed).max(0.01);

                cam.projection = Projection::Orthographic {
                    width: width * scale,
                    height: height * scale,
                    near,
                    far,
                };
            }

            Projection::Perspective { .. } => {
                // optional: dolly for perspective
                let dir = cam.forward();
                cam.position += dir * amount * self.zoom_speed;
            }
        }
    }
}
