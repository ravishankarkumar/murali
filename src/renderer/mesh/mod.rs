// src/renderer/mesh/mod.rs

pub mod latex_quad;
pub mod typst_quad;

use crate::renderer::vertex::{mesh::MeshVertex, text::TextVertex};
use crate::scene::Drawable;

use bytemuck::Pod;
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// CPU-side vertex storage.
/// The renderer decides *which pipeline* to use.
#[derive(Debug, Clone)]
pub enum MeshData {
    Empty,
    Mesh(Vec<MeshVertex>),
    Text(Vec<TextVertex>),
}

/// CPU-side mesh: vertices + indices.
/// This is the only thing a Tattva produces.
#[derive(Debug, Clone)]
pub struct Mesh {
    pub data: MeshData,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn empty() -> Self {
        Self {
            data: MeshData::Empty,
            indices: Vec::new(),
        }
    }

    // ------------------------------------------------------------
    // Geometry builders (MeshVertex)
    // ------------------------------------------------------------

    /// Square in XY plane centered at origin.
    pub fn square(size: f32, color: [f32; 3]) -> Arc<Self> {
        let h = size * 0.5;

        let vertices = vec![
            MeshVertex {
                position: [-h, -h, 0.0],
                color,
            },
            MeshVertex {
                position: [h, -h, 0.0],
                color,
            },
            MeshVertex {
                position: [h, h, 0.0],
                color,
            },
            MeshVertex {
                position: [-h, h, 0.0],
                color,
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Arc::new(Self {
            data: MeshData::Mesh(vertices),
            indices,
        })
    }

    /// Axis-aligned cube centered at origin.
    pub fn cube(size: f32, color: [f32; 3]) -> Arc<Self> {
        let h = size * 0.5;

        let vertices = vec![
            MeshVertex {
                position: [-h, -h, -h],
                color,
            }, // 0
            MeshVertex {
                position: [h, -h, -h],
                color,
            }, // 1
            MeshVertex {
                position: [h, h, -h],
                color,
            }, // 2
            MeshVertex {
                position: [-h, h, -h],
                color,
            }, // 3
            MeshVertex {
                position: [-h, -h, h],
                color,
            }, // 4
            MeshVertex {
                position: [h, -h, h],
                color,
            }, // 5
            MeshVertex {
                position: [h, h, h],
                color,
            }, // 6
            MeshVertex {
                position: [-h, h, h],
                color,
            }, // 7
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, // back
            4, 5, 6, 6, 7, 4, // front
            0, 4, 7, 7, 3, 0, // left
            1, 5, 6, 6, 2, 1, // right
            0, 1, 5, 5, 4, 0, // bottom
            3, 2, 6, 6, 7, 3, // top
        ];

        Arc::new(Self {
            data: MeshData::Mesh(vertices),
            indices,
        })
    }

    /// Triangle fan circle in XY plane.
    pub fn circle(radius: f32, segments: u32, color: [f32; 3]) -> Arc<Self> {
        let seg = segments.max(3);
        let mut vertices = Vec::with_capacity((seg + 1) as usize);

        vertices.push(MeshVertex {
            position: [0.0, 0.0, 0.0],
            color,
        });

        for i in 0..seg {
            let t = (i as f32 / seg as f32) * std::f32::consts::TAU;
            vertices.push(MeshVertex {
                position: [radius * t.cos(), radius * t.sin(), 0.0],
                color,
            });
        }

        let mut indices = Vec::with_capacity((seg * 3) as usize);
        for i in 0..seg {
            indices.push(0);
            indices.push((i + 1) as u16);
            indices.push(if i + 2 <= seg { (i + 2) as u16 } else { 1 });
        }

        Arc::new(Self {
            data: MeshData::Mesh(vertices),
            indices,
        })
    }

    /// Line rendered as a thin rectangle (two triangles).
    ///
    /// This avoids GPU line primitives and works with the triangle pipeline.
    pub fn line(start: [f32; 3], end: [f32; 3], thickness: f32, color: [f32; 3]) -> Arc<Self> {
        let sx = start[0];
        let sy = start[1];
        let sz = start[2];

        let ex = end[0];
        let ey = end[1];
        let ez = end[2];

        let dx = ex - sx;
        let dy = ey - sy;
        let len = (dx * dx + dy * dy).sqrt();

        let (nx, ny) = if len > 1e-6 {
            (-dy / len, dx / len)
        } else {
            (0.0, 1.0)
        };

        let ox = nx * thickness * 0.5;
        let oy = ny * thickness * 0.5;

        let vertices = vec![
            MeshVertex {
                position: [sx + ox, sy + oy, sz],
                color,
            },
            MeshVertex {
                position: [sx - ox, sy - oy, sz],
                color,
            },
            MeshVertex {
                position: [ex - ox, ey - oy, ez],
                color,
            },
            MeshVertex {
                position: [ex + ox, ey + oy, ez],
                color,
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Arc::new(Self {
            data: MeshData::Mesh(vertices),
            indices,
        })
    }

    // ------------------------------------------------------------
    // GPU upload
    // ------------------------------------------------------------

    pub fn into_gpu_instance(
        &self,
        device: &wgpu::Device,
        bind_group: Option<wgpu::BindGroup>,
    ) -> Option<MeshInstance> {
        match &self.data {
            MeshData::Empty => None,

            MeshData::Mesh(v) => {
                let vertex_bytes = bytemuck::cast_slice(v);
                let index_bytes = bytemuck::cast_slice(&self.indices);

                Some(MeshInstance::new(
                    device,
                    vertex_bytes,
                    index_bytes,
                    self.indices.len() as u32,
                    bind_group,
                ))
            }

            MeshData::Text(v) => {
                let vertex_bytes = bytemuck::cast_slice(v);
                let index_bytes = bytemuck::cast_slice(&self.indices);

                Some(MeshInstance::new(
                    device,
                    vertex_bytes,
                    index_bytes,
                    self.indices.len() as u32,
                    bind_group,
                ))
            }
        }
    }

    /// Debug helper
    pub fn vertex_kind(&self) -> &'static str {
        match self.data {
            MeshData::Empty => "Empty",
            MeshData::Mesh(_) => "Mesh",
            MeshData::Text(_) => "Text",
        }
    }
}

/// GPU-side mesh buffers.
pub struct MeshInstance {
    pub vertex_buffer: Arc<wgpu::Buffer>, // Wrap in Arc
    pub index_buffer: Arc<wgpu::Buffer>,  // Wrap in Arc
    pub index_count: u32,
    pub bind_group: Option<Arc<wgpu::BindGroup>>, // Use Arc for sharing
}

// Manually implement Clone so Arc::make_mut works
impl Clone for MeshInstance {
    fn clone(&self) -> Self {
        Self {
            vertex_buffer: self.vertex_buffer.clone(),
            index_buffer: self.index_buffer.clone(),
            index_count: self.index_count,
            bind_group: self.bind_group.clone(),
        }
    }
}

impl MeshInstance {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[u8],
        indices: &[u8],
        index_count: u32,
        bind_group: Option<wgpu::BindGroup>,
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
            bind_group: bind_group.map(Arc::new),
        }
    }
}

impl Drawable for MeshInstance {
    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        if let Some(bg) = &self.bind_group {
            rpass.set_bind_group(0, bg, &[]);
        }

        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
