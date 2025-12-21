use crate::core::sync::Syncable;
use crate::sangh::Sangh;
use crate::projection::Project;

pub struct SanghRegistry {
    pub entries: Vec<Box<dyn Syncable>>,
}

impl SanghRegistry {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add<T: Project + 'static>(&mut self, sangh: Sangh<T>) {
        // Because Sangh<T> implements Syncable, we can cast it to a Boxed trait object
        self.entries.push(Box::new(sangh));
    }
}