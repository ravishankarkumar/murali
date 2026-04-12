// src/frontend/collection/storytelling/stepwise/tattva.rs
//! `Stepwise` — a first-class Murali tattva.

use glam::{Vec3, Vec4};

use crate::frontend::animation::Ease;
use crate::frontend::layout::{Bounded, Bounds};
use crate::frontend::collection::primitives::path::Path;
use crate::frontend::collection::primitives::square::Square;
use crate::frontend::collection::primitives::to_path::ToPath;
use crate::frontend::collection::text::label::Label;
use crate::frontend::style::{ColorSource, StrokeParams, Style};
use crate::projection::{Project, ProjectionCtx};

use super::layout::{StepwiseDirection, StepwiseLayout};
use super::model::StepwiseModel;
use super::state::{StepState, TransitionState};
use super::timeline::TimelineEngine;

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

// ── internal rendering helpers using primitives ─────────────────────────────────

fn render_node_path(ctx: &mut ProjectionCtx, trim: f32, fill_alpha: f32, stroke_color: Vec4, fill_color: Vec4) {
    let mut style = Style::new();
    style.stroke = Some(StrokeParams {
        thickness: STROKE_THICK,
        color: stroke_color,
        ..Default::default()
    });
    style.fill = Some(ColorSource::Solid(fill_color));

    let mut path = Square::new(NODE_SIZE, fill_color)
        .with_style(style)
        .to_path();
    
    path.trim_end = trim;
    path.fill_opacity = fill_alpha;

    path.project(ctx);
}

fn render_default_node(ctx: &mut ProjectionCtx, pos: Vec3, state: &StepState, label: &str) {
    ctx.with_offset(pos, |ctx| {
        match state {
            StepState::Pending => {}
            StepState::Active { t } => {
                let t = Ease::InOutSmooth.eval(*t);
                let outline_t = (t * 2.0).clamp(0.0, 1.0);
                let fill_t = ((t - 0.5) * 2.0).clamp(0.0, 1.0);

                render_node_path(ctx, outline_t, fill_t, STROKE_ACTIVE, FILL_ACTIVE);

                if fill_t > 0.001 {
                    ctx.with_offset(Vec3::new(0.0, 0.0, 0.1), |ctx| {
                        Label::new(label, LABEL_HEIGHT)
                            .with_color(Vec4::new(COLOR_LABEL.x, COLOR_LABEL.y, COLOR_LABEL.z, COLOR_LABEL.w * fill_t))
                            .with_char_reveal(fill_t)
                            .project(ctx);
                    });
                }
            }
            StepState::Completed => {
                render_node_path(ctx, 1.0, 1.0, STROKE_COMPLETED, FILL_COMPLETED);
                
                ctx.with_offset(Vec3::new(0.0, 0.0, 0.1), |ctx| {
                    Label::new(label, LABEL_HEIGHT)
                        .with_color(Vec4::new(COLOR_LABEL.x, COLOR_LABEL.y, COLOR_LABEL.z, COLOR_LABEL.w * 0.8))
                        .project(ctx);
                });
            }
        }
    });
}

fn render_background(ctx: &mut ProjectionCtx, pos: Vec3, state: &StepState) {
    ctx.with_offset(pos, |ctx| {
        match state {
            StepState::Pending => {}
            StepState::Active { t } => {
                let t = Ease::InOutSmooth.eval(*t);
                let outline_t = (t * 2.0).clamp(0.0, 1.0);
                let fill_t = ((t - 0.5) * 2.0).clamp(0.0, 1.0);
                render_node_path(ctx, outline_t, fill_t, STROKE_ACTIVE, FILL_ACTIVE);
            }
            StepState::Completed => {
                render_node_path(ctx, 1.0, 1.0, STROKE_COMPLETED, FILL_COMPLETED);
            }
        }
    });
}

// ── Project ───────────────────────────────────────────────────────────────────

impl Project for Stepwise {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let state = TimelineEngine::compute(&self.model, self.progress);

        // ── transitions ───────────────────────────────────────────────────
        for (i, transition) in self.model.transitions.iter().enumerate() {
            let from = self.layout.position_for(transition.from);
            let to   = self.layout.position_for(transition.to);

            match &state.transitions[i] {
                TransitionState::Hidden => {}
                TransitionState::Drawing { t } => {
                    let mut path = Path::new()
                        .move_to(from.truncate())
                        .line_to(to.truncate())
                        .with_thickness(EDGE_THICKNESS)
                        .with_color(FILL_EDGE);
                    
                    path.trim_end = Ease::InOutSmooth.eval(*t);
                    path.project(ctx);
                }
                TransitionState::Completed => {
                    Path::new()
                        .move_to(from.truncate())
                        .line_to(to.truncate())
                        .with_thickness(EDGE_THICKNESS)
                        .with_color(Vec4::new(FILL_EDGE.x, FILL_EDGE.y, FILL_EDGE.z, FILL_EDGE.w * 0.6))
                        .project(ctx);
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
                    ctx.with_offset(pos, |ctx| {
                        content.project(ctx, step_state);
                    });
                }
            }
        }

        // ── signal dot ────────────────────────────────────────────────────
        if self.signal_progress > 0.001 {
            let n = self.model.sequence.len();
            if n >= 2 {
                let total_hops = (n - 1) as f32;
                let raw = self.signal_progress.clamp(0.0, 1.0) * total_hops;
                let hop = (raw.floor() as usize).min(n - 2);
                let t   = raw - hop as f32;

                let from_idx = self.model.sequence[hop];
                let to_idx   = self.model.sequence[hop + 1];
                let eased    = Ease::InOutSmooth.eval(t);
                let p        = self.layout.lerp_position(from_idx, to_idx, eased);

                ctx.with_offset(Vec3::new(p.x, p.y, 0.2), |ctx| {
                    crate::frontend::collection::primitives::circle::Circle::new(SIGNAL_RADIUS, 20, FILL_SIGNAL)
                        .project(ctx);
                });
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
                ctx.with_offset(Vec3::new(x, base_y + i as f32 * step_y, 0.5), |ctx| {
                    Label::new(text, 0.25).with_color(COLOR_DEBUG).project(ctx);
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
            return Bounds::from_center_size(glam::Vec2::ZERO, glam::vec2(0.01, 0.01));
        }
        let pad  = NODE_SIZE * 0.5 + SIGNAL_RADIUS;
        let last = self.layout.position_for(n - 1);
        match self.layout.direction {
            StepwiseDirection::Horizontal => Bounds::new(
                glam::Vec2::new(-pad, -pad),
                glam::Vec2::new(last.x + pad, pad),
            ),
            StepwiseDirection::Vertical => Bounds::new(
                glam::Vec2::new(-pad, last.y - pad),
                glam::Vec2::new(pad, pad),
            ),
        }
    }
}
