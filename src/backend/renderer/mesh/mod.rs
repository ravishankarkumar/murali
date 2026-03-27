// src/renderer/mesh/mod.rs

pub mod latex_quad;
pub mod typst_quad;

use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MeshPipelineKind {
    Mesh,
    Text,
}

/// GPU-side mesh buffers.
pub struct MeshInstance {
    pub vertex_buffer: Arc<wgpu::Buffer>, // Wrap in Arc
    pub index_buffer: Arc<wgpu::Buffer>,  // Wrap in Arc
    pub index_count: u32,
    pub bind_group: Option<Arc<wgpu::BindGroup>>, // Use Arc for sharing
    pub pipeline_kind: MeshPipelineKind,
}

// Manually implement Clone so Arc::make_mut works
impl Clone for MeshInstance {
    fn clone(&self) -> Self {
        Self {
            vertex_buffer: self.vertex_buffer.clone(),
            index_buffer: self.index_buffer.clone(),
            index_count: self.index_count,
            bind_group: self.bind_group.clone(),
            pipeline_kind: self.pipeline_kind,
        }
    }
}

impl MeshInstance {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[u8],
        indices: &[u8],
        index_count: u32,
        bind_group: Option<Arc<wgpu::BindGroup>>,
        pipeline_kind: MeshPipelineKind,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh-vertex-buffer"),
            contents: vertices,
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh-index-buffer"),
            contents: indices,
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer: Arc::new(vertex_buffer),
            index_buffer: Arc::new(index_buffer),
            index_count,
            bind_group,
            pipeline_kind,
        }
    }
}

pub trait Drawable {
    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>);
}

impl Drawable for MeshInstance {
    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
