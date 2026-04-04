use crate::engine::timeline::Timeline;
use crate::frontend::TattvaId;
use crate::frontend::animation::{
    Animation, BeltEvolve, BeltPhaseBy, BeltPhaseTo, Create, Ease, EquationContinuity, FadeTo,
    FollowAnchor, MatchTransform, MatrixStep, MorphGeometry, MorphObjects, MoveTo, NoiseEvolve,
    NoisePhaseBy, NoisePhaseTo, PropagateSignal, RotateTo, ScaleTo, TriggerCapture,
};
use crate::frontend::layout::Anchor;
use glam::{Quat, Vec3, Vec4};

#[derive(Debug, Clone)]
pub enum AnimKind {
    MoveTo {
        to: Vec3,
        from: Option<Vec3>,
    },
    RotateTo {
        to: Quat,
        from: Option<Quat>,
    },
    ScaleTo {
        to: Vec3,
        from: Option<Vec3>,
    },
    FadeTo {
        opacity: f32,
        from: Option<f32>,
    },
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
    NoisePhaseTo {
        to: f32,
    },
    NoisePhaseBy {
        delta: f32,
    },
    NoiseEvolve {
        speed: Option<f32>,
    },
    BeltPhaseTo {
        to: f32,
    },
    BeltPhaseBy {
        delta: f32,
    },
    BeltEvolve {
        speed: Option<f32>,
    },
    CaptureFrame,
    Propagate {
        to: f32,
    },
}

#[derive(Debug, Clone)]
pub struct AnimationSpec {
    pub target_id: TattvaId,
    pub start_time: f32,
    pub duration: f32,
    pub ease: Option<Ease>,
    pub kind: Option<AnimKind>,
}

pub struct AnimationBuilder<'a> {
    timeline: &'a mut Timeline,
    target_id: TattvaId,
    spec: AnimationSpec,
}

impl<'a> AnimationBuilder<'a> {
    pub fn new(timeline: &'a mut Timeline, target_id: TattvaId) -> Self {
        Self {
            timeline,
            target_id,
            spec: AnimationSpec {
                target_id,
                start_time: 0.0,
                duration: 1.0,
                ease: None,
                kind: None,
            },
        }
    }

    pub fn at(mut self, start_time: f32) -> Self {
        self.spec.start_time = start_time;
        self
    }

    pub fn for_duration(mut self, duration: f32) -> Self {
        self.spec.duration = duration;
        self
    }

    pub fn ease(mut self, ease: Ease) -> Self {
        self.spec.ease = Some(ease);
        self
    }

    pub fn move_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(AnimKind::MoveTo { to, from: None });
        self
    }

    pub fn rotate_to(mut self, to: Quat) -> Self {
        self.spec.kind = Some(AnimKind::RotateTo { to, from: None });
        self
    }

    pub fn scale_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(AnimKind::ScaleTo { to, from: None });
        self
    }

    pub fn fade_to(mut self, opacity: f32) -> Self {
        self.spec.kind = Some(AnimKind::FadeTo {
            opacity,
            from: None,
        });
        self
    }

    pub fn from(mut self, value: impl Into<f32>) -> Self {
        let val = value.into();
        match self.spec.kind.take() {
            Some(AnimKind::FadeTo { opacity, .. }) => {
                self.spec.kind = Some(AnimKind::FadeTo {
                    opacity,
                    from: Some(val),
                });
            }
            Some(k) => self.spec.kind = Some(k),
            None => {}
        }
        self
    }

    pub fn from_vec3(mut self, value: Vec3) -> Self {
        match self.spec.kind.take() {
            Some(AnimKind::MoveTo { to, .. }) => {
                self.spec.kind = Some(AnimKind::MoveTo {
                    to,
                    from: Some(value),
                });
            }
            Some(AnimKind::ScaleTo { to, .. }) => {
                self.spec.kind = Some(AnimKind::ScaleTo {
                    to,
                    from: Some(value),
                });
            }
            Some(k) => self.spec.kind = Some(k),
            None => {}
        }
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

    pub fn matrix_step_column(mut self, col: usize, highlight: Vec4, dim_opacity: f32) -> Self {
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

    pub fn capture_frame(mut self) -> Self {
        self.spec.kind = Some(AnimKind::CaptureFrame);
        self
    }

    pub fn noise_phase_to(mut self, to: f32) -> Self {
        self.spec.kind = Some(AnimKind::NoisePhaseTo { to });
        self
    }

    pub fn noise_phase_by(mut self, delta: f32) -> Self {
        self.spec.kind = Some(AnimKind::NoisePhaseBy { delta });
        self
    }

    pub fn noise_evolve(mut self) -> Self {
        self.spec.kind = Some(AnimKind::NoiseEvolve { speed: None });
        self
    }

    pub fn noise_evolve_with_speed(mut self, speed: f32) -> Self {
        self.spec.kind = Some(AnimKind::NoiseEvolve { speed: Some(speed) });
        self
    }

    pub fn belt_phase_to(mut self, to: f32) -> Self {
        self.spec.kind = Some(AnimKind::BeltPhaseTo { to });
        self
    }

    pub fn belt_phase_by(mut self, delta: f32) -> Self {
        self.spec.kind = Some(AnimKind::BeltPhaseBy { delta });
        self
    }

    pub fn belt_evolve(mut self) -> Self {
        self.spec.kind = Some(AnimKind::BeltEvolve { speed: None });
        self
    }

    pub fn belt_evolve_with_speed(mut self, speed: f32) -> Self {
        self.spec.kind = Some(AnimKind::BeltEvolve { speed: Some(speed) });
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
        let ease = spec.ease.unwrap_or_default();

        let anim: Box<dyn Animation> = match spec.kind {
            Some(AnimKind::MoveTo { to, from }) => {
                let mut m = MoveTo::new(spec.target_id, to, ease);
                if let Some(f) = from {
                    m = m.with_from(f);
                }
                Box::new(m)
            }
            Some(AnimKind::RotateTo { to, from }) => {
                let mut m = RotateTo::new(spec.target_id, to, ease);
                if let Some(f) = from {
                    m = m.with_from(f);
                }
                Box::new(m)
            }
            Some(AnimKind::ScaleTo { to, from }) => {
                let mut m = ScaleTo::new(spec.target_id, to, ease);
                if let Some(f) = from {
                    m = m.with_from(f);
                }
                Box::new(m)
            }
            Some(AnimKind::FadeTo { opacity, from }) => {
                let mut m = FadeTo::new(spec.target_id, opacity, ease);
                if let Some(f) = from {
                    m = m.with_from(f);
                }
                Box::new(m)
            }
            Some(AnimKind::Create) => Box::new(Create::new(spec.target_id, ease)),
            Some(AnimKind::MatchTransform { source_id }) => {
                Box::new(MatchTransform::new(spec.target_id, source_id, ease))
            }
            Some(AnimKind::MorphFrom { source_id }) => {
                Box::new(MorphGeometry::new(source_id, spec.target_id, ease))
            }
            Some(AnimKind::EquationContinuityFrom { source_id }) => {
                Box::new(EquationContinuity::new(source_id, spec.target_id, ease))
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
                ease,
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
                ease,
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
                ease,
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
            Some(AnimKind::CaptureFrame) => Box::new(TriggerCapture::new(spec.target_id)),
            Some(AnimKind::NoisePhaseTo { to }) => {
                Box::new(NoisePhaseTo::new(spec.target_id, to, ease))
            }
            Some(AnimKind::NoisePhaseBy { delta }) => {
                Box::new(NoisePhaseBy::new(spec.target_id, delta, ease))
            }
            Some(AnimKind::NoiseEvolve { speed }) => {
                Box::new(NoiseEvolve::new(spec.target_id, spec.duration, speed, ease))
            }
            Some(AnimKind::BeltPhaseTo { to }) => {
                Box::new(BeltPhaseTo::new(spec.target_id, to, ease))
            }
            Some(AnimKind::BeltPhaseBy { delta }) => {
                Box::new(BeltPhaseBy::new(spec.target_id, delta, ease))
            }
            Some(AnimKind::BeltEvolve { speed }) => {
                Box::new(BeltEvolve::new(spec.target_id, spec.duration, speed, ease))
            }
            Some(AnimKind::Propagate { to }) => {
                Box::new(PropagateSignal::new(spec.target_id, to, ease))
            }
            None => panic!("Murali Error: AnimationBuilder requires a kind before .spawn()"),
        };

        self.timeline
            .add_animation(spec.start_time, spec.duration, anim);
    }
}
