use hecs::World;
use crate::core::timeline::Timeline;
use crate::core::sync::{SyncBoundary, Syncable}; // Ensure Syncable is public in sync.rs
use crate::core::registry::SanghRegistry;
use crate::projection::Project; // Import directly from projection

pub struct Scene {
    pub world: World,
    pub timeline: Timeline,
    sync_boundary: SyncBoundary,
    registry: SanghRegistry,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            timeline: Timeline::new(),
            sync_boundary: SyncBoundary::new(),
            registry: SanghRegistry::new(),
        }
    }

    // pub fn sync(&mut self) {
    //     // We need to borrow world and sync_boundary separately to satisfy borrow checker
    //     let world = &mut self.world;
    //     let boundary = &mut self.sync_boundary;

    //     for entry in self.registry.entries.iter_mut() {
    //         // This requires the Syncable trait we defined in sync.rs
    //         if let Some(syncable) = entry.downcast_mut::<crate::core::sync::BoxedSangh>() {
    //              syncable.sync_to_world(world, boundary);
    //         }
    //     }
    // }

    pub fn update(&mut self, dt: f32) {
        self.timeline.tick(dt);
        self.sync();
    }

    pub fn sync(&mut self) {
        // We split borrows to satisfy the borrow checker
        let world = &mut self.world;
        let boundary = &mut self.sync_boundary;

        for entry in self.registry.entries.iter_mut() {
            // No more Any or downcasting! 
            // We just call the trait method.
            entry.sync_to_world(world, boundary);
        }
    }
    
    // Helper to add objects to the scene
    pub fn add<T: Project + 'static>(&mut self, sangh: crate::sangh::Sangh<T>) {
        self.registry.add(sangh);
    }
}