pub mod shapes;
pub mod axes;

use crate::projection::Project;
use crate::scene::{DrawableProps, SharedProps};

pub struct Sangh<T: Project> {
    pub id: u64,
    pub state: T,
    pub props: SharedProps,
    dirty: bool,
}

impl<T: Project> Sangh<T> {
    pub fn new(id: u64, state: T) -> Self {
        Self {
            id,
            state,
            props: std::sync::Arc::new(parking_lot::RwLock::new(DrawableProps::default())),
            dirty: true, // New objects must be projected immediately
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}
