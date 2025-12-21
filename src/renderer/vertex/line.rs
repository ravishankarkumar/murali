// src/renderer/vertex/line.rs
use glam::Vec3;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LineComponent {
    pub start: Vec3,
    pub _pad1: f32,
    pub end: Vec3,
    pub thickness: f32,
}

impl LineComponent {
    pub fn new(start: Vec3, end: Vec3, thickness: f32) -> Self {
        Self {
            start,
            _pad1: 0.0,
            end,
            thickness,
        }
    }
}
