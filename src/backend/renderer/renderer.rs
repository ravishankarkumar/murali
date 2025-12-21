use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::DeviceExt;

use crate::backend::ecs::components::*;
use crate::backend::renderer::device::{DepthTexture, DeviceManager};
use crate::backend::renderer::mesh::{Drawable, MeshData, MeshInstance}; // Added Drawable and MeshData
use crate::backend::renderer::vertex::mesh::MeshVertex;
use crate::backend::renderer::vertex::text::TextVertex;
use crate::engine::scene::Scene;
use crate::frontend::props::DrawableProps;
use crate::frontend::props::SharedProps;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub mvp: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn from_mat4(mat: Mat4) -> Self {
        Self {
            mvp: mat.to_cols_array_2d(),
        }
    }
}

pub struct Renderer {
    pub device_mgr: Arc<DeviceManager>, // Changed to Arc
    pub clear_color: wgpu::Color,

    mesh_pipeline: wgpu::RenderPipeline,
    text_pipeline: wgpu::RenderPipeline,
    line_pipeline: wgpu::RenderPipeline,

    pub depth: DepthTexture,

    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_slot_size: u64,
    max_drawables: u64,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub line_bind_group_layout: wgpu::BindGroupLayout,

    default_sampler: wgpu::Sampler,
    default_texture_bind_group: wgpu::BindGroup,

    mesh_cache: HashMap<usize, Arc<MeshInstance>>,
}

impl Renderer {
    /// Initializer for the 2.1 Renderer.
    /// Note: You will need to move your pipeline creation logic (Shaders, Layouts) into here.
    pub fn new(device_mgr: Arc<DeviceManager>) -> Self {
        let device = &device_mgr.device;
        // let config = &device_mgr.config;
        let config = device_mgr.config.borrow();

        // 1. Create Depth Texture
        let depth = DepthTexture::create(device, &config);

        // 2. Setup Uniforms (MVP)
        let max_drawables = 1000;
        let min_alignment = device.limits().min_uniform_buffer_offset_alignment as u64;
        let uniform_slot_size =
            (std::mem::size_of::<Uniforms>() as u64 + min_alignment - 1) & !(min_alignment - 1);

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: uniform_slot_size * max_drawables,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ... (Remaining Pipeline/BindGroup setup logic goes here) ...
        // For brevity, I am assuming placeholders for the fields below.
        // You should paste your specific WGPU pipeline descriptors here.

        todo!("Move your pipeline creation logic from 2.0 into this constructor")
    }

    pub fn render_scene(&mut self, scene: &Scene, world: &hecs::World) -> Result<()> {
        let (frame, view) = self.device_mgr.acquire_frame()?;
        let view_proj = scene.camera.view_proj_matrix();

        let mut encoder =
            self.device_mgr
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Murali Render Encoder"),
                });

        // --- 1. PREPARE INSTANCED LINE DATA ---
        let mut line_data = Vec::new();
        let mut line_count = 0;
        for (_, (line, color)) in world.query::<(&LineComponent, &ColorComponent)>().iter() {
            line_data.extend_from_slice(bytemuck::cast_slice(&[
                line.start.x,
                line.start.y,
                line.start.z,
                0.0,
                line.end.x,
                line.end.y,
                line.end.z,
                0.0,
                color.0.x,
                color.0.y,
                color.0.z,
                color.0.w,
                line.thickness,
                0.0,
                0.0,
                0.0,
            ]));
            line_count += 1;
        }

        let line_resources = if line_count > 0 {
            let buffer =
                self.device_mgr
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Line Storage Buffer"),
                        contents: &line_data,
                        usage: wgpu::BufferUsages::STORAGE,
                    });
            let bind_group = self
                .device_mgr
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.line_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }],
                    label: None,
                });
            Some((buffer, bind_group))
        } else {
            None
        };

        let draw_list: Vec<(Arc<MeshInstance>, Mat4, Option<Arc<wgpu::BindGroup>>)> = {
            let mut list = Vec::new();
            let mut query = world.query::<(&MeshComponent, &SharedProps)>();
            for (_, (mesh_comp, props)) in query.iter() {
                let props = DrawableProps::read(props);
                list.push((
                    mesh_comp.0.clone(),
                    props.model_matrix(),
                    mesh_comp.0.bind_group.clone(),
                ));
            }
            list
        };
        // --- 2. MAIN RENDER PASS ---
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Primary Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            if let Some((_, bg)) = &line_resources {
                self.device_mgr.queue.write_buffer(
                    &self.camera_buffer,
                    0,
                    bytemuck::cast_slice(view_proj.as_ref()),
                );
                rpass.set_pipeline(&self.line_pipeline);
                rpass.set_bind_group(0, bg, &[]);
                rpass.set_bind_group(1, &self.camera_bind_group, &[]);
                rpass.draw(0..6, 0..line_count as u32);
            }

            let mut draw_idx = 0;

            // --- RENDER ---
            let mut draw_idx = 0;
            for (mesh, model, bind_group_opt) in &draw_list {
                if draw_idx >= self.max_drawables {
                    break;
                }

                let mvp = view_proj * *model; // Dereference model since we are iterating by ref
                let offset = (draw_idx * self.uniform_slot_size) as u32;

                self.device_mgr.queue.write_buffer(
                    &self.uniform_buffer,
                    offset as u64,
                    bytemuck::cast_slice(&[Uniforms::from_mat4(mvp)]),
                );

                rpass.set_pipeline(&self.mesh_pipeline);
                rpass.set_bind_group(0, &self.uniform_bind_group, &[offset]);

                // bind_group_opt is now &Option<Arc<BindGroup>>
                if let Some(bg) = bind_group_opt.as_ref() {
                    rpass.set_bind_group(1, bg, &[]); // bg is &BindGroup
                } else {
                    rpass.set_bind_group(1, &self.default_texture_bind_group, &[]);
                }

                mesh.draw(&mut rpass); // mesh is &Arc<MeshInstance>
                draw_idx += 1;
            }
        }

        self.device_mgr.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.device_mgr.resize(size);
        //
        let config = self.device_mgr.config.borrow();
        self.depth = DepthTexture::create(&self.device_mgr.device, &config);
    }
}
