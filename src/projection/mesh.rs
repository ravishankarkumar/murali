use crate::backend::renderer::vertex::{mesh::MeshVertex, text::TextVertex};
use glam::{Vec3, Vec4};
use std::sync::Arc;

/// CPU-side vertex storage emitted by projection.
#[derive(Debug, Clone)]
pub enum MeshData {
    Empty,
    Mesh(Vec<MeshVertex>),
    Text(Vec<TextVertex>),
}

/// CPU-side mesh description.
///
/// This lives in the projection layer because it is part of the
/// backend-neutral output produced before GPU upload.
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

    /// Square in XY plane centered at origin.
    pub fn square(size: f32, color: Vec4) -> Arc<Self> {
        Self::rectangle(size, size, color)
    }

    /// Axis-aligned rectangle in XY plane centered at origin.
    pub fn rectangle(width: f32, height: f32, color: Vec4) -> Arc<Self> {
        let hw = width * 0.5;
        let hh = height * 0.5;

        let vertices = vec![
            MeshVertex {
                position: [-hw, -hh, 0.0],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [hw, -hh, 0.0],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [hw, hh, 0.0],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [-hw, hh, 0.0],
                color: [color[0], color[1], color[2]],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Arc::new(Self {
            data: MeshData::Mesh(vertices),
            indices,
        })
    }

    /// Axis-aligned cube centered at origin.
    pub fn cube(size: f32, color: Vec4) -> Arc<Self> {
        let h = size * 0.5;

        let vertices = vec![
            MeshVertex {
                position: [-h, -h, -h],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [h, -h, -h],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [h, h, -h],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [-h, h, -h],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [-h, -h, h],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [h, -h, h],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [h, h, h],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [-h, h, h],
                color: [color[0], color[1], color[2]],
            },
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 0, 4, 7, 7, 3, 0, 1, 5, 6, 6, 2, 1, 0, 1, 5,
            5, 4, 0, 3, 2, 6, 6, 7, 3,
        ];

        Arc::new(Self {
            data: MeshData::Mesh(vertices),
            indices,
        })
    }

    /// Triangle fan circle in XY plane.
    pub fn circle(radius: f32, segments: u32, color: Vec4) -> Arc<Self> {
        let seg = segments.max(3);
        let mut vertices = Vec::with_capacity((seg + 1) as usize);

        vertices.push(MeshVertex {
            position: [0.0, 0.0, 0.0],
            color: [color[0], color[1], color[2]],
        });

        for i in 0..seg {
            let t = (i as f32 / seg as f32) * std::f32::consts::TAU;
            vertices.push(MeshVertex {
                position: [radius * t.cos(), radius * t.sin(), 0.0],
                color: [color[0], color[1], color[2]],
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

    /// Line rendered as a thin rectangle using triangles.
    pub fn line(start: [f32; 3], end: [f32; 3], thickness: f32, color: Vec4) -> Arc<Self> {
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
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [sx - ox, sy - oy, sz],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [ex - ox, ey - oy, ez],
                color: [color[0], color[1], color[2]],
            },
            MeshVertex {
                position: [ex + ox, ey + oy, ez],
                color: [color[0], color[1], color[2]],
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Arc::new(Self {
            data: MeshData::Mesh(vertices),
            indices,
        })
    }

    pub fn vertex_kind(&self) -> &'static str {
        match self.data {
            MeshData::Empty => "Empty",
            MeshData::Mesh(_) => "Mesh",
            MeshData::Text(_) => "Text",
        }
    }

    pub fn translated(&self, offset: Vec3) -> Arc<Self> {
        match &self.data {
            MeshData::Empty => Arc::new(self.clone()),
            MeshData::Mesh(vertices) => Arc::new(Self {
                data: MeshData::Mesh(
                    vertices
                        .iter()
                        .map(|v| {
                            let mut v = *v;
                            v.position[0] += offset.x;
                            v.position[1] += offset.y;
                            v.position[2] += offset.z;
                            v
                        })
                        .collect(),
                ),
                indices: self.indices.clone(),
            }),
            MeshData::Text(vertices) => Arc::new(Self {
                data: MeshData::Text(
                    vertices
                        .iter()
                        .map(|v| {
                            let mut v = *v;
                            v.position[0] += offset.x;
                            v.position[1] += offset.y;
                            v.position[2] += offset.z;
                            v
                        })
                        .collect(),
                ),
                indices: self.indices.clone(),
            }),
        }
    }
}
