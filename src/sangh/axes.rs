// src/sangh/axes.rs

use glam::{Vec3, Vec4};
use crate::core::sync::{Syncable, SyncBoundary};
use crate::renderer::vertex::line::LineComponent;
use crate::renderer::vertex::color::ColorComponent;
use hecs::World;

pub struct Axes {
    pub x_range: (f32, f32),
    pub y_range: (f32, f32),
    pub x_step: f32,
    pub y_step: f32,
    pub thickness: f32,
    pub color: Vec4,
    pub tick_size: f32,
    
    // Internal tracking for the Sync Boundary
    entity_indices: Vec<usize>, 
    dirty: bool,
}

impl Axes {
    pub fn new(x_range: (f32, f32), y_range: (f32, f32)) -> Self {
        Self {
            x_range,
            y_range,
            x_step: 1.0,
            y_step: 1.0,
            thickness: 0.05,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            tick_size: 0.2,
            entity_indices: Vec::new(),
            dirty: true,
        }
    }

    pub fn set_x_range(&mut self, min: f32, max: f32) {
        self.x_range = (min, max);
        self.dirty = true;
    }

    // ... other setters that mark self.dirty = true ...
}

impl Syncable for Axes {
    fn sync_to_world(&mut self, world: &mut World, boundary: &mut SyncBoundary) {
        if !self.dirty {
            return;
        }

        // 1. Clear previous entities associated with this Sangh from the ECS
        for idx in self.entity_indices.drain(..) {
            if let Some(entity) = boundary.entity_map.remove(&idx) {
                let _ = world.despawn(entity);
            }
        }

        let mut current_idx = 0;

        // 2. Project: X-Axis Main Line
        let x_line = LineComponent::new(
            Vec3::new(self.x_range.0, 0.0, 0.0),
            Vec3::new(self.x_range.1, 0.0, 0.0),
            self.thickness
        );
        spawn_line(world, boundary, &mut self.entity_indices, &mut current_idx, x_line, self.color);

        // 3. Project: Y-Axis Main Line
        let y_line = LineComponent::new(
            Vec3::new(0.0, self.y_range.0, 0.0),
            Vec3::new(0.0, self.y_range.1, 0.0),
            self.thickness
        );
        spawn_line(world, boundary, &mut self.entity_indices, &mut current_idx, y_line, self.color);

        // 4. Project: Ticks (X)
        if self.x_step > 0.0 {
            let mut x = self.x_range.0;
            while x <= self.x_range.1 {
                if x.abs() > 0.001 { // Skip origin
                    let tick = LineComponent::new(
                        Vec3::new(x, -self.tick_size * 0.5, 0.0),
                        Vec3::new(x, self.tick_size * 0.5, 0.0),
                        self.thickness * 0.5
                    );
                    spawn_line(world, boundary, &mut self.entity_indices, &mut current_idx, tick, self.color);
                }
                x += self.x_step;
            }
        }

        // 5. Project: Ticks (Y)
        if self.y_step > 0.0 {
            let mut y = self.y_range.0;
            while y <= self.y_range.1 {
                if y.abs() > 0.001 { // Skip origin
                    let tick = LineComponent::new(
                        Vec3::new(-self.tick_size * 0.5, y, 0.0),
                        Vec3::new(self.tick_size * 0.5, y, 0.0),
                        self.thickness * 0.5
                    );
                    spawn_line(world, boundary, &mut self.entity_indices, &mut current_idx, tick, self.color);
                }
                y += self.y_step;
            }
        }

        self.dirty = false;
    }
}

/// Helper to manage the boundary mapping
fn spawn_line(
    world: &mut World, 
    boundary: &mut SyncBoundary, 
    indices: &mut Vec<usize>, 
    current_idx: &mut usize,
    line: LineComponent,
    color: Vec4
) {
    let entity = world.spawn((line, ColorComponent(color)));
    boundary.entity_map.insert(*current_idx, entity);
    indices.push(*current_idx);
    *current_idx += 1;
}