use crate::backend::ecs::components::*;
use crate::backend::renderer::Renderer;
use crate::backend::renderer::mesh::latex_quad::build_textured_quad;
use crate::backend::renderer::mesh::{MeshInstance, MeshPipelineKind};
use crate::frontend::DirtyFlags;
use crate::frontend::TattvaId;
use crate::frontend::tattva_trait::TattvaTrait;
use crate::projection::MeshData;
use crate::projection::{ProjectionCtx, RenderPrimitive};
use crate::resource::latex_resource::backend::compile_latex;
use crate::resource::latex_resource::raster::{normalized_world_height, rasterize_svg};
use crate::resource::text::layout::layout_label;
use crate::resource::text::manager::LabelResources;
use crate::resource::text::mesh::build_label_mesh;
use crate::resource::typst_resource::cache::{TypstRaster, TypstRasterCache};
use crate::resource::typst_resource::compiler::TypstBackend;
use crate::resource::typst_resource::raster::{
    normalized_world_height_from_metrics as normalized_typst_world_height, rasterize_svg_to_rgba,
};
use hecs::{Entity, World};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

/// Manages the mapping between Frontend Tattvas and their materialized ECS entities.
pub struct SyncBoundary {
    /// Maps TattvaId to the list of entities representing its current geometry.
    pub entity_cache: HashMap<TattvaId, Vec<Entity>>,
    label_resources: Option<LabelResources>,
    text_bind_group: Option<Arc<wgpu::BindGroup>>,
    latex_cache_dir: PathBuf,
    typst_backend: Option<TypstBackend>,
    typst_cache: TypstRasterCache,
    reported_runtime_errors: HashSet<String>,
}

impl SyncBoundary {
    const REBUILD_FLAGS: DirtyFlags = DirtyFlags::REBUILD;

    pub fn new() -> Self {
        Self {
            entity_cache: HashMap::new(),
            label_resources: None,
            text_bind_group: None,
            latex_cache_dir: std::env::temp_dir().join("murali_latex_cache"),
            typst_backend: None,
            typst_cache: TypstRasterCache::new(128),
            reported_runtime_errors: HashSet::new(),
        }
    }

    /// The core sync loop. Called once per frame by the Engine.
    ///
    /// NOTE:
    /// - Projection stays CPU-only.
    /// - GPU upload happens *here*.
    pub fn sync_tattva(
        &mut self,
        world: &mut World,
        device: &wgpu::Device,
        renderer: &Renderer,
        tattva: &mut dyn TattvaTrait,
    ) {
        let dirty = tattva.dirty_flags();
        if dirty.is_empty() {
            return;
        }

        if dirty.intersects(Self::REBUILD_FLAGS) {
            self.rebuild_render_entities(world, device, renderer, tattva);
            tattva.clear_all_dirty();
            return;
        }

        self.sync_runtime_only(tattva);
    }

    fn sync_runtime_only(&mut self, tattva: &mut dyn TattvaTrait) {
        tattva.clear_dirty(DirtyFlags::TRANSFORM | DirtyFlags::STYLE | DirtyFlags::VISIBILITY);
    }

    fn rebuild_render_entities(
        &mut self,
        world: &mut World,
        device: &wgpu::Device,
        renderer: &Renderer,
        tattva: &mut dyn TattvaTrait,
    ) {
        self.despawn_cached_entities(world, tattva.id());
        let primitives = self.project_tattva(tattva);
        let entities = self.materialize_primitives(world, device, renderer, tattva, primitives);
        self.entity_cache.insert(tattva.id(), entities);
    }

    fn despawn_cached_entities(&mut self, world: &mut World, tattva_id: TattvaId) {
        if let Some(old_entities) = self.entity_cache.remove(&tattva_id) {
            for entity in old_entities {
                let _ = world.despawn(entity);
            }
        }
    }

    fn project_tattva(&self, tattva: &dyn TattvaTrait) -> Vec<RenderPrimitive> {
        let mut ctx = ProjectionCtx::new(tattva.props().clone());
        tattva.project(&mut ctx);
        ctx.primitives
    }

    fn materialize_primitives(
        &mut self,
        world: &mut World,
        device: &wgpu::Device,
        renderer: &Renderer,
        tattva: &dyn TattvaTrait,
        primitives: Vec<RenderPrimitive>,
    ) -> Vec<Entity> {
        let mut new_entities = Vec::new();

        for primitive in primitives {
            let entity = match primitive {
                RenderPrimitive::Mesh(mesh) => {
                    upload_mesh(device, mesh.as_ref(), None).map(|mesh_instance| {
                        world.spawn((
                            MeshComponent(Arc::new(mesh_instance)),
                            tattva.props().clone(),
                        ))
                    })
                }
                RenderPrimitive::Line {
                    start,
                    end,
                    thickness,
                    color,
                    dash_length,
                    gap_length,
                    dash_offset,
                } => Some(world.spawn((
                    LineComponent {
                        start,
                        end,
                        thickness,
                        dash_length,
                        gap_length,
                        dash_offset,
                    },
                    ColorComponent(color),
                    tattva.props().clone(),
                ))),
                RenderPrimitive::Text {
                    content,
                    height,
                    color,
                    offset,
                } => self
                    .build_label_instance(device, renderer, &content, height, color, offset)
                    .map(|mesh_instance| {
                        world.spawn((
                            MeshComponent(Arc::new(mesh_instance)),
                            tattva.props().clone(),
                        ))
                    }),
                RenderPrimitive::Latex {
                    source,
                    height,
                    color,
                    offset,
                } => self
                    .build_latex_instance(device, renderer, &source, height, color, offset)
                    .map(|mesh_instance| {
                        world.spawn((
                            MeshComponent(Arc::new(mesh_instance)),
                            tattva.props().clone(),
                        ))
                    }),
                RenderPrimitive::Typst {
                    source,
                    height,
                    color,
                    offset,
                } => self
                    .build_typst_instance(device, renderer, &source, height, color, offset)
                    .map(|mesh_instance| {
                        world.spawn((
                            MeshComponent(Arc::new(mesh_instance)),
                            tattva.props().clone(),
                        ))
                    }),
            };

            if let Some(entity) = entity {
                new_entities.push(entity);
            }
        }

        new_entities
    }

    fn build_label_instance(
        &mut self,
        device: &wgpu::Device,
        renderer: &Renderer,
        content: &str,
        height: f32,
        color: glam::Vec4,
        offset: glam::Vec3,
    ) -> Option<MeshInstance> {
        if self.label_resources.is_none() {
            self.label_resources = Some(LabelResources::new());
        }
        let resources = self.label_resources.as_ref()?;

        if self.text_bind_group.is_none() {
            self.text_bind_group = Some(
                renderer
                    .create_text_bind_group_from_raster(
                        &resources.atlas.rgba,
                        resources.atlas.width,
                        resources.atlas.height,
                    )
                    .into(),
            );
        }

        let layout = layout_label(&resources.font, content, height);
        let mesh = build_label_mesh(&layout, &resources.atlas, color);
        let mesh = translate_mesh(mesh.as_ref(), offset);
        upload_mesh(device, &mesh, self.text_bind_group.clone())
    }

    fn build_latex_instance(
        &mut self,
        device: &wgpu::Device,
        renderer: &Renderer,
        source: &str,
        height: f32,
        color: glam::Vec4,
        offset: glam::Vec3,
    ) -> Option<MeshInstance> {
        let latex = match compile_latex(source, &self.latex_cache_dir) {
            Ok(latex) => latex,
            Err(error) => {
                self.report_once(
                    format!("latex-compile::{error}"),
                    format!("LaTeX compile failed for `{source}`: {error}"),
                );
                return None;
            }
        };

        let raster = match rasterize_svg(
            &latex.svg_path,
            renderer.device_mgr.config.borrow().height as f32 / 4.0,
            renderer.device_mgr.max_texture_size(),
        ) {
            Ok(raster) => raster,
            Err(error) => {
                self.report_once(
                    format!("latex-raster::{error}"),
                    format!("LaTeX rasterization failed for `{source}`: {error}"),
                );
                return None;
            }
        };

        let bind_group =
            renderer.create_text_bind_group_from_raster(&raster.rgba, raster.width, raster.height);
        let world_height = normalized_world_height(height, &raster);
        let mesh = build_textured_quad(raster.width, raster.height, world_height, color);
        let mesh = translate_mesh(mesh.as_ref(), offset);
        upload_mesh(device, &mesh, Some(Arc::new(bind_group)))
    }

    fn build_typst_instance(
        &mut self,
        device: &wgpu::Device,
        renderer: &Renderer,
        source: &str,
        height: f32,
        color: glam::Vec4,
        offset: glam::Vec3,
    ) -> Option<MeshInstance> {
        let cache_key = format!("{height:.4}::{source}");
        let raster = if let Some(existing) = self.typst_cache.get(&cache_key) {
            existing
        } else {
            if self.typst_backend.is_none() {
                self.typst_backend = TypstBackend::new().ok();
            }
            let backend = match self.typst_backend.as_ref() {
                Some(backend) => backend,
                None => {
                    self.report_once(
                        "typst-backend-init".to_string(),
                        "Typst backend initialization failed".to_string(),
                    );
                    return None;
                }
            };

            let svg = match backend.render_to_svg(source, height * 36.0) {
                Ok(svg) => svg,
                Err(error) => {
                    self.report_once(
                        format!("typst-compile::{error}"),
                        format!("Typst compilation failed for `{source}`: {error}"),
                    );
                    return None;
                }
            };

            let scale =
                ((renderer.device_mgr.config.borrow().height as f32) / 4.0 / height.max(0.1))
                    .clamp(1.0, 8.0);
            let rasterized = match rasterize_svg_to_rgba(&svg, scale) {
                Ok(result) => result,
                Err(error) => {
                    self.report_once(
                        format!("typst-raster::{error}"),
                        format!("Typst rasterization failed for `{source}`: {error}"),
                    );
                    return None;
                }
            };

            self.typst_cache.insert(
                cache_key.clone(),
                TypstRaster {
                    rgba: rasterized.rgba,
                    width: rasterized.width,
                    height: rasterized.height,
                    normalized_height_px: rasterized.normalized_height_px,
                    svg: Some(svg),
                },
            );
            self.typst_cache.get(&cache_key)?
        };

        let bind_group =
            renderer.create_text_bind_group_from_raster(&raster.rgba, raster.width, raster.height);
        let world_height =
            normalized_typst_world_height(height, raster.height, raster.normalized_height_px);
        let mesh = build_textured_quad(raster.width, raster.height, world_height, color);
        let mesh = translate_mesh(mesh.as_ref(), offset);
        upload_mesh(device, &mesh, Some(Arc::new(bind_group)))
    }

    fn report_once(&mut self, key: String, message: String) {
        if self.reported_runtime_errors.insert(key) {
            eprintln!("{message}");
        }
    }
}

fn upload_mesh(
    device: &wgpu::Device,
    mesh: &crate::projection::Mesh,
    bind_group: Option<Arc<wgpu::BindGroup>>,
) -> Option<MeshInstance> {
    match &mesh.data {
        MeshData::Empty => None,
        MeshData::Mesh(vertices) => {
            let vertex_bytes = bytemuck::cast_slice(vertices);
            let index_bytes = bytemuck::cast_slice(&mesh.indices);
            Some(MeshInstance::new(
                device,
                vertex_bytes,
                index_bytes,
                mesh.indices.len() as u32,
                bind_group,
                MeshPipelineKind::Mesh,
            ))
        }
        MeshData::Text(vertices) => {
            let vertex_bytes = bytemuck::cast_slice(vertices);
            let index_bytes = bytemuck::cast_slice(&mesh.indices);
            Some(MeshInstance::new(
                device,
                vertex_bytes,
                index_bytes,
                mesh.indices.len() as u32,
                bind_group,
                MeshPipelineKind::Text,
            ))
        }
    }
}

fn translate_mesh(mesh: &crate::projection::Mesh, offset: glam::Vec3) -> crate::projection::Mesh {
    match &mesh.data {
        MeshData::Empty => mesh.clone(),
        MeshData::Mesh(vertices) => {
            let translated = vertices
                .iter()
                .map(|v| {
                    let mut v = *v;
                    v.position[0] += offset.x;
                    v.position[1] += offset.y;
                    v.position[2] += offset.z;
                    v
                })
                .collect();
            crate::projection::Mesh {
                data: MeshData::Mesh(translated),
                indices: mesh.indices.clone(),
            }
        }
        MeshData::Text(vertices) => {
            let translated = vertices
                .iter()
                .map(|v| {
                    let mut v = *v;
                    v.position[0] += offset.x;
                    v.position[1] += offset.y;
                    v.position[2] += offset.z;
                    v
                })
                .collect();
            crate::projection::Mesh {
                data: MeshData::Text(translated),
                indices: mesh.indices.clone(),
            }
        }
    }
}
