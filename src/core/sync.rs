use crate::sangh::Sangh;
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::ecs::components::*;
use hecs::{World, Entity};
use std::collections::HashMap;

pub trait Syncable {
    fn sync_to_world(&mut self, world: &mut World, boundary: &mut SyncBoundary);
}

impl<T: Project + 'static> Syncable for Sangh<T> {
    fn sync_to_world(&mut self, world: &mut World, boundary: &mut SyncBoundary) {
        boundary.sync_sangh(world, self);
    }
}

// This helper type allows us to downcast in the Scene
pub type BoxedSangh = dyn Syncable;

/// Manages the mapping between a Sangh and its cached ECS entities.
pub struct SyncBoundary {
    // Maps SanghId -> List of Entities representing it in the ECS
    cache: HashMap<u64, Vec<Entity>>,
}

impl SyncBoundary {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn sync_sangh<T: Project>(
        &mut self, 
        world: &mut World, 
        sangh: &mut Sangh<T>
    ) {
        // 1. Check if we actually need to re-project math
        if sangh.is_dirty() {
            let mut ctx = ProjectionCtx::new();
            sangh.state.project(&mut ctx);

            // Clear old cache for this Sangh
            if let Some(old_entities) = self.cache.remove(&sangh.id) {
                for entity in old_entities {
                    let _ = world.despawn(entity);
                }
            }

            // Spawn new entities from primitives
            let mut new_entities = Vec::new();
            for primitive in ctx.primitives {
                let entity = match primitive {
                    RenderPrimitive::Line { start, end, thickness, color } => {
                        world.spawn((
                            LineComponent { start, end, thickness },
                            ColorComponent(color),
                            sangh.props.clone(), // Every entity shares the Sangh's transform
                        ))
                    }
                    RenderPrimitive::Quad { size, texture_id, color } => {
                        world.spawn((
                            QuadComponent { size, texture_id },
                            ColorComponent(color),
                            sangh.props.clone(),
                        ))
                    }
                };
                new_entities.push(entity);
            }
            
            self.cache.insert(sangh.id, new_entities);
            sangh.clear_dirty();
        }
    }
}