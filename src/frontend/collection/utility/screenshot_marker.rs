use std::path::PathBuf;

use glam::{Vec2, vec2};

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx};

#[derive(Debug, Clone)]
pub struct ScreenshotMarker {
    pub output_path: PathBuf,
    pub armed: bool,
    pub captured: bool,
}

impl ScreenshotMarker {
    pub fn new(output_path: impl Into<PathBuf>) -> Self {
        Self {
            output_path: output_path.into(),
            armed: false,
            captured: false,
        }
    }

    pub fn arm(&mut self) {
        self.armed = true;
    }

    pub fn disarm(&mut self) {
        self.armed = false;
    }

    pub fn mark_captured(&mut self) {
        self.captured = true;
        self.armed = false;
    }

    pub fn reset_capture(&mut self) {
        self.armed = false;
        self.captured = false;
    }

    pub fn should_capture(&self) -> bool {
        self.armed && !self.captured
    }
}

impl Project for ScreenshotMarker {
    fn project(&self, _ctx: &mut ProjectionCtx) {}
}

impl Bounded for ScreenshotMarker {
    fn local_bounds(&self) -> Bounds {
        Bounds::from_center_size(Vec2::ZERO, vec2(0.01, 0.01))
    }
}
