use glam::{Vec2, Vec3, Vec4, vec2, vec3};
use std::collections::HashSet;
use std::sync::Arc;

use crate::backend::renderer::vertex::{mesh::MeshVertex, text::TextVertex};
use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::mesh::MeshData;
use crate::projection::{Mesh, Project, ProjectionCtx, RenderPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowChartDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNodeShape {
    Rectangle,
    Rounded,
    Pill,
    Diamond,
}

pub trait FlowNodeContent: std::fmt::Debug + Send + Sync {
    fn local_bounds(&self) -> Bounds;
    fn project(&self, ctx: &mut ProjectionCtx);
}

#[derive(Debug)]
pub struct ProjectedFlowNodeContent<T> {
    pub state: T,
}

impl<T> ProjectedFlowNodeContent<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> FlowNodeContent for ProjectedFlowNodeContent<T>
where
    T: Project + Bounded + std::fmt::Debug + Send + Sync + 'static,
{
    fn local_bounds(&self) -> Bounds {
        self.state.local_bounds()
    }

    fn project(&self, ctx: &mut ProjectionCtx) {
        self.state.project(ctx);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNodeContentVisibility {
    Always,
    ActiveOnly,
}

#[derive(Debug, Clone)]
pub struct FlowNode {
    pub label: String,
    pub shape: FlowNodeShape,
    pub size: Option<Vec2>,
    pub fill_color: Vec4,
    pub stroke_color: Vec4,
    pub text_color: Vec4,
    pub embedded_content: Option<Arc<dyn FlowNodeContent>>,
    pub content_visibility: FlowNodeContentVisibility,
    pub content_padding: Vec2,
}

impl FlowNode {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shape: FlowNodeShape::Rounded,
            size: None,
            fill_color: Vec4::new(0.16, 0.20, 0.28, 1.0),
            stroke_color: Vec4::new(0.56, 0.63, 0.74, 1.0),
            text_color: Vec4::new(0.96, 0.97, 0.99, 1.0),
            embedded_content: None,
            content_visibility: FlowNodeContentVisibility::Always,
            content_padding: vec2(0.22, 0.18),
        }
    }

    pub fn with_shape(mut self, shape: FlowNodeShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_fill_color(mut self, color: Vec4) -> Self {
        self.fill_color = color;
        self
    }

    pub fn with_stroke_color(mut self, color: Vec4) -> Self {
        self.stroke_color = color;
        self
    }

    pub fn with_text_color(mut self, color: Vec4) -> Self {
        self.text_color = color;
        self
    }

    pub fn with_content<T>(mut self, content: T) -> Self
    where
        T: Project + Bounded + std::fmt::Debug + Send + Sync + 'static,
    {
        self.embedded_content = Some(Arc::new(ProjectedFlowNodeContent::new(content)));
        self
    }

    pub fn with_content_renderer(mut self, content: Arc<dyn FlowNodeContent>) -> Self {
        self.embedded_content = Some(content);
        self
    }

    pub fn with_content_visibility(mut self, visibility: FlowNodeContentVisibility) -> Self {
        self.content_visibility = visibility;
        self
    }

    pub fn with_active_only_content(mut self) -> Self {
        self.content_visibility = FlowNodeContentVisibility::ActiveOnly;
        self
    }

    pub fn with_content_padding(mut self, padding: Vec2) -> Self {
        self.content_padding = padding;
        self
    }
}

impl From<&str> for FlowNode {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlowEdge {
    pub from: usize,
    pub to: usize,
}

impl FlowEdge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone)]
pub struct AgenticFlowChart {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
    pub flow_path: Vec<usize>,
    pub progress: f32,
    pub direction: FlowChartDirection,
    pub default_node_size: Vec2,
    pub text_height: f32,
    pub text_padding: Vec2,
    pub node_gap: f32,
    pub lane_gap: f32,
    pub edge_thickness: f32,
    pub edge_color: Vec4,
    pub active_edge_color: Vec4,
    pub pulse_color: Vec4,
    pub pulse_radius: f32,
    pub arrow_size: f32,
    pub indicate_color: Vec4,
    pub indicate_scale: f32,
    pub indicate_window: f32,
    pub active_content_nodes: HashSet<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlowNodeArrival {
    pub node_index: usize,
    pub visit_index: usize,
    pub time: f32,
}

#[derive(Debug, Clone, Copy)]
struct NodeLayout {
    center: Vec3,
    size: Vec2,
}

impl AgenticFlowChart {
    pub fn new(nodes: Vec<FlowNode>) -> Self {
        Self {
            nodes,
            edges: Vec::new(),
            flow_path: Vec::new(),
            progress: 0.0,
            direction: FlowChartDirection::Horizontal,
            default_node_size: vec2(2.2, 1.0),
            text_height: 0.22,
            text_padding: vec2(0.35, 0.22),
            node_gap: 1.0,
            lane_gap: 0.9,
            edge_thickness: 0.04,
            edge_color: Vec4::new(0.42, 0.49, 0.59, 1.0),
            active_edge_color: Vec4::new(0.98, 0.78, 0.28, 1.0),
            pulse_color: Vec4::new(1.0, 0.96, 0.82, 1.0),
            pulse_radius: 0.12,
            arrow_size: 0.16,
            indicate_color: Vec4::new(1.0, 0.90, 0.42, 1.0),
            indicate_scale: 0.12,
            indicate_window: 0.14,
            active_content_nodes: HashSet::new(),
        }
    }

    pub fn with_edges(mut self, edges: Vec<FlowEdge>) -> Self {
        self.edges = edges;
        self
    }

    pub fn with_flow_path(mut self, flow_path: Vec<usize>) -> Self {
        self.flow_path = flow_path;
        self
    }

    pub fn with_direction(mut self, direction: FlowChartDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn with_text_height(mut self, text_height: f32) -> Self {
        self.text_height = text_height;
        self
    }

    pub fn with_default_node_size(mut self, size: Vec2) -> Self {
        self.default_node_size = size;
        self
    }

    pub fn with_node_gap(mut self, gap: f32) -> Self {
        self.node_gap = gap;
        self
    }

    pub fn with_lane_gap(mut self, gap: f32) -> Self {
        self.lane_gap = gap;
        self
    }

    pub fn with_edge_color(mut self, color: Vec4) -> Self {
        self.edge_color = color;
        self
    }

    pub fn with_active_edge_color(mut self, color: Vec4) -> Self {
        self.active_edge_color = color;
        self
    }

    pub fn with_pulse_color(mut self, color: Vec4) -> Self {
        self.pulse_color = color;
        self
    }

    pub fn with_pulse_radius(mut self, radius: f32) -> Self {
        self.pulse_radius = radius;
        self
    }

    pub fn with_indicate_color(mut self, color: Vec4) -> Self {
        self.indicate_color = color;
        self
    }

    pub fn with_indicate_scale(mut self, scale: f32) -> Self {
        self.indicate_scale = scale;
        self
    }

    pub fn with_active_content_nodes(mut self, nodes: impl IntoIterator<Item = usize>) -> Self {
        self.active_content_nodes = nodes.into_iter().collect();
        self
    }

    pub fn activate_content_node(&mut self, node_index: usize) {
        self.active_content_nodes.insert(node_index);
    }

    pub fn deactivate_content_node(&mut self, node_index: usize) {
        self.active_content_nodes.remove(&node_index);
    }

    fn connection_pairs(&self) -> Vec<FlowEdge> {
        if !self.edges.is_empty() {
            return self.edges.clone();
        }

        let mut pairs = Vec::new();
        for window in self.flow_path.windows(2) {
            let edge = FlowEdge::new(window[0], window[1]);
            if !pairs.contains(&edge) {
                pairs.push(edge);
            }
        }
        pairs
    }

    fn resolved_node_size(&self, node: &FlowNode) -> Vec2 {
        if let Some(size) = node.size {
            return size;
        }

        let text_layout =
            crate::resource::text::layout::measure_label(&node.label, self.text_height);
        let mut size = vec2(
            self.default_node_size
                .x
                .max(text_layout.width + self.text_padding.x * 2.0),
            self.default_node_size
                .y
                .max(text_layout.height + self.text_padding.y * 2.0),
        );

        if let Some(content) = &node.embedded_content {
            let content_size = content.local_bounds().size();
            size.x = size.x.max(content_size.x + node.content_padding.x * 2.0);
            size.y = size.y.max(content_size.y + node.content_padding.y * 2.0);
        }

        size
    }

    fn node_layouts(&self) -> Vec<NodeLayout> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        let sizes: Vec<Vec2> = self
            .nodes
            .iter()
            .map(|node| self.resolved_node_size(node))
            .collect();

        let total_primary: f32 = sizes
            .iter()
            .map(|size| match self.direction {
                FlowChartDirection::Horizontal => size.x,
                FlowChartDirection::Vertical => size.y,
            })
            .sum::<f32>()
            + self.node_gap * self.nodes.len().saturating_sub(1) as f32;

        let mut cursor = -total_primary * 0.5;
        let mut layouts = Vec::with_capacity(self.nodes.len());
        for size in sizes {
            let center = match self.direction {
                FlowChartDirection::Horizontal => {
                    let x = cursor + size.x * 0.5;
                    cursor += size.x + self.node_gap;
                    Vec3::new(x, 0.0, 0.0)
                }
                FlowChartDirection::Vertical => {
                    let y = total_primary * 0.5 - (cursor.abs() + size.y * 0.5);
                    cursor += size.y + self.node_gap;
                    Vec3::new(0.0, y, 0.0)
                }
            };
            layouts.push(NodeLayout { center, size });
        }

        if self.direction == FlowChartDirection::Vertical {
            let mut cursor = total_primary * 0.5;
            for layout in &mut layouts {
                layout.center.y = cursor - layout.size.y * 0.5;
                cursor -= layout.size.y + self.node_gap;
            }
        }

        layouts
    }

    pub fn node_center(&self, node_index: usize) -> Option<Vec3> {
        self.node_layouts()
            .get(node_index)
            .map(|layout| layout.center)
    }

    pub fn node_arrivals(&self, start_time: f32, duration: f32) -> Vec<FlowNodeArrival> {
        if self.flow_path.is_empty() {
            return Vec::new();
        }

        let hop_count = self.flow_path.len().saturating_sub(1);
        self.flow_path
            .iter()
            .copied()
            .enumerate()
            .map(|(visit_index, node_index)| {
                let progress = if hop_count == 0 {
                    0.0
                } else {
                    visit_index as f32 / hop_count as f32
                };
                FlowNodeArrival {
                    node_index,
                    visit_index,
                    time: start_time + duration.max(0.0) * progress,
                }
            })
            .collect()
    }

    pub fn completion_time(&self, start_time: f32, duration: f32) -> Option<f32> {
        if self.flow_path.is_empty() {
            None
        } else {
            Some(start_time + duration.max(0.0))
        }
    }

    fn edge_route(&self, layouts: &[NodeLayout], from: usize, to: usize) -> Option<Vec<Vec3>> {
        let start = *layouts.get(from)?;
        let end = *layouts.get(to)?;
        let span = from.abs_diff(to).max(1) as f32;
        let lane_offset = self.lane_gap * span;
        let stub = 0.28;

        let points = match self.direction {
            FlowChartDirection::Horizontal => {
                if to == from + 1 {
                    let start_anchor =
                        Vec3::new(start.center.x + start.size.x * 0.5, start.center.y, 0.0);
                    let end_anchor = Vec3::new(end.center.x - end.size.x * 0.5, end.center.y, 0.0);
                    vec![start_anchor, end_anchor]
                } else if to > from {
                    let start_anchor =
                        Vec3::new(start.center.x + start.size.x * 0.5, start.center.y, 0.0);
                    let end_anchor = Vec3::new(end.center.x - end.size.x * 0.5, end.center.y, 0.0);
                    let lane_y = lane_offset;
                    vec![
                        start_anchor,
                        Vec3::new(start_anchor.x + stub, start_anchor.y, 0.0),
                        Vec3::new(start_anchor.x + stub, lane_y, 0.0),
                        Vec3::new(end_anchor.x - stub, lane_y, 0.0),
                        Vec3::new(end_anchor.x - stub, end_anchor.y, 0.0),
                        end_anchor,
                    ]
                } else {
                    let start_anchor =
                        Vec3::new(start.center.x, start.center.y - start.size.y * 0.5, 0.0);
                    let end_anchor = Vec3::new(end.center.x, end.center.y - end.size.y * 0.5, 0.0);
                    let lane_y = start_anchor.y.min(end_anchor.y) - lane_offset;
                    vec![
                        start_anchor,
                        Vec3::new(start_anchor.x, lane_y, 0.0),
                        Vec3::new(end_anchor.x, lane_y, 0.0),
                        end_anchor,
                    ]
                }
            }
            FlowChartDirection::Vertical => {
                if to == from + 1 {
                    let start_anchor =
                        Vec3::new(start.center.x, start.center.y - start.size.y * 0.5, 0.0);
                    let end_anchor = Vec3::new(end.center.x, end.center.y + end.size.y * 0.5, 0.0);
                    vec![start_anchor, end_anchor]
                } else if to > from {
                    let start_anchor =
                        Vec3::new(start.center.x, start.center.y - start.size.y * 0.5, 0.0);
                    let end_anchor = Vec3::new(end.center.x, end.center.y + end.size.y * 0.5, 0.0);
                    let lane_x = lane_offset;
                    vec![
                        start_anchor,
                        Vec3::new(start_anchor.x, start_anchor.y - stub, 0.0),
                        Vec3::new(lane_x, start_anchor.y - stub, 0.0),
                        Vec3::new(lane_x, end_anchor.y + stub, 0.0),
                        Vec3::new(end_anchor.x, end_anchor.y + stub, 0.0),
                        end_anchor,
                    ]
                } else {
                    let start_anchor =
                        Vec3::new(start.center.x, start.center.y + start.size.y * 0.5, 0.0);
                    let end_anchor = Vec3::new(end.center.x, end.center.y - end.size.y * 0.5, 0.0);
                    let lane_x = -lane_offset;
                    vec![
                        start_anchor,
                        Vec3::new(start_anchor.x, start_anchor.y + stub, 0.0),
                        Vec3::new(lane_x, start_anchor.y + stub, 0.0),
                        Vec3::new(lane_x, end_anchor.y - stub, 0.0),
                        Vec3::new(end_anchor.x, end_anchor.y - stub, 0.0),
                        end_anchor,
                    ]
                }
            }
        };

        Some(dedup_points(points))
    }

    fn active_hop_state(&self) -> Option<(usize, f32)> {
        let hop_count = self.flow_path.len().checked_sub(1)?;
        if hop_count == 0 {
            return None;
        }

        let scaled = self.progress.clamp(0.0, 1.0) * hop_count as f32;
        let hop_idx = scaled.floor() as usize;
        if hop_idx >= hop_count {
            return Some((hop_count - 1, 1.0));
        }
        Some((hop_idx, scaled - hop_idx as f32))
    }

    fn pulse_position(&self, layouts: &[NodeLayout]) -> Option<Vec3> {
        if self.flow_path.is_empty() {
            return None;
        }
        if self.flow_path.len() == 1 {
            return self.node_center(self.flow_path[0]);
        }

        let (hop_idx, hop_t) = self.active_hop_state()?;
        let route = self.edge_route(
            layouts,
            self.flow_path[hop_idx],
            self.flow_path[hop_idx + 1],
        )?;
        sample_polyline(&route, hop_t)
    }

    fn node_indicate_intensity(&self, node_index: usize) -> f32 {
        if self.flow_path.is_empty() {
            return 0.0;
        }

        let hop_count = self.flow_path.len().saturating_sub(1) as f32;
        let mut best: f32 = 0.0;
        for (arrival_idx, path_node) in self.flow_path.iter().copied().enumerate() {
            if path_node != node_index {
                continue;
            }
            let arrival_t = if hop_count <= 0.0 {
                0.0
            } else {
                arrival_idx as f32 / hop_count
            };
            let delta = ((self.progress.clamp(0.0, 1.0) - arrival_t).abs()
                / self.indicate_window.max(1e-4))
            .clamp(0.0, 1.0);
            let intensity = (1.0 - delta).sin().max(0.0);
            best = best.max(intensity);
        }
        best
    }

    fn node_content_visible(&self, node_index: usize, node: &FlowNode) -> bool {
        match node.content_visibility {
            FlowNodeContentVisibility::Always => true,
            FlowNodeContentVisibility::ActiveOnly => {
                self.active_content_nodes.contains(&node_index)
            }
        }
    }

    fn draw_node(
        &self,
        ctx: &mut ProjectionCtx,
        node_index: usize,
        node: &FlowNode,
        layout: NodeLayout,
        scale: f32,
        stroke_color: Vec4,
        stroke_thickness: f32,
    ) {
        let size = layout.size * scale;
        let outline = shape_outline(node.shape, size);
        let mesh = shape_mesh(node.shape, size, node.fill_color)
            .as_ref()
            .translated(layout.center);
        ctx.emit(RenderPrimitive::Mesh(mesh));

        emit_closed_polyline(
            ctx,
            &translate_points(&outline, layout.center),
            stroke_thickness,
            stroke_color,
        );

        if let Some(content) = &node.embedded_content {
            if self.node_content_visible(node_index, node) {
                let inner_size = vec2(
                    (size.x - node.content_padding.x * 2.0).max(0.05),
                    (size.y - node.content_padding.y * 2.0).max(0.05),
                );
                project_content_in_node(ctx, content.as_ref(), layout.center, inner_size);
            }
        }

        if !node.label.is_empty() {
            ctx.emit(RenderPrimitive::Text {
                content: node.label.clone(),
                height: self.text_height * scale.min(1.08),
                color: node.text_color,
                offset: layout.center,
            });
        }
    }
}

impl Project for AgenticFlowChart {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let layouts = self.node_layouts();
        if layouts.is_empty() {
            return;
        }

        for edge in self.connection_pairs() {
            if let Some(route) = self.edge_route(&layouts, edge.from, edge.to) {
                emit_polyline(ctx, &route, self.edge_thickness, self.edge_color);
                emit_arrowhead(
                    ctx,
                    &route,
                    self.arrow_size,
                    self.edge_thickness,
                    self.edge_color,
                );
            }
        }

        if self.flow_path.len() >= 2 {
            let progress = self.progress.clamp(0.0, 1.0);
            let hop_count = self.flow_path.len() - 1;
            let scaled = progress * hop_count as f32;
            let current_hop = scaled.floor() as usize;
            let current_t = if current_hop >= hop_count {
                1.0
            } else {
                scaled - current_hop as f32
            };

            for hop in 0..hop_count {
                let Some(route) =
                    self.edge_route(&layouts, self.flow_path[hop], self.flow_path[hop + 1])
                else {
                    continue;
                };

                if hop < current_hop {
                    emit_polyline(
                        ctx,
                        &route,
                        self.edge_thickness * 1.15,
                        self.active_edge_color,
                    );
                    emit_arrowhead(
                        ctx,
                        &route,
                        self.arrow_size,
                        self.edge_thickness * 1.15,
                        self.active_edge_color,
                    );
                } else if hop == current_hop && progress < 1.0 {
                    let partial = partial_polyline(&route, current_t);
                    emit_polyline(
                        ctx,
                        &partial,
                        self.edge_thickness * 1.15,
                        self.active_edge_color,
                    );
                    emit_arrowhead(
                        ctx,
                        &partial,
                        self.arrow_size,
                        self.edge_thickness * 1.15,
                        self.active_edge_color,
                    );
                } else if hop == current_hop && progress >= 1.0 {
                    emit_polyline(
                        ctx,
                        &route,
                        self.edge_thickness * 1.15,
                        self.active_edge_color,
                    );
                    emit_arrowhead(
                        ctx,
                        &route,
                        self.arrow_size,
                        self.edge_thickness * 1.15,
                        self.active_edge_color,
                    );
                }
            }
        }

        for (idx, node) in self.nodes.iter().enumerate() {
            if let Some(layout) = layouts.get(idx).copied() {
                let intensity = self.node_indicate_intensity(idx);
                let node_scale = 1.0 + self.indicate_scale * intensity * 0.82;
                self.draw_node(
                    ctx,
                    idx,
                    node,
                    layout,
                    node_scale,
                    node.stroke_color,
                    self.edge_thickness * 0.9,
                );
            }
        }

        for (idx, node) in self.nodes.iter().enumerate() {
            let intensity = self.node_indicate_intensity(idx);
            if intensity <= 0.01 {
                continue;
            }
            if let Some(layout) = layouts.get(idx).copied() {
                let mut indicate_color = self.indicate_color;
                indicate_color.w *= 0.35 + intensity * 0.55;
                let scale = 1.0 + self.indicate_scale * intensity;
                let outline = shape_outline(node.shape, layout.size * scale);
                emit_closed_polyline(
                    ctx,
                    &translate_points(&outline, layout.center),
                    self.edge_thickness * (1.5 + intensity),
                    indicate_color,
                );
            }
        }

        if let Some(pulse_pos) = self.pulse_position(&layouts) {
            let mesh = Mesh::circle(self.pulse_radius, 28, self.pulse_color)
                .as_ref()
                .translated(pulse_pos);
            ctx.emit(RenderPrimitive::Mesh(mesh));
        }
    }
}

impl Bounded for AgenticFlowChart {
    fn local_bounds(&self) -> Bounds {
        let layouts = self.node_layouts();
        if layouts.is_empty() {
            return Bounds::from_center_size(Vec2::ZERO, vec2(0.01, 0.01));
        }

        let mut min = Vec2::splat(f32::INFINITY);
        let mut max = Vec2::splat(f32::NEG_INFINITY);
        let pad = self.pulse_radius
            + self.edge_thickness
            + self.arrow_size
            + self.indicate_scale.max(0.0);

        for layout in &layouts {
            min = min.min(layout.center.truncate() - layout.size * 0.5 - vec2(pad, pad));
            max = max.max(layout.center.truncate() + layout.size * 0.5 + vec2(pad, pad));
        }

        for edge in self.connection_pairs() {
            if let Some(route) = self.edge_route(&layouts, edge.from, edge.to) {
                for point in route {
                    min = min.min(point.truncate() - vec2(pad, pad));
                    max = max.max(point.truncate() + vec2(pad, pad));
                }
            }
        }

        Bounds::new(min, max)
    }
}

fn project_content_in_node(
    ctx: &mut ProjectionCtx,
    content: &dyn FlowNodeContent,
    node_center: Vec3,
    target_size: Vec2,
) {
    let source_bounds = content.local_bounds();
    let source_size = vec2(
        source_bounds.width().max(0.01),
        source_bounds.height().max(0.01),
    );
    let scale = (target_size.x / source_size.x)
        .min(target_size.y / source_size.y)
        .max(0.001);
    let source_center = source_bounds.center();
    let target_center = node_center.truncate();

    let mut subctx = ProjectionCtx::new(ctx.props.clone());
    content.project(&mut subctx);
    for primitive in subctx.primitives {
        emit_transformed_primitive(ctx, primitive, source_center, target_center, scale);
    }
}

fn emit_transformed_primitive(
    ctx: &mut ProjectionCtx,
    primitive: RenderPrimitive,
    source_center: Vec2,
    target_center: Vec2,
    scale: f32,
) {
    match primitive {
        RenderPrimitive::Mesh(mesh) => {
            let transformed = match &mesh.data {
                MeshData::Empty => std::sync::Arc::new(mesh.as_ref().clone()),
                MeshData::Mesh(vertices) => std::sync::Arc::new(Mesh {
                    data: MeshData::Mesh(
                        vertices
                            .iter()
                            .map(|vertex| MeshVertex {
                                position: transform_position(
                                    vertex.position,
                                    source_center,
                                    target_center,
                                    scale,
                                ),
                                color: vertex.color,
                            })
                            .collect(),
                    ),
                    indices: mesh.indices.clone(),
                }),
                MeshData::Text(vertices) => std::sync::Arc::new(Mesh {
                    data: MeshData::Text(
                        vertices
                            .iter()
                            .map(|vertex| TextVertex {
                                position: transform_position(
                                    vertex.position,
                                    source_center,
                                    target_center,
                                    scale,
                                ),
                                uv: vertex.uv,
                                color: vertex.color,
                            })
                            .collect(),
                    ),
                    indices: mesh.indices.clone(),
                }),
            };
            ctx.emit(RenderPrimitive::Mesh(transformed));
        }
        RenderPrimitive::Line {
            start,
            end,
            thickness,
            color,
            dash_length,
            gap_length,
            dash_offset,
        } => {
            ctx.emit(RenderPrimitive::Line {
                start: transform_vec3(start, source_center, target_center, scale),
                end: transform_vec3(end, source_center, target_center, scale),
                thickness: thickness * scale,
                color,
                dash_length: dash_length * scale,
                gap_length: gap_length * scale,
                dash_offset: dash_offset * scale,
            });
        }
        RenderPrimitive::Text {
            content,
            height,
            color,
            offset,
        } => {
            ctx.emit(RenderPrimitive::Text {
                content,
                height: height * scale,
                color,
                offset: transform_vec3(offset, source_center, target_center, scale),
            });
        }
        RenderPrimitive::Latex {
            source,
            height,
            color,
            offset,
        } => {
            ctx.emit(RenderPrimitive::Latex {
                source,
                height: height * scale,
                color,
                offset: transform_vec3(offset, source_center, target_center, scale),
            });
        }
        RenderPrimitive::Typst {
            source,
            height,
            color,
            offset,
        } => {
            ctx.emit(RenderPrimitive::Typst {
                source,
                height: height * scale,
                color,
                offset: transform_vec3(offset, source_center, target_center, scale),
            });
        }
    }
}

fn transform_vec3(point: Vec3, source_center: Vec2, target_center: Vec2, scale: f32) -> Vec3 {
    vec3(
        (point.x - source_center.x) * scale + target_center.x,
        (point.y - source_center.y) * scale + target_center.y,
        point.z * scale,
    )
}

fn transform_position(
    position: [f32; 3],
    source_center: Vec2,
    target_center: Vec2,
    scale: f32,
) -> [f32; 3] {
    [
        (position[0] - source_center.x) * scale + target_center.x,
        (position[1] - source_center.y) * scale + target_center.y,
        position[2] * scale,
    ]
}

fn emit_polyline(ctx: &mut ProjectionCtx, points: &[Vec3], thickness: f32, color: Vec4) {
    for segment in points.windows(2) {
        let start = segment[0];
        let end = segment[1];
        if start.distance_squared(end) <= 1e-6 {
            continue;
        }
        ctx.emit(RenderPrimitive::Line {
            start,
            end,
            thickness,
            color,
            dash_length: 0.0,
            gap_length: 0.0,
            dash_offset: 0.0,
        });
    }
}

fn emit_closed_polyline(ctx: &mut ProjectionCtx, points: &[Vec3], thickness: f32, color: Vec4) {
    if points.len() < 2 {
        return;
    }

    emit_polyline(ctx, points, thickness, color);
    if let (Some(&start), Some(&end)) = (points.first(), points.last()) {
        if start.distance_squared(end) > 1e-6 {
            emit_polyline(ctx, &[end, start], thickness, color);
        }
    }
}

fn emit_arrowhead(
    ctx: &mut ProjectionCtx,
    route: &[Vec3],
    arrow_size: f32,
    thickness: f32,
    color: Vec4,
) {
    let Some((&end, rest)) = route.split_last() else {
        return;
    };
    let Some(prev) = rest
        .iter()
        .rev()
        .find(|point| point.distance_squared(end) > 1e-6)
    else {
        return;
    };
    let direction = (end - *prev).normalize_or_zero();
    if direction.length_squared() <= 1e-6 {
        return;
    }

    let back = -direction * arrow_size;
    let normal = Vec3::new(-direction.y, direction.x, 0.0) * (arrow_size * 0.55);
    emit_polyline(
        ctx,
        &[end + back + normal, end, end + back - normal],
        thickness,
        color,
    );
}

fn translate_points(points: &[Vec2], offset: Vec3) -> Vec<Vec3> {
    points
        .iter()
        .map(|point| Vec3::new(point.x + offset.x, point.y + offset.y, offset.z))
        .collect()
}

fn shape_mesh(shape: FlowNodeShape, size: Vec2, color: Vec4) -> std::sync::Arc<Mesh> {
    match shape {
        FlowNodeShape::Rectangle => Mesh::rectangle(size.x, size.y, color),
        FlowNodeShape::Rounded | FlowNodeShape::Pill | FlowNodeShape::Diamond => {
            Mesh::polygon(shape_outline(shape, size), color)
        }
    }
}

fn shape_outline(shape: FlowNodeShape, size: Vec2) -> Vec<Vec2> {
    match shape {
        FlowNodeShape::Rectangle => rectangle_outline(size),
        FlowNodeShape::Rounded => {
            rounded_rect_outline(size, (size.x.min(size.y) * 0.22).min(size.y * 0.45), 8)
        }
        FlowNodeShape::Pill => rounded_rect_outline(size, (size.y * 0.5).min(size.x * 0.45), 10),
        FlowNodeShape::Diamond => vec![
            vec2(0.0, size.y * 0.5),
            vec2(size.x * 0.5, 0.0),
            vec2(0.0, -size.y * 0.5),
            vec2(-size.x * 0.5, 0.0),
        ],
    }
}

fn rectangle_outline(size: Vec2) -> Vec<Vec2> {
    vec![
        vec2(-size.x * 0.5, -size.y * 0.5),
        vec2(size.x * 0.5, -size.y * 0.5),
        vec2(size.x * 0.5, size.y * 0.5),
        vec2(-size.x * 0.5, size.y * 0.5),
    ]
}

fn rounded_rect_outline(size: Vec2, radius: f32, arc_steps: usize) -> Vec<Vec2> {
    let half = size * 0.5;
    let r = radius.max(0.01).min(half.x.min(half.y));
    let centers = [
        vec2(half.x - r, half.y - r),
        vec2(-(half.x - r), half.y - r),
        vec2(-(half.x - r), -(half.y - r)),
        vec2(half.x - r, -(half.y - r)),
    ];
    let ranges = [
        (0.0, std::f32::consts::FRAC_PI_2),
        (std::f32::consts::FRAC_PI_2, std::f32::consts::PI),
        (std::f32::consts::PI, std::f32::consts::PI * 1.5),
        (std::f32::consts::PI * 1.5, std::f32::consts::TAU),
    ];

    let mut points = Vec::new();
    for (center, (start, end)) in centers.into_iter().zip(ranges) {
        for step in 0..=arc_steps {
            let t = step as f32 / arc_steps.max(1) as f32;
            let angle = start + (end - start) * t;
            points.push(center + vec2(angle.cos() * r, angle.sin() * r));
        }
    }
    points
}

fn dedup_points(points: Vec<Vec3>) -> Vec<Vec3> {
    let mut result = Vec::with_capacity(points.len());
    for point in points {
        if result
            .last()
            .map(|prev: &Vec3| prev.distance_squared(point) <= 1e-6)
            .unwrap_or(false)
        {
            continue;
        }
        result.push(point);
    }
    result
}

fn polyline_length(points: &[Vec3]) -> f32 {
    points
        .windows(2)
        .map(|segment| segment[0].distance(segment[1]))
        .sum()
}

fn sample_polyline(points: &[Vec3], t: f32) -> Option<Vec3> {
    let partial = partial_polyline(points, t);
    partial.last().copied()
}

fn partial_polyline(points: &[Vec3], t: f32) -> Vec<Vec3> {
    if points.is_empty() {
        return Vec::new();
    }
    if points.len() == 1 {
        return points.to_vec();
    }

    let total_length = polyline_length(points);
    if total_length <= 1e-6 {
        return vec![points[0], *points.last().unwrap_or(&points[0])];
    }

    let target_length = total_length * t.clamp(0.0, 1.0);
    let mut traveled = 0.0;
    let mut result = vec![points[0]];

    for segment in points.windows(2) {
        let start = segment[0];
        let end = segment[1];
        let segment_length = start.distance(end);
        if segment_length <= 1e-6 {
            continue;
        }

        if traveled + segment_length < target_length {
            result.push(end);
            traveled += segment_length;
            continue;
        }

        let remaining = (target_length - traveled).clamp(0.0, segment_length);
        let local_t = remaining / segment_length;
        result.push(start.lerp(end, local_t));
        return dedup_points(result);
    }

    dedup_points(points.to_vec())
}
