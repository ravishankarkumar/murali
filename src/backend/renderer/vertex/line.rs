// src/renderer/vertex/line.rs
use bytemuck::{Pod, Zeroable};
use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct LineComponent {
    pub start: Vec3,
    pub end: Vec3,
    pub thickness: f32,
    pub dash_length: f32,
    pub gap_length: f32,
    pub dash_offset: f32,
}

impl LineComponent {
    pub fn new(start: Vec3, end: Vec3, thickness: f32) -> Self {
        Self {
            start,
            end,
            thickness,
            dash_length: 0.0,
            gap_length: 0.0,
            dash_offset: 0.0,
        }
    }
}
