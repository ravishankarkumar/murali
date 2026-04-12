// src/frontend/collection/storytelling/stepwise/tattva.rs
//! `Stepwise` — a first-class Murali tattva.

use glam::{Vec2, Vec3, Vec4};

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

    /// Computes signal intensity (0.0 to 1.0) and visited status for a node
    fn node_signal_state(&self, node_idx: usize) -> (f32, bool) {
        if self.signal_progress < 0.001 {
            return (0.0, false);
        }

        let n = self.model.sequence.len();
        if n == 0 {
            return (0.0, false);
        }

        let total_segments = (2 * n - 1) as f32;
        let raw_pos = self.signal_progress * total_segments;
        let segment = (raw_pos.floor() as usize).min(2 * n - 2);
        let segment_t = (raw_pos - segment as f32).clamp(0.0, 1.0);

        let mut intensity = 0.0;
        let mut visited = false;

        for h in 0..n {
            let s_idx = self.model.sequence[h];
            if s_idx == node_idx {
                let pulse_segment = h * 2;
                
                // Determine visitation
                if segment >= pulse_segment {
                    visited = true;
                }

                // Determine pulse intensity (during its specific even segment)
                if segment == pulse_segment {
                    // Triangle pulse peaked at 0.5
                    intensity = if segment_t < 0.5 {
                        segment_t * 2.0
                    } else {
                        2.0 - segment_t * 2.0
                    };
                    intensity = Ease::InOutSmooth.eval(intensity);
                }
            }
        }

        (intensity, visited)
    }

    fn is_transition_signaled(&self, transition_idx: usize) -> bool {
        if self.signal_progress < 0.001 {
            return false;
        }

        let transition = &self.model.transitions[transition_idx];
        let n = self.model.sequence.len();
        if n < 2 {
            return false;
        }

        let total_segments = (2 * n - 1) as f32;
        let raw_pos = self.signal_progress * total_segments;
        let current_segment = raw_pos.floor() as usize;

        for h in 0..(n - 1) {
            let from = self.model.sequence[h];
            let to = self.model.sequence[h + 1];
            if from == transition.from && to == transition.to {
                let transit_segment = h * 2 + 1;
                if current_segment >= transit_segment {
                    return true;
                }
            }
        }
        false
    }
}

// ── internal rendering helpers using primitives ─────────────────────────────────

fn node_size_for(label: &str) -> glam::Vec2 {
    let layout = crate::resource::text::layout::measure_label(label, LABEL_HEIGHT);
    let min_width = NODE_SIZE;
    let width = (layout.width + 0.6).max(min_width);
    glam::vec2(width, NODE_SIZE) // Keep height consistent at basic node size
}

fn rounded_rect_path(size: glam::Vec2, radius: f32, color: glam::Vec4) -> Path {
    let hw = size.x * 0.5;
    let hh = size.y * 0.5;
    let r = radius.min(hw).min(hh);

    let mut path = Path::new();
    path.style.fill = Some(ColorSource::Solid(color));
    path.closed = true;

    // Start at top-left after corner
    path = path.move_to(glam::vec2(-hw + r, -hh))
        .line_to(glam::vec2(hw - r, -hh))
        .quad_to(glam::vec2(hw, -hh), glam::vec2(hw, -hh + r))
        .line_to(glam::vec2(hw, hh - r))
        .quad_to(glam::vec2(hw, hh), glam::vec2(hw - r, hh))
        .line_to(glam::vec2(-hw + r, hh))
        .quad_to(glam::vec2(-hw, hh), glam::vec2(-hw, hh - r))
        .line_to(glam::vec2(-hw, -hh + r))
        .quad_to(glam::vec2(-hw, -hh), glam::vec2(-hw + r, -hh));

    path
}

fn render_node_path(ctx: &mut ProjectionCtx, size: glam::Vec2, trim: f32, fill_alpha: f32, stroke_color: Vec4, fill_color: Vec4) {
    let mut style = Style::new();
    style.stroke = Some(StrokeParams {
        thickness: STROKE_THICK,
        color: stroke_color,
        ..Default::default()
    });
    style.fill = Some(ColorSource::Solid(fill_color));

    let mut path = rounded_rect_path(size, 0.15, fill_color);
    path.style = style;
    
    path.trim_end = trim;
    path.fill_opacity = fill_alpha;

    path.project(ctx);
}

fn render_default_node(ctx: &mut ProjectionCtx, pos: Vec3, state: &StepState, label: &str, scale: f32, is_visited: bool) {
    let size = node_size_for(label) * scale;
    ctx.with_offset(pos, |ctx| {
        let (stroke, fill) = (STROKE_ACTIVE, FILL_ACTIVE);

        match state {
            StepState::Pending => {}
            StepState::Active { t } => {
                let t = Ease::InOutSmooth.eval(*t);
                let outline_t = (t * 2.0).clamp(0.0, 1.0);
                let fill_t = ((t - 0.5) * 2.0).clamp(0.0, 1.0);

                render_node_path(ctx, size, outline_t, fill_t, stroke, fill);

                if fill_t > 0.001 {
                    ctx.with_offset(Vec3::new(0.0, 0.0, 0.1), |ctx| {
                        Label::new(label, LABEL_HEIGHT * scale)
                            .with_color(Vec4::new(COLOR_LABEL.x, COLOR_LABEL.y, COLOR_LABEL.z, COLOR_LABEL.w * fill_t))
                            .with_char_reveal(fill_t)
                            .project(ctx);
                    });
                }
            }
            StepState::Completed => {
                let s_color = if is_visited { STROKE_ACTIVE } else { STROKE_COMPLETED };
                let f_color = if is_visited { FILL_ACTIVE } else { FILL_COMPLETED };

                render_node_path(ctx, size, 1.0, 1.0, s_color, f_color);
                
                ctx.with_offset(Vec3::new(0.0, 0.0, 0.1), |ctx| {
                    Label::new(label, LABEL_HEIGHT * scale)
                        .with_color(Vec4::new(COLOR_LABEL.x, COLOR_LABEL.y, COLOR_LABEL.z, COLOR_LABEL.w * 0.8))
                        .project(ctx);
                });
            }
        }
    });
}

fn render_background(ctx: &mut ProjectionCtx, pos: Vec3, state: &StepState, label: &str, scale: f32, is_visited: bool) {
    let size = node_size_for(label) * scale;
    ctx.with_offset(pos, |ctx| {
        let (stroke, fill) = (STROKE_ACTIVE, FILL_ACTIVE);

        match state {
            StepState::Pending => {}
            StepState::Active { t } => {
                let t = Ease::InOutSmooth.eval(*t);
                let outline_t = (t * 2.0).clamp(0.0, 1.0);
                let fill_t = ((t - 0.5) * 2.0).clamp(0.0, 1.0);
                render_node_path(ctx, size, outline_t, fill_t, stroke, fill);
            }
            StepState::Completed => {
                let s_color = if is_visited { STROKE_ACTIVE } else { STROKE_COMPLETED };
                let f_color = if is_visited { FILL_ACTIVE } else { FILL_COMPLETED };
                render_node_path(ctx, size, 1.0, 1.0, s_color, f_color);
            }
        }
    });
}

// ── Project ───────────────────────────────────────────────────────────────────

impl Project for Stepwise {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let state = TimelineEngine::compute(&self.model, self.progress);

        // 1. Precalculate node sizes and positions based on actual labels
        let mut node_sizes = Vec::with_capacity(self.model.steps.len());
        let mut node_positions = Vec::with_capacity(self.model.steps.len());
        
        let mut cursor = 0.0;
        let gap = self.layout.spacing; // Repurpose spacing as "Gap"

        for (i, step) in self.model.steps.iter().enumerate() {
            let size = node_size_for(&step.label);
            node_sizes.push(size);
            
            if i == 0 {
                cursor = 0.0;
            } else {
                let prev_size = node_sizes[i-1];
                let dist = match self.layout.direction {
                    StepwiseDirection::Horizontal => prev_size.x * 0.5 + gap + size.x * 0.5,
                    StepwiseDirection::Vertical   => prev_size.y * 0.5 + gap + size.y * 0.5,
                };
                cursor += dist;
            }
            
            let pos = match self.layout.direction {
                StepwiseDirection::Horizontal => Vec3::new(cursor, 0.0, 0.0),
                StepwiseDirection::Vertical   => Vec3::new(0.0, -cursor, 0.0),
            };
            node_positions.push(pos);
        }

        // ── transitions ───────────────────────────────────────────────────
        for (i, transition) in self.model.transitions.iter().enumerate() {
            let from_idx = transition.from;
            let to_idx   = transition.to;
            
            let from_raw = node_positions[from_idx];
            let to_raw   = node_positions[to_idx];
            let from_size = node_sizes[from_idx];
            let to_size   = node_sizes[to_idx];
            
            // Clip edges to node boundaries (Rectangular clipping)
            let (from, to) = clip_edge_rect(from_raw, to_raw, from_size, to_size, 0.05);

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
                    // Check if signal has passed this edge
                    let is_signaled = self.is_transition_signaled(i);
                    let color = if is_signaled {
                        STROKE_ACTIVE // Use a bright color for "activated" edges
                    } else {
                        Vec4::new(FILL_EDGE.x, FILL_EDGE.y, FILL_EDGE.z, FILL_EDGE.w * 0.6)
                    };

                    Path::new()
                        .move_to(from.truncate())
                        .line_to(to.truncate())
                        .with_thickness(EDGE_THICKNESS)
                        .with_color(color)
                        .project(ctx);
                }
            }
        }

        // ── nodes ─────────────────────────────────────────────────────────
        for (i, step) in self.model.steps.iter().enumerate() {
            let step_state = &state.steps[i];
            let pos = node_positions[i];

            // Determine signal-based indication for this node
            let (signal_intensity, is_visited) = self.node_signal_state(i);
            let scale_mod = 1.0 + 0.15 * signal_intensity; // 15% pulse
            
            ctx.with_offset(pos, |ctx| {
                match &step.content {
                    None => render_default_node(ctx, Vec3::ZERO, step_state, &step.label, scale_mod, is_visited),
                    Some(content) => {
                        if !content.draws_own_background() {
                            render_background(ctx, Vec3::ZERO, step_state, &step.label, scale_mod, is_visited);
                        }
                        ctx.with_offset(Vec3::ZERO, |ctx| {
                            content.project(ctx, step_state);
                        });
                    }
                }
            });
        }

        // ── signal dot ────────────────────────────────────────────────────
        if self.signal_progress > 0.001 {
            let n = self.model.sequence.len();
            if n >= 1 {
                let total_segments = (2 * n - 1) as f32;
                let raw_pos = self.signal_progress.clamp(0.0, 1.0) * total_segments;
                let segment = (raw_pos.floor() as usize).min(2 * n - 2);
                let segment_t = (raw_pos - segment as f32).clamp(0.0, 1.0);

                // Even segment: Node Pulse (Node i = segment / 2)
                // Odd segment:  Transit (h = (segment - 1) / 2)
                if segment % 2 == 1 && n >= 2 {
                    let hop = (segment - 1) / 2;
                    let from_idx = self.model.sequence[hop];
                    let to_idx   = self.model.sequence[hop + 1];
                    
                    let from_raw = node_positions[from_idx];
                    let to_raw   = node_positions[to_idx];
                    let from_size = node_sizes[from_idx];
                    let to_size   = node_sizes[to_idx];

                    let (from, to) = clip_edge_rect(from_raw, to_raw, from_size, to_size, 0.0);

                    let p = from.lerp(to, Ease::InOutSmooth.eval(segment_t));

                    ctx.with_offset(Vec3::new(p.x, p.y, 0.2), |ctx| {
                        crate::frontend::collection::primitives::circle::Circle::new(SIGNAL_RADIUS, 20, FILL_SIGNAL)
                            .project(ctx);
                    });
                }
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

/// Ray-rectangle intersection.
/// Returns clipped (start, end) points.
fn clip_edge_rect(from: Vec3, to: Vec3, from_size: Vec2, to_size: Vec2, padding: f32) -> (Vec3, Vec3) {
    let dir = to - from;
    let dist = dir.length();
    if dist < 1e-6 {
        return (from, to);
    }
    let d = dir / dist;

    let get_t = |dir: Vec3, size: Vec2| {
        let mag_x = dir.x.abs();
        let mag_y = dir.y.abs();
        
        let tx = if mag_x > 1e-6 { (size.x * 0.5 + padding) / mag_x } else { f32::INFINITY };
        let ty = if mag_y > 1e-6 { (size.y * 0.5 + padding) / mag_y } else { f32::INFINITY };
        
        tx.min(ty)
    };

    let t_start = get_t(d, from_size);
    let t_end   = get_t(-d, to_size);

    (from + d * t_start, to - d * t_end)
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
