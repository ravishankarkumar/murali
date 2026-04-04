pub mod builder;
pub mod camera_animation;
pub mod camera_animation_builder;

use std::collections::HashMap;

use crate::engine::scene::Scene;
use crate::frontend::collection::ai::agentic_flow_chart::AgenticFlowChart;
use crate::frontend::collection::ai::signal_flow::SignalFlow;
use crate::frontend::collection::math::equation::{
    EquationLayout, EquationPart, EquationPartLayout,
};
use crate::frontend::collection::math::matrix::{Matrix, MatrixCellLayout};
use crate::frontend::collection::primitives::circle::Circle;
use crate::frontend::collection::primitives::ellipse::Ellipse;
use crate::frontend::collection::primitives::line::Line;
use crate::frontend::collection::primitives::path::Path;
use crate::frontend::collection::primitives::polygon::Polygon;
use crate::frontend::collection::primitives::rectangle::Rectangle;
use crate::frontend::collection::primitives::square::Square;
use crate::frontend::collection::primitives::to_path::ToPath;
use crate::frontend::collection::utility::screenshot_marker::ScreenshotMarker;
use crate::frontend::layout::{Anchor, Bounded, Bounds};
use crate::frontend::props::DrawableProps;
use crate::frontend::tattva_trait::TattvaTrait;
use crate::frontend::{DirtyFlags, Tattva, TattvaId};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::resource::text::layout::measure_label;
use glam::{Quat, Vec2, Vec3, Vec4};

/// Common easing curves for deterministic interpolation.
#[derive(Copy, Clone, Debug, Default)]
pub enum Ease {
    #[default]
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
}

impl Ease {
    pub fn eval(&self, t: f32) -> f32 {
        match self {
            Ease::Linear => t,
            Ease::InQuad => t * t,
            Ease::OutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Ease::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Ease::InCubic => t * t * t,
            Ease::OutCubic => 1.0 - (1.0 - t).powi(3),
            Ease::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
        }
    }
}

/// The core trait for all Frontend logic changes over time.
/// Every implementation must be deterministic.
pub trait Animation: Send + Sync {
    fn on_start(&mut self, scene: &mut Scene);
    fn apply_at(&mut self, scene: &mut Scene, t: f32);
    fn on_finish(&mut self, _scene: &mut Scene) {}
    fn reset(&mut self, _scene: &mut Scene) {}
}

fn with_props_mut<F>(scene: &mut Scene, target_id: TattvaId, dirty: DirtyFlags, f: F)
where
    F: FnOnce(&mut DrawableProps),
{
    if let Some(tattva) = scene.get_tattva_any_mut(target_id) {
        let mut props = DrawableProps::write(tattva.props());
        f(&mut props);
        drop(props);
        tattva.mark_dirty(dirty);
    }
}

pub struct MoveTo {
    pub target_id: TattvaId,
    pub to: Vec3,
    pub ease: Ease,
    from: Option<Vec3>,
}

impl MoveTo {
    pub fn new(target_id: TattvaId, to: Vec3, ease: Ease) -> Self {
        Self {
            target_id,
            to,
            ease,
            from: None,
        }
    }

    pub fn with_from(mut self, from: Vec3) -> Self {
        self.from = Some(from);
        self
    }
}

impl Animation for MoveTo {
    fn on_start(&mut self, scene: &mut Scene) {
        if self.from.is_none() {
            if let Some(tattva) = scene.get_tattva_any(self.target_id) {
                let props = DrawableProps::read(tattva.props());
                self.from = Some(props.position);
            }
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let new_pos = self
            .from
            .unwrap_or(self.to)
            .lerp(self.to, self.ease.eval(t));
        with_props_mut(scene, self.target_id, DirtyFlags::TRANSFORM, |props| {
            props.position = new_pos;
        });
    }
}

pub struct RotateTo {
    pub target_id: TattvaId,
    pub to: Quat,
    pub ease: Ease,
    from: Option<Quat>,
}

impl RotateTo {
    pub fn new(target_id: TattvaId, to: Quat, ease: Ease) -> Self {
        Self {
            target_id,
            to,
            ease,
            from: None,
        }
    }

    pub fn with_from(mut self, from: Quat) -> Self {
        self.from = Some(from);
        self
    }
}

impl Animation for RotateTo {
    fn on_start(&mut self, scene: &mut Scene) {
        if self.from.is_none() {
            if let Some(tattva) = scene.get_tattva_any(self.target_id) {
                let props = DrawableProps::read(tattva.props());
                self.from = Some(props.rotation);
            }
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let from = self.from.unwrap_or(self.to);
        let rotation = from.slerp(self.to, self.ease.eval(t));
        with_props_mut(scene, self.target_id, DirtyFlags::TRANSFORM, |props| {
            props.rotation = rotation;
        });
    }
}

pub struct ScaleTo {
    pub target_id: TattvaId,
    pub to: Vec3,
    pub ease: Ease,
    from: Option<Vec3>,
}

impl ScaleTo {
    pub fn new(target_id: TattvaId, to: Vec3, ease: Ease) -> Self {
        Self {
            target_id,
            to,
            ease,
            from: None,
        }
    }

    pub fn with_from(mut self, from: Vec3) -> Self {
        self.from = Some(from);
        self
    }
}

impl Animation for ScaleTo {
    fn on_start(&mut self, scene: &mut Scene) {
        if self.from.is_none() {
            if let Some(tattva) = scene.get_tattva_any(self.target_id) {
                let props = DrawableProps::read(tattva.props());
                self.from = Some(props.scale);
            }
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let scale = self
            .from
            .unwrap_or(self.to)
            .lerp(self.to, self.ease.eval(t));
        with_props_mut(scene, self.target_id, DirtyFlags::TRANSFORM, |props| {
            props.scale = scale;
        });
    }
}

pub struct FadeTo {
    pub target_id: TattvaId,
    pub to: f32,
    pub ease: Ease,
    from: Option<f32>,
}

impl FadeTo {
    pub fn new(target_id: TattvaId, to: f32, ease: Ease) -> Self {
        Self {
            target_id,
            to: to.clamp(0.0, 1.0),
            ease,
            from: None,
        }
    }

    pub fn with_from(mut self, from: f32) -> Self {
        self.from = Some(from);
        self
    }
}

impl Animation for FadeTo {
    fn on_start(&mut self, scene: &mut Scene) {
        if self.from.is_none() {
            if let Some(tattva) = scene.get_tattva_any(self.target_id) {
                let props = DrawableProps::read(tattva.props());
                self.from = Some(props.opacity);
            }
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let opacity = self.from.unwrap_or(self.to)
            + (self.to - self.from.unwrap_or(self.to)) * self.ease.eval(t);
        with_props_mut(
            scene,
            self.target_id,
            DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
            |props| {
                props.visible = opacity > 0.001;
                props.opacity = opacity.clamp(0.0, 1.0);
            },
        );
    }
}

pub struct Create {
    pub target_id: TattvaId,
    pub ease: Ease,
    target_opacity: Option<f32>,
}

impl Create {
    pub fn new(target_id: TattvaId, ease: Ease) -> Self {
        Self {
            target_id,
            ease,
            target_opacity: None,
        }
    }
}

impl Animation for Create {
    fn on_start(&mut self, scene: &mut Scene) {
        if let Some(tattva) = scene.get_tattva_any_mut(self.target_id) {
            let mut props = DrawableProps::write(tattva.props());
            self.target_opacity = Some(props.opacity.max(0.001));
            props.visible = true;
            props.opacity = 0.0;
            drop(props);
            tattva.mark_dirty(DirtyFlags::STYLE | DirtyFlags::VISIBILITY);
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let target = self.target_opacity.unwrap_or(1.0);
        let opacity = target * self.ease.eval(t);
        with_props_mut(
            scene,
            self.target_id,
            DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
            |props| {
                props.visible = true;
                props.opacity = opacity.clamp(0.0, target);
            },
        );
    }
}

pub struct FollowAnchor {
    pub follower_id: TattvaId,
    pub target_id: TattvaId,
    pub target_anchor: Anchor,
    pub follower_anchor: Anchor,
    pub offset: Vec3,
}

impl FollowAnchor {
    pub fn new(
        follower_id: TattvaId,
        target_id: TattvaId,
        target_anchor: Anchor,
        follower_anchor: Anchor,
        offset: Vec3,
    ) -> Self {
        Self {
            follower_id,
            target_id,
            target_anchor,
            follower_anchor,
            offset,
        }
    }

    fn sync_position(&self, scene: &mut Scene) {
        let Some(target_point) = scene.anchor_position(self.target_id, self.target_anchor) else {
            return;
        };
        let Some(local_bounds) = scene.local_bounds(self.follower_id) else {
            return;
        };
        let pos = target_point + self.offset.truncate() - local_bounds.anchor(self.follower_anchor);
        scene.set_position(self.follower_id, pos);
    }
}

impl Animation for FollowAnchor {
    fn on_start(&mut self, scene: &mut Scene) {
        self.sync_position(scene);
    }

    fn apply_at(&mut self, scene: &mut Scene, _t: f32) {
        self.sync_position(scene);
    }
}

pub struct PropagateSignal {
    pub target_id: TattvaId,
    pub ease: Ease,
    from: Option<f32>,
    to: f32,
}

impl PropagateSignal {
    pub fn new(target_id: TattvaId, to: f32, ease: Ease) -> Self {
        Self {
            target_id,
            ease,
            from: None,
            to: to.clamp(0.0, 1.0),
        }
    }
}

impl Animation for PropagateSignal {
    fn on_start(&mut self, scene: &mut Scene) {
        if let Some(flow) = scene.get_tattva_typed::<SignalFlow>(self.target_id) {
            self.from = Some(flow.state.progress);
        } else if let Some(flow) = scene.get_tattva_typed::<AgenticFlowChart>(self.target_id) {
            self.from = Some(flow.state.progress);
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let from = self.from.unwrap_or(0.0);
        let progress = from + (self.to - from) * self.ease.eval(t);
        if let Some(flow) = scene.get_tattva_typed_mut::<SignalFlow>(self.target_id) {
            flow.state.progress = progress.clamp(0.0, 1.0);
            flow.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        } else if let Some(flow) = scene.get_tattva_typed_mut::<AgenticFlowChart>(self.target_id) {
            flow.state.progress = progress.clamp(0.0, 1.0);
            flow.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        }
    }

    fn on_finish(&mut self, scene: &mut Scene) {
        if let Some(flow) = scene.get_tattva_typed_mut::<SignalFlow>(self.target_id) {
            flow.state.progress = self.to;
            flow.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        } else if let Some(flow) = scene.get_tattva_typed_mut::<AgenticFlowChart>(self.target_id) {
            flow.state.progress = self.to;
            flow.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        }
    }

    fn reset(&mut self, scene: &mut Scene) {
        if let Some(flow) = scene.get_tattva_typed_mut::<SignalFlow>(self.target_id) {
            flow.state.progress = self.from.unwrap_or(0.0);
            flow.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        } else if let Some(flow) = scene.get_tattva_typed_mut::<AgenticFlowChart>(self.target_id) {
            flow.state.progress = self.from.unwrap_or(0.0);
            flow.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        }
    }
}

pub struct TriggerCapture {
    pub target_id: TattvaId,
}

impl TriggerCapture {
    pub fn new(target_id: TattvaId) -> Self {
        Self { target_id }
    }
}

impl Animation for TriggerCapture {
    fn on_start(&mut self, scene: &mut Scene) {
        if let Some(marker) = scene.get_tattva_typed_mut::<ScreenshotMarker>(self.target_id) {
            marker.state.arm();
            marker.mark_dirty(DirtyFlags::STYLE | DirtyFlags::VISIBILITY);
        }
    }

    fn apply_at(&mut self, _scene: &mut Scene, _t: f32) {}

    fn reset(&mut self, scene: &mut Scene) {
        if let Some(marker) = scene.get_tattva_typed_mut::<ScreenshotMarker>(self.target_id) {
            marker.state.reset_capture();
            marker.mark_dirty(DirtyFlags::STYLE | DirtyFlags::VISIBILITY);
        }
    }
}

fn safe_div_vec2(numerator: Vec2, denominator: Vec2, fallback: Vec2) -> Vec2 {
    Vec2::new(
        if denominator.x.abs() > 1e-5 {
            numerator.x / denominator.x
        } else {
            fallback.x
        },
        if denominator.y.abs() > 1e-5 {
            numerator.y / denominator.y
        } else {
            fallback.y
        },
    )
}

#[derive(Clone)]
struct MatchSnapshot {
    source: DrawableProps,
    target: DrawableProps,
    matched_position: Vec3,
    matched_scale: Vec3,
}

fn build_match_snapshot(
    scene: &Scene,
    source_id: TattvaId,
    target_id: TattvaId,
) -> Option<MatchSnapshot> {
    let source = scene.get_tattva_any(source_id)?;
    let target = scene.get_tattva_any(target_id)?;
    let source_props = DrawableProps::read(source.props()).clone();
    let target_props = DrawableProps::read(target.props()).clone();
    let source_bounds = scene.world_bounds(source_id)?;
    let target_local = target.local_bounds();
    let target_local_size = target_local.size();
    let source_size = source_bounds.size();

    let matched_xy = safe_div_vec2(
        source_size,
        Vec2::new(target_local_size.x.max(1e-5), target_local_size.y.max(1e-5)),
        target_props.scale.truncate(),
    );

    Some(MatchSnapshot {
        source: source_props.clone(),
        target: target_props.clone(),
        matched_position: Vec3::new(
            source_bounds.center().x,
            source_bounds.center().y,
            source_props.position.z,
        ),
        matched_scale: Vec3::new(matched_xy.x, matched_xy.y, target_props.scale.z),
    })
}

pub struct MatchTransform {
    pub source_id: TattvaId,
    pub target_id: TattvaId,
    pub ease: Ease,
    snapshot: Option<MatchSnapshot>,
}

impl MatchTransform {
    pub fn new(source_id: TattvaId, target_id: TattvaId, ease: Ease) -> Self {
        Self {
            source_id,
            target_id,
            ease,
            snapshot: None,
        }
    }
}

impl Animation for MatchTransform {
    fn on_start(&mut self, scene: &mut Scene) {
        self.snapshot = build_match_snapshot(scene, self.source_id, self.target_id);
        let Some(snapshot) = &self.snapshot else {
            return;
        };

        with_props_mut(scene, self.target_id, DirtyFlags::TRANSFORM, |props| {
            props.visible = true;
            props.position = snapshot.matched_position;
            props.scale = snapshot.matched_scale;
            props.rotation = snapshot.source.rotation;
        });
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let eased = self.ease.eval(t);
        with_props_mut(scene, self.target_id, DirtyFlags::TRANSFORM, |props| {
            props.visible = true;
            props.position = snapshot
                .matched_position
                .lerp(snapshot.target.position, eased);
            props.scale = snapshot.matched_scale.lerp(snapshot.target.scale, eased);
            props.rotation = snapshot
                .source
                .rotation
                .slerp(snapshot.target.rotation, eased);
        });
    }

    fn on_finish(&mut self, scene: &mut Scene) {
        if let Some(snapshot) = &self.snapshot {
            with_props_mut(scene, self.target_id, DirtyFlags::TRANSFORM, |props| {
                *props = snapshot.target.clone();
            });
        }
    }

    fn reset(&mut self, scene: &mut Scene) {
        if let Some(snapshot) = &self.snapshot {
            with_props_mut(scene, self.target_id, DirtyFlags::TRANSFORM, |props| {
                *props = snapshot.target.clone();
            });
        }
    }
}

pub struct MorphObjects {
    pub source_id: TattvaId,
    pub target_id: TattvaId,
    pub ease: Ease,
    snapshot: Option<MatchSnapshot>,
}

impl MorphObjects {
    pub fn new(source_id: TattvaId, target_id: TattvaId, ease: Ease) -> Self {
        Self {
            source_id,
            target_id,
            ease,
            snapshot: None,
        }
    }
}

impl Animation for MorphObjects {
    fn on_start(&mut self, scene: &mut Scene) {
        self.snapshot = build_match_snapshot(scene, self.source_id, self.target_id);
        let Some(snapshot) = &self.snapshot else {
            return;
        };

        with_props_mut(
            scene,
            self.target_id,
            DirtyFlags::TRANSFORM | DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
            |props| {
                props.visible = true;
                props.position = snapshot.matched_position;
                props.scale = snapshot.matched_scale;
                props.rotation = snapshot.source.rotation;
                props.opacity = 0.0;
            },
        );
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let eased = self.ease.eval(t);
        with_props_mut(
            scene,
            self.target_id,
            DirtyFlags::TRANSFORM | DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
            |props| {
                props.visible = true;
                props.position = snapshot
                    .matched_position
                    .lerp(snapshot.target.position, eased);
                props.scale = snapshot.matched_scale.lerp(snapshot.target.scale, eased);
                props.rotation = snapshot
                    .source
                    .rotation
                    .slerp(snapshot.target.rotation, eased);
                props.opacity = snapshot.target.opacity * eased;
            },
        );
        with_props_mut(
            scene,
            self.source_id,
            DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
            |props| {
                props.opacity = snapshot.source.opacity * (1.0 - eased);
                props.visible = props.opacity > 0.001;
            },
        );
    }

    fn on_finish(&mut self, scene: &mut Scene) {
        if let Some(snapshot) = &self.snapshot {
            with_props_mut(
                scene,
                self.target_id,
                DirtyFlags::TRANSFORM | DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    *props = snapshot.target.clone();
                },
            );
            with_props_mut(
                scene,
                self.source_id,
                DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    props.opacity = 0.0;
                    props.visible = false;
                },
            );
        }
    }

    fn reset(&mut self, scene: &mut Scene) {
        if let Some(snapshot) = &self.snapshot {
            with_props_mut(
                scene,
                self.target_id,
                DirtyFlags::TRANSFORM | DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    *props = snapshot.target.clone();
                },
            );
            with_props_mut(
                scene,
                self.source_id,
                DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    *props = snapshot.source.clone();
                },
            );
        }
    }
}

#[derive(Clone)]
struct EquationContinuitySnapshot {
    source_props: DrawableProps,
    target_props: DrawableProps,
    source_parts: Vec<EquationPartLayout>,
    target_parts: Vec<EquationPartLayout>,
    original_target_parts: Vec<EquationPart>,
}

pub struct EquationContinuity {
    pub source_id: TattvaId,
    pub target_id: TattvaId,
    pub ease: Ease,
    snapshot: Option<EquationContinuitySnapshot>,
}

impl EquationContinuity {
    pub fn new(source_id: TattvaId, target_id: TattvaId, ease: Ease) -> Self {
        Self {
            source_id,
            target_id,
            ease,
            snapshot: None,
        }
    }
}

impl Animation for EquationContinuity {
    fn on_start(&mut self, scene: &mut Scene) {
        let source_tattva = match scene.get_tattva_typed::<EquationLayout>(self.source_id) {
            Some(t) => t,
            None => return,
        };
        let target_tattva = match scene.get_tattva_typed::<EquationLayout>(self.target_id) {
            Some(t) => t,
            None => return,
        };

        self.snapshot = Some(EquationContinuitySnapshot {
            source_props: DrawableProps::read(&source_tattva.props).clone(),
            target_props: DrawableProps::read(&target_tattva.props).clone(),
            source_parts: source_tattva.state.layout_snapshot(),
            target_parts: target_tattva.state.layout_snapshot(),
            original_target_parts: target_tattva.state.parts.clone(),
        });
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let eased = self.ease.eval(t);
        let source_by_key: HashMap<&str, &EquationPartLayout> = snapshot
            .source_parts
            .iter()
            .map(|part| (part.key.as_str(), part))
            .collect();

        let target_props = snapshot.target_props.clone();
        let source_props = snapshot.source_props.clone();

        if let Some(target) = scene.get_tattva_typed_mut::<EquationLayout>(self.target_id) {
            for (idx, part) in target.state.parts.iter_mut().enumerate() {
                let base = &snapshot.original_target_parts[idx];
                let target_layout = &snapshot.target_parts[idx];
                *part = base.clone();

                if let Some(source_layout) = source_by_key.get(target_layout.key.as_str()) {
                    let source_world = source_props.position.truncate()
                        + source_layout.center.truncate() * source_props.scale.truncate();
                    let target_local_start = safe_div_vec2(
                        source_world - target_props.position.truncate(),
                        target_props.scale.truncate(),
                        target_layout.center.truncate(),
                    );
                    let blended_center = Vec2::new(target_local_start.x, target_local_start.y)
                        .lerp(target_layout.center.truncate(), eased);
                    part.offset += Vec3::new(
                        blended_center.x - target_layout.center.x,
                        blended_center.y - target_layout.center.y,
                        0.0,
                    );
                    part.scale =
                        source_layout.scale + (target_layout.scale - source_layout.scale) * eased;
                    part.opacity = source_layout.opacity
                        + (target_layout.opacity - source_layout.opacity) * eased;
                } else {
                    part.offset += Vec3::new(0.0, (1.0 - eased) * target_layout.height * 0.35, 0.0);
                    part.opacity = target_layout.opacity * eased;
                    part.scale = 0.9 + 0.1 * eased;
                }
            }
            target.mark_dirty(DirtyFlags::TEXT_LAYOUT | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        }

        with_props_mut(
            scene,
            self.source_id,
            DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
            |props| {
                props.opacity = snapshot.source_props.opacity * (1.0 - eased);
                props.visible = props.opacity > 0.001;
            },
        );
        with_props_mut(
            scene,
            self.target_id,
            DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
            |props| {
                props.visible = true;
                props.opacity = snapshot.target_props.opacity;
            },
        );
    }

    fn on_finish(&mut self, scene: &mut Scene) {
        if let Some(snapshot) = &self.snapshot {
            if let Some(target) = scene.get_tattva_typed_mut::<EquationLayout>(self.target_id) {
                target.state.parts = snapshot.original_target_parts.clone();
                target.mark_dirty(DirtyFlags::TEXT_LAYOUT | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
            }
            with_props_mut(
                scene,
                self.source_id,
                DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    props.opacity = 0.0;
                    props.visible = false;
                },
            );
            with_props_mut(
                scene,
                self.target_id,
                DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    *props = snapshot.target_props.clone();
                },
            );
        }
    }

    fn reset(&mut self, scene: &mut Scene) {
        if let Some(snapshot) = &self.snapshot {
            if let Some(target) = scene.get_tattva_typed_mut::<EquationLayout>(self.target_id) {
                target.state.parts = snapshot.original_target_parts.clone();
                target.mark_dirty(DirtyFlags::TEXT_LAYOUT | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
            }
            with_props_mut(
                scene,
                self.source_id,
                DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    *props = snapshot.source_props.clone();
                },
            );
            with_props_mut(
                scene,
                self.target_id,
                DirtyFlags::STYLE | DirtyFlags::VISIBILITY,
                |props| {
                    *props = snapshot.target_props.clone();
                },
            );
        }
    }
}

#[derive(Clone)]
enum MatrixSelection {
    Cells(Vec<(usize, usize)>),
    Row(usize),
    Column(usize),
}

#[derive(Clone)]
struct MatrixStepSnapshot {
    original: Matrix,
}

pub struct MatrixStep {
    pub target_id: TattvaId,
    pub ease: Ease,
    pub highlight: Vec4,
    pub dim_opacity: f32,
    selection: MatrixSelection,
    snapshot: Option<MatrixStepSnapshot>,
}

impl MatrixStep {
    pub fn cells(
        target_id: TattvaId,
        cells: Vec<(usize, usize)>,
        highlight: Vec4,
        dim_opacity: f32,
        ease: Ease,
    ) -> Self {
        Self {
            target_id,
            ease,
            highlight,
            dim_opacity,
            selection: MatrixSelection::Cells(cells),
            snapshot: None,
        }
    }

    pub fn row(
        target_id: TattvaId,
        row: usize,
        highlight: Vec4,
        dim_opacity: f32,
        ease: Ease,
    ) -> Self {
        Self {
            target_id,
            ease,
            highlight,
            dim_opacity,
            selection: MatrixSelection::Row(row),
            snapshot: None,
        }
    }

    pub fn column(
        target_id: TattvaId,
        col: usize,
        highlight: Vec4,
        dim_opacity: f32,
        ease: Ease,
    ) -> Self {
        Self {
            target_id,
            ease,
            highlight,
            dim_opacity,
            selection: MatrixSelection::Column(col),
            snapshot: None,
        }
    }

    fn is_selected(&self, layout: &MatrixCellLayout) -> bool {
        match &self.selection {
            MatrixSelection::Cells(cells) => cells.contains(&(layout.row, layout.col)),
            MatrixSelection::Row(row) => layout.row == *row,
            MatrixSelection::Column(col) => layout.col == *col,
        }
    }
}

impl Animation for MatrixStep {
    fn on_start(&mut self, scene: &mut Scene) {
        let Some(matrix) = scene.get_tattva_typed::<Matrix>(self.target_id) else {
            return;
        };
        self.snapshot = Some(MatrixStepSnapshot {
            original: matrix.state.clone(),
        });
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let eased = self.ease.eval(t);
        let layout = snapshot.original.layout_snapshot();

        if let Some(matrix) = scene.get_tattva_typed_mut::<Matrix>(self.target_id) {
            matrix.state = snapshot.original.clone();
            for cell_layout in &layout {
                let Some(cell) = matrix.state.cell_mut(cell_layout.row, cell_layout.col) else {
                    continue;
                };
                if self.is_selected(cell_layout) {
                    let mut highlight = self.highlight;
                    highlight.w *= eased;
                    cell.highlight = Some(highlight);
                    cell.scale = cell.scale + 0.12 * eased;
                    cell.opacity = cell.opacity + (1.0 - cell.opacity) * (eased * 0.35);
                } else {
                    cell.highlight = None;
                    cell.opacity =
                        cell.opacity + (self.dim_opacity.clamp(0.1, 1.0) - cell.opacity) * eased;
                }
            }
            matrix.mark_dirty(
                DirtyFlags::TEXT_LAYOUT
                    | DirtyFlags::BOUNDS
                    | DirtyFlags::STYLE
                    | DirtyFlags::GEOMETRY,
            );
        }
    }

    fn on_finish(&mut self, scene: &mut Scene) {
        self.apply_at(scene, 1.0);
    }

    fn reset(&mut self, scene: &mut Scene) {
        if let Some(snapshot) = &self.snapshot {
            if let Some(matrix) = scene.get_tattva_typed_mut::<Matrix>(self.target_id) {
                matrix.state = snapshot.original.clone();
                matrix.mark_dirty(
                    DirtyFlags::TEXT_LAYOUT
                        | DirtyFlags::BOUNDS
                        | DirtyFlags::STYLE
                        | DirtyFlags::GEOMETRY,
                );
            }
        }
    }
}

pub struct MorphGeometry {
    pub source_id: TattvaId,
    pub target_id: TattvaId,
    pub ease: Ease,
    source_path: Option<Path>,
    target_path: Option<Path>,
    original_target_tattva: Option<Box<dyn TattvaTrait>>,
}

impl MorphGeometry {
    pub fn new(source_id: TattvaId, target_id: TattvaId, ease: Ease) -> Self {
        Self {
            source_id,
            target_id,
            ease,
            source_path: None,
            target_path: None,
            original_target_tattva: None,
        }
    }

    fn try_get_path(scene: &Scene, id: TattvaId) -> Option<Path> {
        let tattva = scene.get_tattva_any(id)?;
        let any = tattva.as_any();

        // Check if it's already a Path
        if let Some(p) = any.downcast_ref::<Tattva<Path>>() {
            return Some(p.state.clone());
        }

        // Try common primitives
        if let Some(p) = any.downcast_ref::<Tattva<Rectangle>>() {
            return Some(p.state.to_path());
        }
        if let Some(p) = any.downcast_ref::<Tattva<Circle>>() {
            return Some(p.state.to_path());
        }
        if let Some(p) = any.downcast_ref::<Tattva<Square>>() {
            return Some(p.state.to_path());
        }
        if let Some(p) = any.downcast_ref::<Tattva<Ellipse>>() {
            return Some(p.state.to_path());
        }
        if let Some(p) = any.downcast_ref::<Tattva<Polygon>>() {
            return Some(p.state.to_path());
        }
        if let Some(p) = any.downcast_ref::<Tattva<Line>>() {
            return Some(p.state.to_path());
        }

        None
    }
}

impl Animation for MorphGeometry {
    fn on_start(&mut self, scene: &mut Scene) {
        let mut source_path = match Self::try_get_path(scene, self.source_id) {
            Some(p) => p,
            None => {
                println!("Error: Could not get path for source {}", self.source_id);
                return;
            }
        };
        let mut target_path = match Self::try_get_path(scene, self.target_id) {
            Some(p) => p,
            None => {
                println!("Error: Could not get path for target {}", self.target_id);
                return;
            }
        };

        // Align segments counts
        let max_segments = source_path.segments.len().max(target_path.segments.len());
        source_path.resample(max_segments);
        target_path.resample(max_segments);

        // Align starting points to minimize travel distance
        source_path = source_path.align_to(&target_path);

        self.source_path = Some(source_path.clone());
        self.target_path = Some(target_path);

        // Displace the current target with a Path Tattva
        if let Some(tattva_box) = scene.tattvas.remove(&self.target_id) {
            let shared_props = tattva_box.props().clone();

            // Ensure target is visible and opaque via the SHARED props
            {
                let mut props = shared_props.write();
                props.visible = true;
                props.opacity = 1.0;
            }

            // Create a new Tattva<Path> but REUSE the original SharedProps Arc
            let intermediate = Tattva {
                id: self.target_id,
                state: source_path,
                props: shared_props,
                dirty: DirtyFlags::ALL,
            };

            // Hide the source shape
            if let Some(source) = scene.get_tattva_any_mut(self.source_id) {
                let mut s_props = source.props().write();
                s_props.visible = false;
                s_props.opacity = 0.0;
            }

            // We have to store the original and replace it in the scene
            self.original_target_tattva = Some(tattva_box);
            scene.replace_tattva(self.target_id, Box::new(intermediate));
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t: f32) {
        let (Some(s_path), Some(t_path)) = (&self.source_path, &self.target_path) else {
            return;
        };

        let current_path = s_path.lerp(t_path, self.ease.eval(t));

        if let Some(tattva) = scene.get_tattva_typed_mut::<Path>(self.target_id) {
            tattva.state = current_path;
            tattva.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
        }
    }

    fn on_finish(&mut self, scene: &mut Scene) {
        // Swap back the original target Tattva
        if let Some(original) = self.original_target_tattva.take() {
            scene.replace_tattva(self.target_id, original);

            // Marks geometry as dirty since it's back to original type
            if let Some(t) = scene.get_tattva_any_mut(self.target_id) {
                {
                    let mut props = t.props().write();
                    props.visible = true;
                    props.opacity = 1.0;
                }
                t.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::BOUNDS | DirtyFlags::STYLE);
            }
        }
    }
}
