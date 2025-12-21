use crate::backend::ecs::components::*;
use crate::projection::{ProjectionCtx, RenderPrimitive};
use crate::frontend::TattvaId;
use crate::frontend::tattva_trait::TattvaTrait;
use crate::backend::renderer::mesh::MeshInstance;
use hecs::{Entity, World};
use std::collections::HashMap;
use std::sync::Arc;

/// Manages the mapping between Frontend Tattvas and their materialized ECS entities.
pub struct SyncBoundary {
    /// Maps TattvaId to the list of entities representing its current geometry.
    pub entity_cache: HashMap<TattvaId, Vec<Entity>>,
}

impl SyncBoundary {
    pub fn new() -> Self {
        Self {
            entity_cache: HashMap::new(),
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
        tattva: &mut dyn TattvaTrait,
    ) {
        // Only re-project if structural state changed
        if !tattva.is_dirty() {
            return;
        }

        // Despawn old entities
        if let Some(old_entities) = self.entity_cache.remove(&tattva.id()) {
            for entity in old_entities {
                let _ = world.despawn(entity);
            }
        }

        // Run projection (CPU-only)
        let mut ctx = ProjectionCtx::new(tattva.props().clone());
        tattva.project(&mut ctx);

        let mut new_entities = Vec::new();

        for primitive in ctx.primitives {
            let entity = match primitive {
                // -----------------------------
                // CPU Mesh → GPU MeshInstance
                // -----------------------------
                RenderPrimitive::Mesh(mesh) => {
                    let mesh_instance = mesh
                        .into_gpu_instance(device, None)
                        .expect("Mesh had no GPU data");

                    world.spawn((
                        MeshComponent(Arc::new(mesh_instance)),
                        tattva.props().clone(),
                    ))
                }

                RenderPrimitive::Line { start, end, thickness, color } => {
                    world.spawn((
                        LineComponent { start, end, thickness },
                        ColorComponent(color),
                        tattva.props().clone(),
                    ))
                }

                RenderPrimitive::Text { content, height, color } => {
                    world.spawn((
                        TextComponent { content, height },
                        ColorComponent(color),
                        tattva.props().clone(),
                    ))
                }

                RenderPrimitive::Latex { source, height, color } => {
                    world.spawn((
                        LatexComponent { source, height },
                        ColorComponent(color),
                        tattva.props().clone(),
                    ))
                }

                RenderPrimitive::Typst { source, height, color } => {
                    world.spawn((
                        TypstComponent { source, height },
                        ColorComponent(color),
                        tattva.props().clone(),
                    ))
                }
            };

            new_entities.push(entity);
        }

        // Cache and finalize
        self.entity_cache.insert(tattva.id(), new_entities);
        tattva.clear_dirty();
    }
}
