// src/camera/controller/orbit.rs

use crate::engine::camera::Camera;
use glam::{Vec2, Vec3};

pub struct OrbitCameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub sensitivity: f32,
}

impl OrbitCameraController {
    pub fn new(radius: f32) -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            radius,
            sensitivity: 0.005,
        }
    }

    pub fn handle_mouse_drag(&mut self, delta: Vec2, cam: &mut Camera) {
        self.yaw += delta.x * self.sensitivity;
        self.pitch = (self.pitch + delta.y * self.sensitivity).clamp(-1.5, 1.5);

        let dir = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );

        cam.position = cam.target - dir * self.radius;
    }

    pub fn sync_from_camera(&mut self, cam: &Camera) {
        let v = (cam.position - cam.target).normalize();
        self.pitch = v.y.asin();
        self.yaw = v.z.atan2(v.x);
        self.radius = (cam.position - cam.target).length();
    }
}
