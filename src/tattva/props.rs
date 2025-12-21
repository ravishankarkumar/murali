// src/tattva/props.rs
use crate::transform::Transform;
use glam::{Vec3, Quat};

#[derive(Debug, Clone)]
pub struct TattvaProps {
    /// Per-instance transform (translate/rotate/scale)
    pub transform: Transform,

    /// Generic optional visual properties
    pub color: Option<[f32; 3]>,
    pub visible: bool,

    /// Shape-specific optional params:
    pub size: Option<f32>,      // cube, square
    pub width: Option<f32>,     // rectangle
    pub height: Option<f32>,    // rectangle
    pub radius: Option<f32>,    // circle
    pub segments: Option<u32>,  // circle tessellation
    pub thickness: Option<f32>, // line thickness

    /// Line-specific parameters (3D positions)
    pub start: Option<Vec3>,
    pub end: Option<Vec3>,
}

impl Default for TattvaProps {
    fn default() -> Self {
        Self {
            transform: Transform::identity(),
            color: None,
            visible: true,

            size: None,
            width: None,
            height: None,
            radius: None,
            segments: None,
            thickness: None,

            start: None,
            end: None,
        }
    }
}

impl TattvaProps {
    pub fn new() -> Self { Self::default() }

    // ----- line-specific position setters -----

    pub fn with_start(mut self, start: Vec3) -> Self {
        self.start = Some(start);
        self
    }

    pub fn with_end(mut self, end: Vec3) -> Self {
        self.end = Some(end);
        self
    }

    // ----- transform setters -----

    pub fn with_position(mut self, p: Vec3) -> Self {
        self.transform.translate = p;
        self
    }

    pub fn with_rotation(mut self, r: Quat) -> Self {
        self.transform.rotate = r;
        self
    }

    pub fn with_scale(mut self, s: Vec3) -> Self {
        self.transform.scale = s;
        self
    }

    // ----- shape parameter setters -----

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_width_height(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    pub fn with_radius(mut self, r: f32) -> Self {
        self.radius = Some(r);
        self
    }

    pub fn with_segments(mut self, seg: u32) -> Self {
        self.segments = Some(seg);
        self
    }

    pub fn with_thickness(mut self, t: f32) -> Self {
        self.thickness = Some(t);
        self
    }

    pub fn with_color(mut self, color: [f32;3]) -> Self {
        self.color = Some(color);
        self
    }

    pub fn visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }
}
