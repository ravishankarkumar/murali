use glam::Vec4;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ColorComponent(pub Vec4);

impl ColorComponent {
    pub fn white() -> Self {
        Self(Vec4::ONE)
    }
    
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(Vec4::new(r, g, b, a))
    }
}