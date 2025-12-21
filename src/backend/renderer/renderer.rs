use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroU64;
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

        // ===============================
        // 🔑 MISSING FIELD 1: Camera Buffer
        // ===============================
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: 64, // Mat4 size
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ===============================
        // Shaders
        // ===============================
        let mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mesh-shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/mesh.wgsl"))),
        });

        let text_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text-shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/text.wgsl"))),
        });

        let line_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("line-shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/line.wgsl"))),
        });

        // ===============================
        // Bind group layouts
        // ===============================
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform-bind-group-layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: Some(NonZeroU64::new(uniform_slot_size).unwrap()),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture-bind-group-layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // 🔑 MISSING FIELD 2: Camera Bind Group Layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("camera_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // 🔑 MISSING FIELD 3: Line Bind Group Layout (Storage for lines)
        let line_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("line_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //     label: Some("pipeline-layout"),
        //     bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
        //     push_constant_ranges: &[],
        // });

        let mesh_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh-pipeline-layout"),
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let text_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text-pipeline-layout"),
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let line_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("line-pipeline-layout"),
            bind_group_layouts: &[&line_bind_group_layout, &camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        // ===============================
        // Pipelines
        // ===============================
        let mesh_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mesh-pipeline"),
            layout: Some(&mesh_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &mesh_shader,
                entry_point: "vs_main",
                buffers: &[MeshVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &mesh_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth.format,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                // depth_write_enabled: true,
                // depth_compare: wgpu::CompareFunction::Less,
                // depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
        });

        let text_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text-pipeline"),
            layout: Some(&text_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &text_shader,
                entry_point: "vs_main",
                buffers: &[TextVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &text_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth.format,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
        });

        // 🔑 MISSING FIELD 4: Line Pipeline
        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("line-pipeline"),
            layout: Some(&line_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &line_shader,
                entry_point: "vs_main",
                buffers: &[], // Lines are generated in shader from Storage Buffer
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &line_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth.format,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
        });

        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform-bind-group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    // resource: uniform_buffer.as_entire_binding(),
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: Some(NonZeroU64::new(uniform_slot_size).unwrap()),
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&default_sampler),
                },
            ],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let (_, _, default_texture_bind_group) = Self::create_texture_bind_group_from_rgba(
            device,
            &device_mgr.queue,
            &texture_bind_group_layout,
            &default_sampler,
            &[255, 255, 255, 255],
            1,
            1,
        );

        Self {
            device_mgr: device_mgr.clone(),
            clear_color: wgpu::Color {
                r: 0.05,
                g: 0.1,
                b: 0.15,
                a: 1.0,
            },
            mesh_pipeline,
            text_pipeline,
            depth,
            uniform_buffer,
            uniform_bind_group,
            uniform_slot_size,
            max_drawables,
            mesh_cache: HashMap::new(),
            texture_bind_group_layout,
            default_sampler,
            default_texture_bind_group,
            line_pipeline,
            camera_buffer,
            camera_bind_group,
            line_bind_group_layout
            // start_time: Instant::now(),
        }
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

    //helpers
    pub fn create_texture_bind_group_from_rgba(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::TextureView, wgpu::BindGroup) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("rgba-texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture-bind-group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });

        (texture, view, bind_group)
    }

    pub fn create_text_bind_group_from_raster(
        &self,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) -> wgpu::BindGroup {
        let (_t, _v, bg) = Self::create_texture_bind_group_from_rgba(
            &self.device_mgr.device,
            &self.device_mgr.queue,
            &self.texture_bind_group_layout,
            &self.default_sampler,
            rgba,
            width,
            height,
        );
        bg
    }
}
