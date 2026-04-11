//! `Stepwise` — a first-class Murali tattva.
//!
//! Usage:
//! ```rust
//! let sw = Stepwise::new(model).with_layout(StepwiseLayout::vertical(2.0));
//! let id = scene.add_tattva(sw, Vec3::ZERO);
//! // Phase 1: build
//! timeline.animate(id).at(0.0).for_duration(5.0).ease(Ease::Linear).propagate_to(1.0).spawn();
//! // Phase 2: signal flow
//! timeline.animate(id).at(6.0).for_duration(3.0).ease(Ease::InOutQuad).signal_to(1.0).spawn();
//! ```

use glam::{Vec2, Vec3, Vec4};

use crate::frontend::animation::Ease;
use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Mesh, Project, ProjectionCtx, RenderPrimitive};

use super::model::StepwiseModel;
use super::state::{StepState, TransitionState};
use super::timeline::TimelineEngine;
use super::layout::{StepwiseDirection, StepwiseLayout};

// ── visual constants ─────────────────────────────────────────────────────────

const NODE_SIZE:       f32  = 1.2;
const STROKE_THICK:    f32  = 0.04;
const LABEL_HEIGHT:    f32  = 0.3;
const SIGNAL_RADIUS:   f32  = 0.1;
const EDGE_THICKNESS:  f32  = 0.04;

const FILL_ACTIVE:     Vec4 = Vec4::new(0.12, 0.18, 0.30, 1.0); // dark blue fill
const FILL_COMPLETED:  Vec4 = Vec4::new(0.18, 0.22, 0.28, 1.0); // muted completed
const STROKE_ACTIVE:   Vec4 = Vec4::new(0.35, 0.70, 1.00, 1.0); // bright blue outline
const STROKE_COMPLETED:Vec4 = Vec4::new(0.40, 0.50, 0.60, 1.0); // muted outline
const COLOR_LABEL:     Vec4 = Vec4::new(0.95, 0.97, 1.00, 1.0);
const FILL_SIGNAL:     Vec4 = Vec4::new(1.0,  0.82, 0.25, 1.0);
const FILL_EDGE:       Vec4 = Vec4::new(0.35, 0.45, 0.60, 1.0);
const COLOR_DEBUG:     Vec4 = Vec4::new(1.0,  1.0,  0.5,  1.0);

// ── Stepwise tattva ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Stepwise {
    pub model:           StepwiseModel,
    pub progress:        f32,
    /// Independent signal flow progress (0→1). Drives the signal dot along the
    /// sequence path. Animate this separately after `progress` reaches 1.0 to
    /// show concept flow on the fully-built diagram.
    pub signal_progress: f32,
    pub layout:          StepwiseLayout,
    pub debug:           bool,
}

impl Stepwise {
    pub fn new(model: StepwiseModel) -> Self {
        Self {
            model,
            progress: 0.0,
            signal_progress: 0.0,
            layout: StepwiseLayout::default(),
            debug: false,
        }
    }

    pub fn with_progress(mut self, p: f32) -> Self {
        self.progress = p.clamp(0.0, 1.0);
        self
    }

    pub fn with_signal_progress(mut self, p: f32) -> Self {
        self.signal_progress = p.clamp(0.0, 1.0);
        self
    }

    pub fn with_layout(mut self, layout: StepwiseLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

// ── write animation helpers ───────────────────────────────────────────────────

fn with_alpha(c: Vec4, a: f32) -> Vec4 {
    Vec4::new(c.x, c.y, c.z, c.w * a)
}

/// Emit the four edges of a square outline, revealing them progressively.
///
/// `t` in [0, 1] traces the perimeter: top → right → bottom → left.
/// Each edge occupies 0.25 of the total perimeter.
fn emit_square_outline(ctx: &mut ProjectionCtx, pos: Vec3, size: f32, color: Vec4, t: f32) {
    let h = size * 0.5;
    // corners: top-left → top-right → bottom-right → bottom-left → top-left
    let corners = [
        Vec3::new(pos.x - h, pos.y + h, pos.z),
        Vec3::new(pos.x + h, pos.y + h, pos.z),
        Vec3::new(pos.x + h, pos.y - h, pos.z),
        Vec3::new(pos.x - h, pos.y - h, pos.z),
        Vec3::new(pos.x - h, pos.y + h, pos.z),
    ];

    let total_t = t.clamp(0.0, 1.0) * 4.0; // 4 edges

    for edge in 0..4 {
        let edge_start_t = edge as f32;
        let edge_end_t   = edge_start_t + 1.0;

        if total_t <= edge_start_t { break; }

        let local_t = ((total_t - edge_start_t) / 1.0).clamp(0.0, 1.0);
        let a = corners[edge];
        let b = corners[edge + 1];
        let end = a + (b - a) * local_t;

        ctx.emit(RenderPrimitive::Line {
            start: a, end,
            thickness: STROKE_THICK,
            color,
            dash_length: 0.0, gap_length: 0.0, dash_offset: 0.0,
        });
    }
}

/// Emit a filled square mesh.
fn emit_square_fill(ctx: &mut ProjectionCtx, pos: Vec3, size: f32, color: Vec4) {
    let mesh = Mesh::square(size, color).as_ref().translated(pos);
    ctx.emit(RenderPrimitive::Mesh(mesh));
}

/// Reveal `n` characters out of `label` based on `t` in [0, 1].
fn revealed_text(label: &str, t: f32) -> String {
    let count = label.chars().count();
    let reveal = (count as f32 * t).ceil() as usize;
    label.chars().take(reveal).collect()
}

// ── node rendering ────────────────────────────────────────────────────────────

/// Manim-style write for the default node:
/// - `t 0.0 → 0.5`: outline strokes draw in around the perimeter
/// - `t 0.5 → 1.0`: fill fades in + label characters reveal
///
/// This is purely a rendering concern — `StepState` drives it, no engine change.
fn render_default_node(ctx: &mut ProjectionCtx, pos: Vec3, state: &StepState, label: &str) {
    match state {
        StepState::Pending => {
            // Invisible — nothing drawn until the step becomes active.
        }

        StepState::Active { t } => {
            let t = Ease::InOutSmooth.eval(*t);

            // Phase 1 (t 0→0.5): draw outline
            let outline_t = (t * 2.0).clamp(0.0, 1.0);
            emit_square_outline(ctx, pos, NODE_SIZE, STROKE_ACTIVE, outline_t);

            // Phase 2 (t 0.5→1.0): fill + label appear
            let fill_t = ((t - 0.5) * 2.0).clamp(0.0, 1.0);
            if fill_t > 0.001 {
                emit_square_fill(ctx, pos, NODE_SIZE, with_alpha(FILL_ACTIVE, fill_t));
                let text = revealed_text(label, fill_t);
                if !text.is_empty() {
                    ctx.emit(RenderPrimitive::Text {
                        content: text,
                        height:  LABEL_HEIGHT,
                        color:   with_alpha(COLOR_LABEL, fill_t),
                        offset:  pos + Vec3::new(0.0, 0.0, 0.1),
                    });
                }
            }
        }

        StepState::Completed => {
            // Fully drawn: fill + label + muted outline.
            emit_square_fill(ctx, pos, NODE_SIZE, FILL_COMPLETED);
            emit_square_outline(ctx, pos, NODE_SIZE, STROKE_COMPLETED, 1.0);
            ctx.emit(RenderPrimitive::Text {
                content: label.to_string(),
                height:  LABEL_HEIGHT,
                color:   with_alpha(COLOR_LABEL, 0.8),
                offset:  pos + Vec3::new(0.0, 0.0, 0.1),
            });
        }
    }
}

/// Background for custom content that opts into the standard container.
fn render_background(ctx: &mut ProjectionCtx, pos: Vec3, state: &StepState) {
    match state {
        StepState::Pending => {}
        StepState::Active { t } => {
            let t = Ease::InOutSmooth.eval(*t);
            let outline_t = (t * 2.0).clamp(0.0, 1.0);
            emit_square_outline(ctx, pos, NODE_SIZE, STROKE_ACTIVE, outline_t);
            let fill_t = ((t - 0.5) * 2.0).clamp(0.0, 1.0);
            if fill_t > 0.001 {
                emit_square_fill(ctx, pos, NODE_SIZE, with_alpha(FILL_ACTIVE, fill_t));
            }
        }
        StepState::Completed => {
            emit_square_fill(ctx, pos, NODE_SIZE, FILL_COMPLETED);
            emit_square_outline(ctx, pos, NODE_SIZE, STROKE_COMPLETED, 1.0);
        }
    }
}

// ── Project ───────────────────────────────────────────────────────────────────

impl Project for Stepwise {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let state = TimelineEngine::compute(&self.model, self.progress);

        // ── transitions ───────────────────────────────────────────────────
        for (i, transition) in self.model.transitions.iter().enumerate() {
            let from = self.layout.position_for(transition.from);
            let to   = self.layout.position_for(transition.to);
            // TODO: use transition.route (Vec<Direction>) for non-straight paths (v2)

            match &state.transitions[i] {
                TransitionState::Hidden => {}

                TransitionState::Drawing { t } => {
                    // Line grows from source toward destination.
                    let eased = Ease::InOutSmooth.eval(*t);
                    let end   = from + (to - from) * eased;
                    ctx.emit(RenderPrimitive::Line {
                        start: from, end,
                        thickness: EDGE_THICKNESS,
                        color: FILL_EDGE,
                        dash_length: 0.0, gap_length: 0.0, dash_offset: 0.0,
                    });
                }

                TransitionState::Completed => {
                    ctx.emit(RenderPrimitive::Line {
                        start: from, end: to,
                        thickness: EDGE_THICKNESS,
                        color: with_alpha(FILL_EDGE, 0.6),
                        dash_length: 0.0, gap_length: 0.0, dash_offset: 0.0,
                    });
                }
            }
        }

        // ── nodes ─────────────────────────────────────────────────────────
        for (i, step) in self.model.steps.iter().enumerate() {
            let step_state = &state.steps[i];
            let pos = self.layout.position_for(i);

            match &step.content {
                None => render_default_node(ctx, pos, step_state, &step.label),
                Some(content) => {
                    if !content.draws_own_background() {
                        render_background(ctx, pos, step_state);
                    }
                    content.project(ctx, pos, step_state);
                }
            }
        }

        // ── signal dot ────────────────────────────────────────────────────
        // Driven by `signal_progress` (0→1), independent of `progress`.
        // Travels along the sequence path so it can be animated separately
        // after the diagram is fully built.
        if self.signal_progress > 0.001 {
            let n = self.model.sequence.len();
            if n >= 2 {
                // Map signal_progress across the full sequence path.
                let total_hops = (n - 1) as f32;
                let raw = self.signal_progress.clamp(0.0, 1.0) * total_hops;
                let hop = (raw.floor() as usize).min(n - 2);
                let t   = raw - hop as f32;

                let from_idx = self.model.sequence[hop];
                let to_idx   = self.model.sequence[hop + 1];
                let eased    = Ease::InOutSmooth.eval(t);
                let p        = self.layout.lerp_position(from_idx, to_idx, eased);

                let mesh = Mesh::circle(SIGNAL_RADIUS, 20, FILL_SIGNAL)
                    .as_ref()
                    .translated(Vec3::new(p.x, p.y, 0.2));
                ctx.emit(RenderPrimitive::Mesh(mesh));
            }
        }

        // ── debug overlay ─────────────────────────────────────────────────
        if self.debug {
            let base_y = 3.0_f32;
            let step_y = -0.5_f32;
            let x      = -8.0_f32;

            for (i, step) in self.model.steps.iter().enumerate() {
                let text = match &state.steps[i] {
                    StepState::Completed    => format!("[✓] {}", step.label),
                    StepState::Active { t } => format!("[→] {} ({:.2})", step.label, t),
                    StepState::Pending      => format!("[ ] {}", step.label),
                };
                ctx.emit(RenderPrimitive::Text {
                    content: text,
                    height:  0.25,
                    color:   COLOR_DEBUG,
                    offset:  Vec3::new(x, base_y + i as f32 * step_y, 0.5),
                });
            }

            if self.signal_progress > 0.001 {
                ctx.emit(RenderPrimitive::Text {
                    content: format!("signal: {:.2}", self.signal_progress),
                    height:  0.25,
                    color:   COLOR_DEBUG,
                    offset:  Vec3::new(x, base_y + self.model.steps.len() as f32 * step_y, 0.5),
                });
            }
        }
    }
}

// ── Bounded ───────────────────────────────────────────────────────────────────

impl Bounded for Stepwise {
    fn local_bounds(&self) -> Bounds {
        let n = self.model.steps.len();
        if n == 0 {
            return Bounds::from_center_size(Vec2::ZERO, glam::vec2(0.01, 0.01));
        }
        let pad  = NODE_SIZE * 0.5 + SIGNAL_RADIUS;
        let last = self.layout.position_for(n - 1);
        match self.layout.direction {
            StepwiseDirection::Horizontal => Bounds::new(
                Vec2::new(-pad, -pad),
                Vec2::new(last.x + pad, pad),
            ),
            StepwiseDirection::Vertical => Bounds::new(
                Vec2::new(-pad, last.y - pad),
                Vec2::new(pad, pad),
            ),
        }
    }
}
