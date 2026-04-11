use glam::{Vec3, Vec4};
use crate::projection::{ProjectionCtx, RenderPrimitive};
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

/// Trait for custom node content.
///
/// Receives the projection context, the node's world-space centre, and the
/// current step state. Emit `RenderPrimitive`s to render the node.
///
/// Stepwise handles layout, transitions, and the signal dot.
/// Content handles everything drawn inside (or instead of) the node.
pub trait StepContent: Send + Sync {
    fn project(&self, ctx: &mut ProjectionCtx, position: Vec3, state: &StepState);

    /// Return `true` if this content draws its own background container.
    /// When `false` (default), Stepwise draws the standard background rect
    /// behind the content, giving it a consistent visual frame.
    fn draws_own_background(&self) -> bool { false }
}

// ── TattvaContent adapter ────────────────────────────────────────────────────

/// Wraps any `Project` implementor as `StepContent`.
///
/// The inner tattva renders at its own origin; `TattvaContent` translates all
/// emitted primitives to the node's world-space position and scales their
/// opacity by the step state.
///
/// This means existing tattvas (`Label`, `AgenticFlowChart`, etc.) can be
/// embedded in Stepwise nodes with zero changes to their own code:
///
/// ```rust
/// s.step_with("Reason", TattvaContent::new(Label::new("Reason", 0.4)));
/// ```
pub struct TattvaContent<T: crate::projection::Project + Send + Sync> {
    pub inner: T,
}

impl<T: crate::projection::Project + Send + Sync> TattvaContent<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: crate::projection::Project + Send + Sync> StepContent for TattvaContent<T> {
    fn project(&self, ctx: &mut ProjectionCtx, position: Vec3, state: &StepState) {
        let opacity = match state {
            StepState::Pending        => 0.0,  // invisible until introduced
            StepState::Active { t }   => *t,
            StepState::Completed      => 0.75,
        };

        // Collect primitives from the inner tattva into a temporary context.
        let mut sub = ProjectionCtx::new(ctx.props.clone());
        self.inner.project(&mut sub);

        // Re-emit with position offset and opacity applied.
        for prim in sub.primitives {
            ctx.emit(translate_and_fade(prim, position, opacity));
        }
    }
}

/// Translate all position data in a primitive by `offset` and multiply alpha
/// by `opacity`.
fn translate_and_fade(prim: RenderPrimitive, offset: Vec3, opacity: f32) -> RenderPrimitive {
    match prim {
        RenderPrimitive::Mesh(mesh) => {
            RenderPrimitive::Mesh(mesh.as_ref().translated(offset))
            // Note: per-vertex alpha is baked into mesh colors; we can't easily
            // scale it here without re-tessellating. Opacity is handled by the
            // tattva's DrawableProps at the scene level for standalone tattvas,
            // but inside StepContent we rely on the content author to respect
            // state. TattvaContent is a best-effort adapter.
        }
        RenderPrimitive::Line { start, end, thickness, color, dash_length, gap_length, dash_offset } => {
            RenderPrimitive::Line {
                start: start + offset,
                end:   end   + offset,
                thickness,
                color: fade(color, opacity),
                dash_length,
                gap_length,
                dash_offset,
            }
        }
        RenderPrimitive::Text { content, height, color, offset: text_offset } => {
            RenderPrimitive::Text {
                content,
                height,
                color:  fade(color, opacity),
                offset: text_offset + offset,
            }
        }
        RenderPrimitive::Latex { source, height, color, offset: text_offset } => {
            RenderPrimitive::Latex {
                source,
                height,
                color:  fade(color, opacity),
                offset: text_offset + offset,
            }
        }
        RenderPrimitive::Typst { source, height, color, offset: text_offset } => {
            RenderPrimitive::Typst {
                source,
                height,
                color:  fade(color, opacity),
                offset: text_offset + offset,
            }
        }
    }
}

fn fade(color: Vec4, opacity: f32) -> Vec4 {
    Vec4::new(color.x, color.y, color.z, color.w * opacity)
}
