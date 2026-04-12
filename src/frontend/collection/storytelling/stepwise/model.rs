// src/frontend/collection/storytelling/stepwise/model.rs
use glam::Vec3;
use crate::projection::{ProjectionCtx, Project};
use super::state::StepState;

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Step {
    pub label: String,
    pub content: Option<Box<dyn StepContent>>,
}

impl std::fmt::Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Step")
            .field("label", &self.label)
            .field("content", &self.content.as_ref().map(|_| "<StepContent>"))
            .finish()
    }
}

impl Clone for Step {
    fn clone(&self) -> Self {
        Self { label: self.label.clone(), content: None }
    }
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: usize,
    pub to: usize,
    pub route: Option<Vec<Direction>>,
}

#[derive(Debug, Clone)]
pub struct StepwiseModel {
    pub steps: Vec<Step>,
    pub transitions: Vec<Transition>,
    pub sequence: Vec<usize>,
}

// ── StepContent trait ────────────────────────────────────────────────────────

pub trait StepContent: Send + Sync {
    fn project(&self, ctx: &mut ProjectionCtx, state: &StepState);

    /// Return `true` if this content draws its own background container.
    fn draws_own_background(&self) -> bool { false }
}

// ── TattvaContent adapter ────────────────────────────────────────────────────

pub struct TattvaContent<T: Project + Send + Sync> {
    pub inner: T,
}

impl<T: Project + Send + Sync> TattvaContent<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Project + Send + Sync> StepContent for TattvaContent<T> {
    fn project(&self, ctx: &mut ProjectionCtx, state: &StepState) {
        let opacity = match state {
            StepState::Pending        => 0.0,
            StepState::Active { t }   => *t,
            StepState::Completed      => 0.75,
        };

        // Use the new tiered opacity stack in the engine:
        ctx.with_opacity(opacity, |ctx| {
            self.inner.project(ctx);
        });
    }
}
