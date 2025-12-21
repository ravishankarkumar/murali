// src/camera/controller/mod.rs

pub mod orbit;
pub mod pan_zoom;

use glam::Vec2;
use crate::camera::Camera;

use orbit::OrbitCameraController;
use pan_zoom::PanZoomCameraController;

/// Exactly one camera controller is active at a time.
pub enum ActiveCameraController {
    Orbit(OrbitCameraController),
    PanZoom(PanZoomCameraController),
}

impl ActiveCameraController {
    pub fn handle_mouse_drag(&mut self, delta: Vec2, cam: &mut Camera) {
        match self {
            ActiveCameraController::Orbit(ctrl) => {
                ctrl.handle_mouse_drag(delta, cam);
            }
            ActiveCameraController::PanZoom(ctrl) => {
                ctrl.pan(delta, cam);
            }
        }
    }

    pub fn handle_scroll(&mut self, delta: f32, cam: &mut Camera) {
        match self {
            ActiveCameraController::Orbit(ctrl) => {
                // optional: dolly for orbit (later)
                let dir = cam.forward();
                cam.position += dir * delta * 0.1;
            }
            ActiveCameraController::PanZoom(ctrl) => {
                ctrl.zoom(delta, cam);
            }
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ActiveCameraController::Orbit(_) => "Orbit (3D)",
            ActiveCameraController::PanZoom(_) => "Pan + Zoom (2D)",
        }
    }
}
