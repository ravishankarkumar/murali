// Main agentic flow chart module
//
// This module provides a flexible flowchart visualization system with animation support.
// The implementation is split across multiple files for maintainability:
//
// - types.rs: Core enums and basic types
// - node.rs: FlowNode struct and related functionality  
// - edge.rs: FlowEdge struct and routing
// - chart.rs: Main AgenticFlowChart implementation

pub mod types;
pub mod node;
pub mod edge;
pub mod model;
mod shapes;
mod layout;
mod routing;
mod animation;
mod renderer;
mod chart;

// Re-export everything for backward compatibility
pub use types::*;
pub use node::{FlowNode, FlowNodeContent, ProjectedFlowNodeContent};
pub use edge::FlowEdge;
pub use model::{FlowChartModel, FlowChartStyle, FlowChartAnimation};
pub use chart::AgenticFlowChart;