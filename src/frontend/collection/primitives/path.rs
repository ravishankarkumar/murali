use crate::frontend::layout::{Bounded, Bounds};
use crate::frontend::style::{StrokeParams, Style};
use crate::math::bezier::{cubic_bezier, quadratic_bezier};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use glam::{Vec2, Vec4, vec2, vec3};

#[derive(Debug, Clone, Copy)]
pub enum PathSegment {
    MoveTo(Vec2),
    LineTo(Vec2),
    QuadTo(Vec2, Vec2),        // control, end
    CubicTo(Vec2, Vec2, Vec2), // control1, control2, end
}

impl PathSegment {
    pub fn end_point(&self) -> Vec2 {
        match *self {
            PathSegment::MoveTo(p) => p,
            PathSegment::LineTo(p) => p,
            PathSegment::QuadTo(_, p) => p,
            PathSegment::CubicTo(_, _, p) => p,
        }
    }

    /// Splits a single segment into two at parametric value `t`.
    /// This is used for "smooth" resampling where we add points without changing shape.
    pub fn split(&self, start: Vec2, t: f32) -> (Self, Self) {
        match *self {
            PathSegment::MoveTo(p) => (PathSegment::MoveTo(p), PathSegment::MoveTo(p)), // Shouldn't happen
            PathSegment::LineTo(p) => {
                let mid = start.lerp(p, t);
                (PathSegment::LineTo(mid), PathSegment::LineTo(p))
            }
            PathSegment::QuadTo(ctrl, end) => {
                let l1 = start.lerp(ctrl, t);
                let l2 = ctrl.lerp(end, t);
                let mid = l1.lerp(l2, t);
                (PathSegment::QuadTo(l1, mid), PathSegment::QuadTo(l2, end))
            }
            PathSegment::CubicTo(ctrl1, ctrl2, end) => {
                let l1 = start.lerp(ctrl1, t);
                let l2 = ctrl1.lerp(ctrl2, t);
                let l3 = ctrl2.lerp(end, t);
                let q1 = l1.lerp(l2, t);
                let q2 = l2.lerp(l3, t);
                let mid = q1.lerp(q2, t);
                (
                    PathSegment::CubicTo(l1, q1, mid),
                    PathSegment::CubicTo(q2, l3, end),
                )
            }
        }
    }

    pub fn to_cubic(&self, start: Vec2) -> (Vec2, Vec2, Vec2) {
        match *self {
            PathSegment::MoveTo(p) => (start, p, p),
            PathSegment::LineTo(p) => (start, p, p),
            PathSegment::QuadTo(ctrl, end) => {
                let c1 = start + 2.0 / 3.0 * (ctrl - start);
                let c2 = end + 2.0 / 3.0 * (ctrl - end);
                (c1, c2, end)
            }
            PathSegment::CubicTo(c1, c2, end) => (c1, c2, end),
        }
    }

    pub fn lerp(&self, other: &Self, start: Vec2, other_start: Vec2, t: f32) -> Self {
        if let (PathSegment::MoveTo(p1), PathSegment::MoveTo(p2)) = (self, other) {
            return PathSegment::MoveTo(p1.lerp(*p2, t));
        }

        let (s_c1, s_c2, s_end) = self.to_cubic(start);
        let (o_c1, o_c2, o_end) = other.to_cubic(other_start);

        PathSegment::CubicTo(s_c1.lerp(o_c1, t), s_c2.lerp(o_c2, t), s_end.lerp(o_end, t))
    }
}

/// A Tattva for drawing complex paths consisting of lines and Bézier curves.
#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub style: Style,
    pub closed: bool,
}

impl Path {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            style: Style::new().with_stroke(StrokeParams::default()),
            closed: false,
        }
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        if let Some(stroke) = &mut self.style.stroke {
            stroke.thickness = thickness;
        }
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        if let Some(stroke) = &mut self.style.stroke {
            stroke.color = color;
        }
        self
    }

    pub fn with_dash(mut self, dash: f32, gap: f32) -> Self {
        if let Some(stroke) = &mut self.style.stroke {
            stroke.dash_length = dash;
            stroke.gap_length = gap;
        }
        self
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn move_to(mut self, p: Vec2) -> Self {
        self.segments.push(PathSegment::MoveTo(p));
        self
    }

    pub fn line_to(mut self, p: Vec2) -> Self {
        self.segments.push(PathSegment::LineTo(p));
        self
    }

    pub fn quad_to(mut self, ctrl: Vec2, end: Vec2) -> Self {
        self.segments.push(PathSegment::QuadTo(ctrl, end));
        self
    }

    pub fn cubic_to(mut self, ctrl1: Vec2, ctrl2: Vec2, end: Vec2) -> Self {
        self.segments.push(PathSegment::CubicTo(ctrl1, ctrl2, end));
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Increases the segment count of the path until it reaches `target_count`.
    /// Uses geometric subdivision (splitting segments at their midpoint) to preserve the exact shape.
    pub fn resample(&mut self, target_count: usize) {
        if self.segments.is_empty() || self.segments.len() >= target_count {
            return;
        }

        while self.segments.len() < target_count {
            let mut new_segments = Vec::with_capacity(target_count);
            let needed = target_count - self.segments.len();
            let mut split_this_round = 0;

            let split_ratio = if needed >= self.segments.len() {
                self.segments.len()
            } else {
                needed
            };

            let mut current = vec2(0.0, 0.0);
            for seg in self.segments.iter() {
                if split_this_round < split_ratio && !matches!(seg, PathSegment::MoveTo(_)) {
                    let (s1, s2) = seg.split(current, 0.5);
                    new_segments.push(s1);
                    new_segments.push(s2);
                    split_this_round += 1;
                } else {
                    new_segments.push(*seg);
                }
                current = seg.end_point();
            }

            self.segments = new_segments;
            if split_this_round == 0 {
                break;
            }
        }
    }

    /// Reorders the segments of a closed path to minimize the travel distance to another path.
    /// This prevents "spinning" or "twisting" during morphing.
    pub fn align_to(&self, other: &Self) -> Self {
        if self.segments.len() != other.segments.len() || !self.closed || self.segments.is_empty() {
            return self.clone();
        }

        let n = self.segments.len();
        let mut best_shift = 0;
        let mut min_dist_sq = f32::MAX;

        // Extract points for comparison
        let other_points: Vec<Vec2> = other.segments.iter().map(|s| s.end_point()).collect();
        let self_points: Vec<Vec2> = self.segments.iter().map(|s| s.end_point()).collect();

        for shift in 0..n {
            let mut current_dist_sq = 0.0;
            for i in 0..n {
                let p1 = self_points[(i + shift) % n];
                let p2 = other_points[i];
                current_dist_sq += (p1 - p2).length_squared();
            }

            if current_dist_sq < min_dist_sq {
                min_dist_sq = current_dist_sq;
                best_shift = shift;
            }
        }

        if best_shift == 0 {
            return self.clone();
        }

        // Perform the cyclic shift
        let mut new_segments = Vec::with_capacity(n);

        // The point before the new start becomes the new MoveTo
        let new_start_idx = (best_shift + n) % n;
        let start_point = if new_start_idx == 0 {
            self_points[n - 1]
        } else {
            self_points[new_start_idx - 1]
        };

        new_segments.push(PathSegment::MoveTo(start_point));
        for i in 0..n {
            let idx = (new_start_idx + i) % n;
            let seg = self.segments[idx];
            match seg {
                PathSegment::MoveTo(p) => {
                    new_segments.push(PathSegment::LineTo(p));
                }
                _ => new_segments.push(seg),
            }
        }

        let mut new_path = self.clone();
        new_path.segments = new_segments;
        new_path
    }

    pub fn lerp(&self, target: &Self, t: f32) -> Path {
        let mut new_segments = Vec::with_capacity(self.segments.len());
        let mut s_curr = vec2(0.0, 0.0);
        let mut t_curr = vec2(0.0, 0.0);

        for (s_seg, t_seg) in self.segments.iter().zip(target.segments.iter()) {
            new_segments.push(s_seg.lerp(t_seg, s_curr, t_curr, t));
            s_curr = s_seg.end_point();
            t_curr = t_seg.end_point();
        }

        Path {
            segments: new_segments,
            style: self.style.lerp(&target.style, t),
            closed: self.closed,
        }
    }
}

impl Project for Path {
    fn project(&self, ctx: &mut ProjectionCtx) {
        if self.segments.is_empty() {
            return;
        }

        let mut current_point = vec2(0.0, 0.0);
        let mut first_point = None;
        let mut cumulative_dist = 0.0;
        let mut all_points = Vec::new();

        for segment in &self.segments {
            match *segment {
                PathSegment::MoveTo(p) => {
                    current_point = p;
                    if first_point.is_none() {
                        first_point = Some(p);
                    }
                    all_points.push(p);
                }
                PathSegment::LineTo(p) => {
                    if let Some(stroke) = &self.style.stroke {
                        let len = (p - current_point).length();
                        ctx.emit(RenderPrimitive::Line {
                            start: vec3(current_point.x, current_point.y, 0.0),
                            end: vec3(p.x, p.y, 0.0),
                            thickness: stroke.thickness,
                            color: stroke.color,
                            dash_length: stroke.dash_length,
                            gap_length: stroke.gap_length,
                            dash_offset: stroke.dash_offset + cumulative_dist,
                        });
                        cumulative_dist += len;
                    }
                    current_point = p;
                    all_points.push(p);
                }
                PathSegment::QuadTo(ctrl, end) => {
                    let steps = 16;
                    let mut prev_p = current_point;
                    for i in 1..=steps {
                        let t = i as f32 / steps as f32;
                        let curr_p = quadratic_bezier(current_point, ctrl, end, t);
                        if let Some(stroke) = &self.style.stroke {
                            let len = (curr_p - prev_p).length();
                            ctx.emit(RenderPrimitive::Line {
                                start: vec3(prev_p.x, prev_p.y, 0.0),
                                end: vec3(curr_p.x, curr_p.y, 0.0),
                                thickness: stroke.thickness,
                                color: stroke.color,
                                dash_length: stroke.dash_length,
                                gap_length: stroke.gap_length,
                                dash_offset: stroke.dash_offset + cumulative_dist,
                            });
                            cumulative_dist += len;
                        }
                        prev_p = curr_p;
                        all_points.push(curr_p);
                    }
                    current_point = end;
                }
                PathSegment::CubicTo(ctrl1, ctrl2, end) => {
                    let steps = 24;
                    let mut prev_p = current_point;
                    for i in 1..=steps {
                        let t = i as f32 / steps as f32;
                        let curr_p = cubic_bezier(current_point, ctrl1, ctrl2, end, t);
                        if let Some(stroke) = &self.style.stroke {
                            let len = (curr_p - prev_p).length();
                            ctx.emit(RenderPrimitive::Line {
                                start: vec3(prev_p.x, prev_p.y, 0.0),
                                end: vec3(curr_p.x, curr_p.y, 0.0),
                                thickness: stroke.thickness,
                                color: stroke.color,
                                dash_length: stroke.dash_length,
                                gap_length: stroke.gap_length,
                                dash_offset: stroke.dash_offset + cumulative_dist,
                            });
                            cumulative_dist += len;
                        }
                        prev_p = curr_p;
                        all_points.push(curr_p);
                    }
                    current_point = end;
                }
            }
        }

        // Handle closed path stroke
        if self.closed {
            if let Some(first) = first_point {
                let len = (current_point - first).length();
                if len > 0.001 {
                    if let Some(stroke) = &self.style.stroke {
                        ctx.emit(RenderPrimitive::Line {
                            start: vec3(current_point.x, current_point.y, 0.0),
                            end: vec3(first.x, first.y, 0.0),
                            thickness: stroke.thickness,
                            color: stroke.color,
                            dash_length: stroke.dash_length,
                            gap_length: stroke.gap_length,
                            dash_offset: stroke.dash_offset + cumulative_dist,
                        });
                    }
                }
            }
        }

        // Handle Fill using Lyon Tessellator for robust triangulation
        if let Some(fill) = &self.style.fill {
            use crate::backend::renderer::vertex::mesh::MeshVertex;
            use lyon_tessellation as lyon;
            use lyon_tessellation::path::Path as LyonPath;
            use lyon_tessellation::{FillOptions, FillTessellator, VertexBuffers};

            let mut builder = LyonPath::builder();
            let mut current_pos = vec2(0.0, 0.0);
            let mut in_contour = false;

            for segment in &self.segments {
                match *segment {
                    PathSegment::MoveTo(p) => {
                        if in_contour {
                            builder.end(self.closed);
                        }
                        builder.begin(lyon::math::point(p.x, p.y));
                        current_pos = p;
                        in_contour = true;
                    }
                    PathSegment::LineTo(p) => {
                        if !in_contour {
                            builder.begin(lyon::math::point(current_pos.x, current_pos.y));
                            in_contour = true;
                        }
                        builder.line_to(lyon::math::point(p.x, p.y));
                        current_pos = p;
                    }
                    PathSegment::QuadTo(ctrl, end) => {
                        if !in_contour {
                            builder.begin(lyon::math::point(current_pos.x, current_pos.y));
                            in_contour = true;
                        }
                        builder.quadratic_bezier_to(
                            lyon::math::point(ctrl.x, ctrl.y),
                            lyon::math::point(end.x, end.y),
                        );
                        current_pos = end;
                    }
                    PathSegment::CubicTo(ctrl1, ctrl2, end) => {
                        if !in_contour {
                            builder.begin(lyon::math::point(current_pos.x, current_pos.y));
                            in_contour = true;
                        }
                        builder.cubic_bezier_to(
                            lyon::math::point(ctrl1.x, ctrl1.y),
                            lyon::math::point(ctrl2.x, ctrl2.y),
                            lyon::math::point(end.x, end.y),
                        );
                        current_pos = end;
                    }
                }
            }
            if in_contour {
                builder.end(self.closed);
            }

            let lpath = builder.build();
            let mut tessellator = FillTessellator::new();
            let mut geometry: VertexBuffers<lyon::math::Point, u16> = VertexBuffers::new();

            let res = tessellator.tessellate_path(
                &lpath,
                &FillOptions::default(),
                &mut lyon::geometry_builder::simple_builder(&mut geometry),
            );

            if res.is_ok() {
                let color_source = fill.clone();
                let get_color = |pos: [f32; 2]| -> [f32; 4] {
                    match &color_source {
                        crate::projection::style::ColorSource::Solid(c) => [c[0], c[1], c[2], c[3]],
                        crate::projection::style::ColorSource::LinearGradient {
                            start,
                            end,
                            stops,
                        } => {
                            let c = crate::projection::Mesh::evaluate_gradient(
                                glam::vec2(pos[0], pos[1]),
                                *start,
                                *end,
                                stops,
                            );
                            [c[0], c[1], c[2], c[3]]
                        }
                    }
                };

                let vertices: Vec<MeshVertex> = geometry
                    .vertices
                    .iter()
                    .map(|v| MeshVertex {
                        position: [v.x, v.y, 0.0],
                        color: get_color([v.x, v.y]),
                    })
                    .collect();

                let mesh = crate::projection::Mesh::from_tessellation(vertices, geometry.indices);
                ctx.emit(RenderPrimitive::Mesh(mesh));
            }
        }
    }
}

impl Bounded for Path {
    fn local_bounds(&self) -> Bounds {
        if self.segments.is_empty() {
            return Bounds::default();
        }

        let mut min = vec2(f32::MAX, f32::MAX);
        let mut max = vec2(f32::MIN, f32::MIN);

        let mut update_bounds = |p: Vec2| {
            min = vec2(min.x.min(p.x), min.y.min(p.y));
            max = vec2(max.x.max(p.x), max.y.max(p.y));
        };

        for segment in &self.segments {
            match *segment {
                PathSegment::MoveTo(p) => {
                    update_bounds(p);
                }
                PathSegment::LineTo(p) => {
                    update_bounds(p);
                }
                PathSegment::QuadTo(ctrl, end) => {
                    // For simplicity, we just include control points in the bounds.
                    // A tighter bound would involve finding the extrema of the Bézier curve.
                    update_bounds(ctrl);
                    update_bounds(end);
                }
                PathSegment::CubicTo(ctrl1, ctrl2, end) => {
                    update_bounds(ctrl1);
                    update_bounds(ctrl2);
                    update_bounds(end);
                }
            }
        }

        Bounds::new(min, max)
    }
}
