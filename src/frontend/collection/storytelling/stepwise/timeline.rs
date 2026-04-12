use crate::frontend::collection::storytelling::stepwise::model::*;
use crate::frontend::collection::storytelling::stepwise::state::*;

pub struct TimelineEngine;

impl TimelineEngine {
    pub fn compute(model: &StepwiseModel, progress: f32) -> StepwiseState {
        let num_steps = model.sequence.len();

        if num_steps == 0 {
            return StepwiseState { steps: vec![], transitions: vec![] };
        }

        let p = progress.clamp(0.0, 1.0);

        if p >= 1.0 {
            return StepwiseState {
                steps: vec![StepState::Completed; model.steps.len()],
                transitions: vec![TransitionState::Completed; model.transitions.len()],
            };
        }

        // Precompute sequence index lookup (O(n))
        let mut seq_index = vec![None; model.steps.len()];
        for (i, &step_idx) in model.sequence.iter().enumerate() {
            seq_index[step_idx] = Some(i);
        }

        let segment_size = 1.0 / num_steps as f32;
        let raw_index    = p / segment_size;
        let active_idx   = (raw_index.floor() as usize).min(num_steps - 1);
        let local_t      = ((p - active_idx as f32 * segment_size) / segment_size).clamp(0.0, 1.0);

        // Staggering:
        // Transition (i-1 -> i) grows from local_t 0.0 -> 0.6
        // Node i reveals from local_t 0.4 -> 1.0
        let edge_reveal_duration = 0.6_f32;
        let node_reveal_start    = 0.4_f32;
        let node_reveal_duration = 0.6_f32;

        let cur_edge_t = (local_t / edge_reveal_duration).clamp(0.0, 1.0);
        let cur_node_t = ((local_t - node_reveal_start) / node_reveal_duration).clamp(0.0, 1.0);

        // Step states
        let mut steps = vec![StepState::Pending; model.steps.len()];
        for (i, &step_idx) in model.sequence.iter().enumerate() {
            steps[step_idx] = if i < active_idx {
                StepState::Completed
            } else if i == active_idx {
                if i == 0 {
                    // First node reveals for the full duration
                    StepState::Active { t: local_t }
                } else {
                    StepState::Active { t: cur_node_t }
                }
            } else {
                StepState::Pending
            };
        }

        // Transition states
        let mut transitions = vec![TransitionState::Hidden; model.transitions.len()];
        for (i, transition) in model.transitions.iter().enumerate() {
            if let (Some(from_i), Some(to_i)) = (seq_index[transition.from], seq_index[transition.to]) {
                if from_i + 1 == to_i {
                    // Forward adjacent: staggered reveal
                    transitions[i] = if from_i < active_idx {
                        TransitionState::Completed
                    } else if from_i == active_idx {
                        // This is problematic. The transition *into* active_idx 
                        // should be animating when i == active_idx.
                        // Wait, my loop says transitions[i] depends on from_i == active_idx.
                        // That's for the edge *leaving* the current node.
                        // We want the edge *entering* the current node.
                        TransitionState::Hidden
                    } else {
                        TransitionState::Hidden
                    };
                    
                    // Fix: The edge flow from i-1 to i happens during active_idx == i
                    if to_i == active_idx && to_i > 0 {
                        transitions[i] = TransitionState::Drawing { t: cur_edge_t };
                    } else if to_i < active_idx {
                        transitions[i] = TransitionState::Completed;
                    }

                } else {
                    // Non-adjacent (back-edges, skip-edges): instant after 'from' node is done
                    transitions[i] = if from_i < active_idx {
                        TransitionState::Completed
                    } else {
                        TransitionState::Hidden
                    };
                }
            }
        }

        StepwiseState { steps, transitions }
    }
}
