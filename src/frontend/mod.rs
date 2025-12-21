// src/frontend/mod.rs

pub mod props;
pub mod layout;
pub mod animation;
pub mod collection;
pub mod tattva_trait;

use props::DrawableProps;
use crate::frontend::props::SharedProps;
use std::sync::Arc;

pub type TattvaId = usize;

pub struct Tattva<T> {
    pub id: TattvaId,
    pub state: T,
    pub props: SharedProps,
    dirty: bool,
}

impl<T> Tattva<T> {
    pub fn new(id: TattvaId, state: T) -> Self {
        Self {
            id,
            state,
            props: SharedProps::default(),
            dirty: true, // New objects are always dirty
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

    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.state);
        self.mark_dirty();
    }
}
