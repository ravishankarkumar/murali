use glam::{Vec2, Vec3, Vec4, vec2};
use std::sync::Arc;

use crate::backend::renderer::vertex::mesh::MeshVertex;
use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Mesh, Project, ProjectionCtx, RenderPrimitive};

/// How to render the parametric surface
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurfaceRenderMode {
    /// Solid filled mesh
    Solid,
    /// Wireframe grid lines
    Wireframe,
    /// Both solid and wireframe
    SolidWithWireframe,
}

/// A parametric 3D surface defined by f(u, v) -> Vec3
/// 
/// Example: Sphere
/// ```ignore
/// let sphere = ParametricSurface::new(
///     (0.0, std::f32::consts::PI),      // u_range (theta)
///     (0.0, 2.0 * std::f32::consts::PI), // v_range (phi)
///     |u, v| {
///         let sin_u = u.sin();
///         Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
///     }
/// );
/// ```
#[derive(Clone)]
pub struct ParametricSurface {
    pub u_range: (f32, f32),
    pub v_range: (f32, f32),
    pub u_samples: usize,
    pub v_samples: usize,
    pub color: Vec4,
    pub f: Arc<dyn Fn(f32, f32) -> Vec3 + Send + Sync>,
    /// Animation progress: 0.0 = nothing drawn, 1.0 = fully drawn
    pub write_progress: f32,
    /// Render mode: Solid mesh or wireframe grid
    pub render_mode: SurfaceRenderMode,
    /// Optional color function based on height/parameter
    pub color_fn: Option<Arc<dyn Fn(f32) -> Vec4 + Send + Sync>>,
}

impl ParametricSurface {
    pub fn new(
        u_range: (f32, f32),
        v_range: (f32, f32),
        f: impl Fn(f32, f32) -> Vec3 + Send + Sync + 'static,
    ) -> Self {
        Self {
            u_range,
            v_range,
            u_samples: 32,
            v_samples: 32,
            color: Vec4::new(0.44, 0.84, 0.71, 1.0),
            f: Arc::new(f),
            write_progress: 1.0,
            render_mode: SurfaceRenderMode::Solid,
            color_fn: None,
        }
    }

    pub fn with_samples(mut self, u_samples: usize, v_samples: usize) -> Self {
        self.u_samples = u_samples.max(2);
        self.v_samples = v_samples.max(2);
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_write_progress(mut self, progress: f32) -> Self {
        self.write_progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn with_render_mode(mut self, mode: SurfaceRenderMode) -> Self {
        self.render_mode = mode;
        self
    }

    pub fn with_color_fn(mut self, color_fn: impl Fn(f32) -> Vec4 + Send + Sync + 'static) -> Self {
        self.color_fn = Some(Arc::new(color_fn));
        self
    }

    /// Generate mesh vertices and indices for the surface
    fn generate_mesh(&self) -> (Vec<MeshVertex>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        if self.write_progress <= 0.0 {
            return (vertices, indices); // Nothing to draw
        }

        let u_step = (self.u_range.1 - self.u_range.0) / (self.u_samples - 1) as f32;
        let v_step = (self.v_range.1 - self.v_range.0) / (self.v_samples - 1) as f32;

        // Calculate how many rows to draw based on write_progress
        let total_rows = self.u_samples;
        let rows_to_draw = ((self.write_progress * total_rows as f32).ceil() as usize).min(total_rows);

        // Generate vertices for rows we're drawing
        for i in 0..rows_to_draw {
            for j in 0..self.v_samples {
                let u = self.u_range.0 + i as f32 * u_step;
                let v = self.v_range.0 + j as f32 * v_step;
                let pos = (self.f)(u, v);

                vertices.push(MeshVertex {
                    position: [pos.x, pos.y, pos.z],
                    color: [self.color.x, self.color.y, self.color.z, self.color.w],
                });
            }
        }

        // Generate indices (triangles) only for complete rows
        for i in 0..(rows_to_draw.saturating_sub(1)) {
            for j in 0..(self.v_samples - 1) {
                let a = (i * self.v_samples + j) as u16;
                let b = (i * self.v_samples + j + 1) as u16;
                let c = ((i + 1) * self.v_samples + j) as u16;
                let d = ((i + 1) * self.v_samples + j + 1) as u16;

                // First triangle
                indices.push(a);
                indices.push(b);
                indices.push(c);

                // Second triangle
                indices.push(b);
                indices.push(d);
                indices.push(c);
            }
        }

        (vertices, indices)
    }

    /// Sample points on the surface for bounds calculation
    fn sample_points(&self) -> Vec<Vec3> {
        let mut pts = Vec::new();
        let u_step = (self.u_range.1 - self.u_range.0) / (self.u_samples - 1) as f32;
        let v_step = (self.v_range.1 - self.v_range.0) / (self.v_samples - 1) as f32;

        for i in 0..self.u_samples {
            for j in 0..self.v_samples {
                let u = self.u_range.0 + i as f32 * u_step;
                let v = self.v_range.0 + j as f32 * v_step;
                pts.push((self.f)(u, v));
            }
        }

        pts
    }

    /// Emit wireframe grid lines with optional color mapping
    fn emit_wireframe(&self, ctx: &mut ProjectionCtx) {
        if self.write_progress <= 0.0 {
            return;
        }

        let u_step = (self.u_range.1 - self.u_range.0) / (self.u_samples - 1) as f32;
        let v_step = (self.v_range.1 - self.v_range.0) / (self.v_samples - 1) as f32;

        let total_rows = self.u_samples;
        let rows_to_draw = ((self.write_progress * total_rows as f32).ceil() as usize).min(total_rows);

        // Generate grid points
        let mut grid_points = Vec::new();
        for i in 0..rows_to_draw {
            let mut row = Vec::new();
            for j in 0..self.v_samples {
                let u = self.u_range.0 + i as f32 * u_step;
                let v = self.v_range.0 + j as f32 * v_step;
                let pos = (self.f)(u, v);
                row.push(pos);
            }
            grid_points.push(row);
        }

        // Calculate animation phase for line drawing
        // Phase 1 (0.0-0.5): Draw horizontal lines
        // Phase 2 (0.5-1.0): Draw vertical lines
        let line_progress = (self.write_progress * 2.0).min(1.0);
        let h_lines_progress = line_progress;
        let v_lines_progress = ((self.write_progress - 0.5) * 2.0).max(0.0);

        // Draw horizontal lines (u-direction) - drawn first
        let total_h_lines = grid_points.len();
        let h_lines_to_draw = ((h_lines_progress * total_h_lines as f32).ceil() as usize).min(total_h_lines);
        
        for row_idx in 0..h_lines_to_draw {
            let row = &grid_points[row_idx];
            for j in 0..row.len() - 1 {
                let start = row[j];
                let end = row[j + 1];
                let color = self.get_color_for_point(start);
                ctx.emit(RenderPrimitive::Line {
                    start,
                    end,
                    thickness: 0.02,
                    color,
                    dash_length: 0.0,
                    gap_length: 0.0,
                    dash_offset: 0.0,
                });
            }
        }

        // Draw vertical lines (v-direction) - drawn after horizontal
        if v_lines_progress > 0.0 {
            let total_v_lines = self.v_samples;
            let v_lines_to_draw = ((v_lines_progress * total_v_lines as f32).ceil() as usize).min(total_v_lines);
            
            for j in 0..v_lines_to_draw {
                for i in 0..grid_points.len() - 1 {
                    let start = grid_points[i][j];
                    let end = grid_points[i + 1][j];
                    let color = self.get_color_for_point(start);
                    ctx.emit(RenderPrimitive::Line {
                        start,
                        end,
                        thickness: 0.02,
                        color,
                        dash_length: 0.0,
                        gap_length: 0.0,
                        dash_offset: 0.0,
                    });
                }
            }
        }
    }

    /// Get color for a point based on color function or default color
    fn get_color_for_point(&self, point: Vec3) -> Vec4 {
        if let Some(color_fn) = &self.color_fn {
            // Use height (z-coordinate) as the parameter for color mapping
            color_fn(point.z)
        } else {
            self.color
        }
    }
}

impl Project for ParametricSurface {
    fn project(&self, ctx: &mut ProjectionCtx) {
        match self.render_mode {
            SurfaceRenderMode::Solid => {
                let (vertices, indices) = self.generate_mesh();
                let mesh = Mesh::from_tessellation(vertices, indices);
                ctx.emit(RenderPrimitive::Mesh(mesh));
            }
            SurfaceRenderMode::Wireframe => {
                self.emit_wireframe(ctx);
            }
            SurfaceRenderMode::SolidWithWireframe => {
                let (vertices, indices) = self.generate_mesh();
                let mesh = Mesh::from_tessellation(vertices, indices);
                ctx.emit(RenderPrimitive::Mesh(mesh));
                self.emit_wireframe(ctx);
            }
        }
    }
}

impl Bounded for ParametricSurface {
    fn local_bounds(&self) -> Bounds {
        let pts = self.sample_points();
        let mut min_xy = Vec2::splat(f32::INFINITY);
        let mut max_xy = Vec2::splat(f32::NEG_INFINITY);
        let mut min_z = f32::INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for p in pts {
            min_xy.x = min_xy.x.min(p.x);
            min_xy.y = min_xy.y.min(p.y);
            max_xy.x = max_xy.x.max(p.x);
            max_xy.y = max_xy.y.max(p.y);
            min_z = min_z.min(p.z);
            max_z = max_z.max(p.z);
        }

        let z_pad = max_z.abs().max(min_z.abs()) * 0.15;
        Bounds::new(min_xy - vec2(z_pad, z_pad), max_xy + vec2(z_pad, z_pad))
    }
}
