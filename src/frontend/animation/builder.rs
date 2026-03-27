use crate::engine::timeline::Timeline;
use crate::frontend::animation::{
    Animation, Create, Ease, EquationContinuity, FadeTo, FollowAnchor, MatchTransform,
    MatrixStep, MorphObjects, MoveTo, PropagateSignal, RotateTo, ScaleTo,
};
use crate::frontend::layout::Anchor;
use crate::frontend::TattvaId;
use glam::{Quat, Vec3, Vec4};

#[derive(Debug, Clone)]
pub enum AnimKind {
    MoveTo { to: Vec3 },
    RotateTo { to: Quat },
    ScaleTo { to: Vec3 },
    FadeTo { opacity: f32 },
    Create,
    MatchTransform {
        source_id: TattvaId,
    },
    MorphFrom {
        source_id: TattvaId,
    },
    EquationContinuityFrom {
        source_id: TattvaId,
    },
    MatrixStepCells {
        cells: Vec<(usize, usize)>,
        highlight: Vec4,
        dim_opacity: f32,
    },
    MatrixStepRow {
        row: usize,
        highlight: Vec4,
        dim_opacity: f32,
    },
    MatrixStepColumn {
        col: usize,
        highlight: Vec4,
        dim_opacity: f32,
    },
    FollowAnchor {
        target_id: TattvaId,
        target_anchor: Anchor,
        follower_anchor: Anchor,
        offset: Vec3,
    },
    Propagate {
        to: f32,
    },
}

#[derive(Debug, Clone)]
pub struct AnimationSpec {
    pub target_id: TattvaId,
    pub start_time: f32,
    pub duration: f32,
    pub ease: Ease,
    pub kind: Option<AnimKind>,
}

impl AnimationSpec {
    pub fn new(target_id: TattvaId) -> Self {
        Self {
            target_id,
            start_time: 0.0,
            duration: 1.0,
            ease: Ease::Linear,
            kind: None,
        }
    }
}

pub struct AnimationBuilder<'a> {
    timeline: &'a mut Timeline,
    spec: AnimationSpec,
}

impl<'a> AnimationBuilder<'a> {
    pub fn new(timeline: &'a mut Timeline, target_id: TattvaId) -> Self {
        Self {
            timeline,
            spec: AnimationSpec::new(target_id),
        }
    }

    pub fn at(mut self, t: f32) -> Self {
        self.spec.start_time = t;
        self
    }

    pub fn for_duration(mut self, d: f32) -> Self {
        self.spec.duration = d.max(0.0);
        self
    }

    pub fn ease(mut self, e: Ease) -> Self {
        self.spec.ease = e;
        self
    }

    pub fn move_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(AnimKind::MoveTo { to });
        self
    }

    pub fn rotate_to(mut self, to: Quat) -> Self {
        self.spec.kind = Some(AnimKind::RotateTo { to });
        self
    }

    pub fn scale_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(AnimKind::ScaleTo { to });
        self
    }

    pub fn fade_to(mut self, opacity: f32) -> Self {
        self.spec.kind = Some(AnimKind::FadeTo { opacity });
        self
    }

    pub fn create(mut self) -> Self {
        self.spec.kind = Some(AnimKind::Create);
        self
    }

    pub fn match_transform(mut self, source_id: TattvaId) -> Self {
        self.spec.kind = Some(AnimKind::MatchTransform { source_id });
        self
    }

    pub fn morph_from(mut self, source_id: TattvaId) -> Self {
        self.spec.kind = Some(AnimKind::MorphFrom { source_id });
        self
    }

    pub fn equation_continuity_from(mut self, source_id: TattvaId) -> Self {
        self.spec.kind = Some(AnimKind::EquationContinuityFrom { source_id });
        self
    }

    pub fn matrix_step_cells(
        mut self,
        cells: Vec<(usize, usize)>,
        highlight: Vec4,
        dim_opacity: f32,
    ) -> Self {
        self.spec.kind = Some(AnimKind::MatrixStepCells {
            cells,
            highlight,
            dim_opacity,
        });
        self
    }

    pub fn matrix_step_row(mut self, row: usize, highlight: Vec4, dim_opacity: f32) -> Self {
        self.spec.kind = Some(AnimKind::MatrixStepRow {
            row,
            highlight,
            dim_opacity,
        });
        self
    }

    pub fn matrix_step_column(
        mut self,
        col: usize,
        highlight: Vec4,
        dim_opacity: f32,
    ) -> Self {
        self.spec.kind = Some(AnimKind::MatrixStepColumn {
            col,
            highlight,
            dim_opacity,
        });
        self
    }

    pub fn follow_anchor(
        mut self,
        target_id: TattvaId,
        target_anchor: Anchor,
        follower_anchor: Anchor,
        offset: Vec3,
    ) -> Self {
        self.spec.kind = Some(AnimKind::FollowAnchor {
            target_id,
            target_anchor,
            follower_anchor,
            offset,
        });
        self
    }

    pub fn propagate(mut self) -> Self {
        self.spec.kind = Some(AnimKind::Propagate { to: 1.0 });
        self
    }

    pub fn propagate_to(mut self, to: f32) -> Self {
        self.spec.kind = Some(AnimKind::Propagate {
            to: to.clamp(0.0, 1.0),
        });
        self
    }

    pub fn spawn(self) {
        let spec = self.spec;

        let anim: Box<dyn Animation> = match spec.kind {
            Some(AnimKind::MoveTo { to }) => Box::new(MoveTo::new(spec.target_id, to, spec.ease)),
            Some(AnimKind::RotateTo { to }) => {
                Box::new(RotateTo::new(spec.target_id, to, spec.ease))
            }
            Some(AnimKind::ScaleTo { to }) => Box::new(ScaleTo::new(spec.target_id, to, spec.ease)),
            Some(AnimKind::FadeTo { opacity }) => {
                Box::new(FadeTo::new(spec.target_id, opacity, spec.ease))
            }
            Some(AnimKind::Create) => Box::new(Create::new(spec.target_id, spec.ease)),
            Some(AnimKind::MatchTransform { source_id }) => {
                Box::new(MatchTransform::new(source_id, spec.target_id, spec.ease))
            }
            Some(AnimKind::MorphFrom { source_id }) => {
                Box::new(MorphObjects::new(source_id, spec.target_id, spec.ease))
            }
            Some(AnimKind::EquationContinuityFrom { source_id }) => {
                Box::new(EquationContinuity::new(source_id, spec.target_id, spec.ease))
            }
            Some(AnimKind::MatrixStepCells {
                cells,
                highlight,
                dim_opacity,
            }) => Box::new(MatrixStep::cells(
                spec.target_id,
                cells,
                highlight,
                dim_opacity,
                spec.ease,
            )),
            Some(AnimKind::MatrixStepRow {
                row,
                highlight,
                dim_opacity,
            }) => Box::new(MatrixStep::row(
                spec.target_id,
                row,
                highlight,
                dim_opacity,
                spec.ease,
            )),
            Some(AnimKind::MatrixStepColumn {
                col,
                highlight,
                dim_opacity,
            }) => Box::new(MatrixStep::column(
                spec.target_id,
                col,
                highlight,
                dim_opacity,
                spec.ease,
            )),
            Some(AnimKind::FollowAnchor {
                target_id,
                target_anchor,
                follower_anchor,
                offset,
            }) => Box::new(FollowAnchor::new(
                spec.target_id,
                target_id,
                target_anchor,
                follower_anchor,
                offset,
            )),
            Some(AnimKind::Propagate { to }) => {
                Box::new(PropagateSignal::new(spec.target_id, to, spec.ease))
            }
            None => panic!(
                "Murali Error: AnimationBuilder requires a kind before .spawn()"
            ),
        };

        self.timeline
            .add_animation(spec.start_time, spec.duration, anim);
    }
}
