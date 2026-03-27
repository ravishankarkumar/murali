use glam::Vec4;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub background: Vec4,
    pub surface: Vec4,
    pub surface_alt: Vec4,
    pub text_primary: Vec4,
    pub text_muted: Vec4,
    pub accent: Vec4,
    pub accent_alt: Vec4,
    pub positive: Vec4,
    pub warning: Vec4,
}

impl Theme {
    pub fn ai_under_the_hood() -> Self {
        Self {
            name: "ai-under-the-hood",
            background: Vec4::new(0.04, 0.07, 0.11, 1.0),
            surface: Vec4::new(0.11, 0.18, 0.27, 1.0),
            surface_alt: Vec4::new(0.17, 0.25, 0.36, 1.0),
            text_primary: Vec4::new(0.95, 0.97, 0.99, 1.0),
            text_muted: Vec4::new(0.74, 0.81, 0.89, 1.0),
            accent: Vec4::new(0.34, 0.76, 0.96, 1.0),
            accent_alt: Vec4::new(0.67, 0.56, 0.98, 1.0),
            positive: Vec4::new(0.31, 0.82, 0.58, 1.0),
            warning: Vec4::new(0.98, 0.72, 0.26, 1.0),
        }
    }

    pub fn classroom_light() -> Self {
        Self {
            name: "classroom-light",
            background: Vec4::new(0.97, 0.96, 0.93, 1.0),
            surface: Vec4::new(0.88, 0.90, 0.92, 1.0),
            surface_alt: Vec4::new(0.81, 0.85, 0.89, 1.0),
            text_primary: Vec4::new(0.14, 0.17, 0.21, 1.0),
            text_muted: Vec4::new(0.35, 0.40, 0.46, 1.0),
            accent: Vec4::new(0.14, 0.51, 0.84, 1.0),
            accent_alt: Vec4::new(0.86, 0.40, 0.28, 1.0),
            positive: Vec4::new(0.24, 0.59, 0.38, 1.0),
            warning: Vec4::new(0.90, 0.58, 0.18, 1.0),
        }
    }
}
