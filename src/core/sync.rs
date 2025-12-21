// src/core/sync.rs

use crate::ecs::components::*;
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::sangh::Sangh;
use hecs::{Entity, World};
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
    pub entity_map: HashMap<usize, Entity>,
    pub cache: HashMap<u64, Vec<Entity>>,
}

impl SyncBoundary {
    pub fn new() -> Self {
        Self {
            entity_map: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    pub fn sync_sangh<T: Project>(&mut self, world: &mut World, sangh: &mut Sangh<T>) {
        // 1. If geometry changed, we must re-project
        if sangh.is_dirty() {
            let mut ctx = ProjectionCtx::new();
            sangh.state.project(&mut ctx);

            if let Some(old_entities) = self.cache.remove(&sangh.id) {
                for entity in old_entities {
                    let _ = world.despawn(entity);
                }
            }

            let mut new_entities = Vec::new();
            for primitive in ctx.primitives {
                let entity = match primitive {
                    RenderPrimitive::Line {
                        start,
                        end,
                        thickness,
                        color,
                    } => {
                        world.spawn((
                            LineComponent {
                                start,
                                end,
                                thickness,
                            },
                            ColorComponent(color),
                            sangh.props.clone(), // This clones the ARC, not the data
                        ))
                    }
                    RenderPrimitive::Quad {
                        size,
                        texture_id,
                        color,
                    } => world.spawn((
                        QuadComponent { size, texture_id },
                        ColorComponent(color),
                        sangh.props.clone(),
                    )),
                };
                new_entities.push(entity);
            }

            self.cache.insert(sangh.id, new_entities);
            sangh.clear_dirty();
        } else {
            // 2. OPTIONAL: If the geometry isn't dirty, we don't do anything!
            // Because the ECS entities already hold an Arc to sangh.props,
            // any animation that mutates sangh.props will be "seen" by the
            // renderer automatically without any work here.
        }
    }
}
