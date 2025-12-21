// src/renderer/renderer.rs

use crate::renderer::device::DeviceManager;
use crate::renderer::mesh::{Mesh, MeshInstance};
use crate::renderer::vertex::color::ColorComponent;
use crate::renderer::vertex::line::LineComponent;
use crate::renderer::vertex::mesh::MeshVertex;
use crate::renderer::vertex::text::TextVertex;
use crate::scene::{Drawable, DrawableProps, Scene};
use crate::tattva::Tattva;

use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// ===============================
/// Uniforms (MVP)
/// ===============================
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

pub struct DepthTexture {
    pub view: wgpu::TextureView,
    pub format: wgpu::TextureFormat,
}

impl DepthTexture {
    pub fn create(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let format = wgpu::TextureFormat::Depth24Plus;
        let depth = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth-texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = depth.create_view(&wgpu::TextureViewDescriptor::default());
        Self { view, format }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawableKind {
    Mesh,
    Text,
}

#[derive(Clone)]
pub struct DrawableInstance {
    pub mesh: Option<Arc<MeshInstance>>,
    pub props: DrawableProps,
    pub tattva_id: Option<usize>,
    pub kind: DrawableKind,
}

pub struct Renderer {
    pub device_mgr: DeviceManager,
    pub clear_color: wgpu::Color,

    // Pipelines
    mesh_pipeline: wgpu::RenderPipeline,
    text_pipeline: wgpu::RenderPipeline,
    line_pipeline: wgpu::RenderPipeline,

    depth: DepthTexture,

    // Uniform handling
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_slot_size: u64,
    max_drawables: u64,

    // Line handling (ECS)
    line_bind_group_layout: wgpu::BindGroupLayout,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    mesh_cache: HashMap<usize, Arc<MeshInstance>>,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    default_sampler: wgpu::Sampler,
    default_texture_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(device_mgr: DeviceManager) -> Self {
        let device = &device_mgr.device;
        let config = &device_mgr.config;

        // --- Shader Loading ---
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

        let depth = DepthTexture::create(device, config);

        // --- Uniform & Alignment Calculation ---
        let limits = device.limits();
        let uniform_slot_size = wgpu::util::align_to(
            std::mem::size_of::<Uniforms>() as u64,
            limits.min_uniform_buffer_offset_alignment as u64,
        );
        let max_drawables =
            (limits.max_uniform_buffer_binding_size as u64 / uniform_slot_size).max(1);

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform-buffer"),
            size: uniform_slot_size * max_drawables,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // --- Bind Group Layouts ---
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform-bgl"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(NonZeroU64::new(uniform_slot_size).unwrap()),
                    },
                    count: None,
                }],
            });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture-bgl"),
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

        // Line Storage BGL (Group 0 for Line Pipeline)
        let line_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("line-storage-bgl"),
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

        // Camera BGL (Group 1 for Line Pipeline)
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("camera-bgl"),
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

        // --- Pipelines ---
        let mesh_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh-pl"),
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let line_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("line-pl"),
            bind_group_layouts: &[&line_bind_group_layout, &camera_bind_group_layout],
            push_constant_ranges: &[],
        });

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
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
        });

        let text_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text-pipeline"),
            layout: Some(&mesh_pipeline_layout),
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

        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("line-pipeline"),
            layout: Some(&line_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &line_shader,
                entry_point: "vs_main",
                buffers: &[],
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
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
        });

        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform-bg"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: Some(NonZeroU64::new(uniform_slot_size).unwrap()),
                }),
            }],
        });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera-buffer"),
            size: 64, // 4x4 matrix
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera-bg"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
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
            device_mgr,
            clear_color: wgpu::Color {
                r: 0.05,
                g: 0.1,
                b: 0.15,
                a: 1.0,
            },
            mesh_pipeline,
            text_pipeline,
            line_pipeline,
            depth,
            uniform_buffer,
            uniform_bind_group,
            uniform_slot_size,
            max_drawables,
            line_bind_group_layout,
            camera_buffer,
            camera_bind_group,
            mesh_cache: HashMap::new(),
            texture_bind_group_layout,
            default_sampler,
            default_texture_bind_group,
        }
    }

    pub fn render_scene(&mut self, scene: &Scene) -> Result<()> {
        let (frame, view) = self.device_mgr.acquire_frame()?;
        let view_proj = scene.camera.view_proj_matrix();

        let mut encoder =
            self.device_mgr
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // --- 1. COLLECT ECS LINE DATA ---
        let mut line_data = Vec::new();
        let mut line_count = 0;
        for (_, (line, color)) in scene
            .world
            .query::<(&LineComponent, &ColorComponent)>()
            .iter()
        {
            line_data.extend_from_slice(&[line.start.x, line.start.y, line.start.z, 0.0]);
            line_data.extend_from_slice(&[line.end.x, line.end.y, line.end.z, 0.0]);
            line_data.extend_from_slice(&[color.0.x, color.0.y, color.0.z, color.0.w]);
            line_data.extend_from_slice(&[line.thickness, 0.0, 0.0, 0.0]);
            line_count += 1;
        }

        let mut line_bg_holder = None;
        let mut line_buffer_holder = None;
        if line_count > 0 {
            let lb = self
                .device_mgr
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Line Storage"),
                    contents: bytemuck::cast_slice(&line_data),
                    usage: wgpu::BufferUsages::STORAGE,
                });

            let lbg = self
                .device_mgr
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.line_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: lb.as_entire_binding(),
                    }],
                    label: None,
                });

            // Store them in the "holders" so they don't get dropped early
            line_buffer_holder = Some(lb);
            line_bg_holder = Some(lbg);
        }

        {
            let color_attachment = wgpu::ColorTargetState {
                format: self.device_mgr.config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            }
            .into_attachment(&view, self.clear_color);

            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(color_attachment)],
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

            // --- PATH A: ECS INSTANCED LINES ---

            if let Some(bg) = &line_bg_holder {
                self.device_mgr.queue.write_buffer(
                    &self.camera_buffer,
                    0,
                    bytemuck::cast_slice(view_proj.as_ref()),
                );

                rpass.set_pipeline(&self.line_pipeline);
                rpass.set_bind_group(0, &bg, &[]);
                rpass.set_bind_group(1, &self.camera_bind_group, &[]);
                rpass.draw(0..6, 0..line_count as u32);
            }

            // --- PATH B: TRADITIONAL DRAWABLES (Labels / Meshes) ---
            for (i, drawable) in scene.drawables.iter().enumerate() {
                if i as u64 >= self.max_drawables {
                    break;
                }
                let mesh_inst = match &drawable.mesh {
                    Some(m) => m,
                    None => continue,
                };

                let model = drawable.props.model_matrix();
                let mvp = view_proj * model;
                let offset = i as u64 * self.uniform_slot_size;

                self.device_mgr.queue.write_buffer(
                    &self.uniform_buffer,
                    offset,
                    bytemuck::cast_slice(&[Uniforms::from_mat4(mvp)]),
                );

                match drawable.kind {
                    DrawableKind::Mesh => rpass.set_pipeline(&self.mesh_pipeline),
                    DrawableKind::Text => rpass.set_pipeline(&self.text_pipeline),
                }

                rpass.set_bind_group(0, &self.uniform_bind_group, &[offset as u32]);

                // Bind specific texture for text, or default for mesh
                if let Some(bg) = &mesh_inst.bind_group {
                    rpass.set_bind_group(1, bg, &[]);
                } else {
                    rpass.set_bind_group(1, &self.default_texture_bind_group, &[]);
                }

                mesh_inst.draw(&mut rpass);
            }
        }

        self.device_mgr.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn create_drawable_for_tattva(&mut self, t: &dyn Tattva) -> Option<Arc<MeshInstance>> {
        let mesh_arc = t.mesh();
        if matches!(mesh_arc.data, crate::renderer::mesh::MeshData::Empty) {
            return None;
        }

        let key = Arc::as_ptr(&mesh_arc) as usize;
        if let Some(existing) = self.mesh_cache.get(&key) {
            return Some(Arc::clone(existing));
        }

        let inst = mesh_arc
            .into_gpu_instance(&self.device_mgr.device, None)
            .expect("Mesh upload failed");
        let inst_arc = Arc::new(inst);
        self.mesh_cache.insert(key, Arc::clone(&inst_arc));
        Some(inst_arc)
    }

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
        let (_, _, bg) = Self::create_texture_bind_group_from_rgba(
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

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.device_mgr.resize(size.width, size.height);
        self.depth = DepthTexture::create(&self.device_mgr.device, &self.device_mgr.config);
    }
}

// Extension trait helper for cleaner render pass creation

trait ColorTargetExt {
    fn into_attachment<'a>(
        self,
        view: &'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -> wgpu::RenderPassColorAttachment<'a>;
}

impl ColorTargetExt for wgpu::ColorTargetState {
    fn into_attachment<'a>(
        self,
        view: &'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -> wgpu::RenderPassColorAttachment<'a> {
        wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear),
                store: wgpu::StoreOp::Store,
            },
        }
    }
}
