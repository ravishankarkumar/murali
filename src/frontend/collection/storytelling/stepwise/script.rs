use crate::frontend::collection::storytelling::stepwise::model::{
    Direction, Step, StepwiseModel, Transition,
};

pub fn stepwise<F: FnOnce(&mut ScriptBuilder)>(f: F) -> StepwiseModel {
    let mut builder = ScriptBuilder {
        steps: Vec::new(),
        explicit_connections: Vec::new(),
        has_explicit_connections: false,
    };
    f(&mut builder);
    builder.build()
}

pub struct ScriptBuilder {
    steps: Vec<Step>,
    explicit_connections: Vec<(usize, usize, Option<Vec<Direction>>)>,
    has_explicit_connections: bool,
}

impl ScriptBuilder {
    pub fn step(&mut self, label: &str) -> usize {
        let idx = self.steps.len();
        self.steps.push(Step {
            label: label.to_string(),
            content: None,
        });
        idx
    }

    pub fn connect(&mut self, from: usize, to: usize) -> ConnectionBuilder<'_> {
        let n = self.steps.len();
        for &idx in &[from, to] {
            if idx >= n {
                panic!(
                    "stepwise: connect() called with invalid step index {}; only {} steps have been added",
                    idx, n
                );
            }
        }
        self.has_explicit_connections = true;
        self.explicit_connections.push((from, to, None));
        let transition_index = self.explicit_connections.len() - 1;
        ConnectionBuilder {
            builder: self,
            transition_index,
        }
    }

    fn build(self) -> StepwiseModel {
        let n = self.steps.len();

        if !self.has_explicit_connections {
            // Auto-generate linear chain
            let sequence: Vec<usize> = (0..n).collect();
            let transitions: Vec<Transition> = (0..n.saturating_sub(1))
                .map(|i| Transition {
                    from: i,
                    to: i + 1,
                    route: None,
                })
                .collect();
            return StepwiseModel {
                steps: self.steps,
                transitions,
                sequence,
            };
        }

        // Build transitions from explicit connections
        let transitions: Vec<Transition> = self
            .explicit_connections
            .into_iter()
            .map(|(from, to, route)| Transition { from, to, route })
            .collect();

        // Topological sort via Kahn's algorithm
        let mut in_degree = vec![0usize; n];
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for t in &transitions {
            adj[t.from].push(t.to);
            in_degree[t.to] += 1;
        }

        let mut queue: std::collections::VecDeque<usize> = (0..n)
            .filter(|&i| in_degree[i] == 0)
            .collect();

        let mut sequence = Vec::with_capacity(n);
        while let Some(node) = queue.pop_front() {
            sequence.push(node);
            for &neighbor in &adj[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        // If cycle exists, remaining nodes are not added (spec says use whatever was produced)

        StepwiseModel {
            steps: self.steps,
            transitions,
            sequence,
        }
    }
}

pub struct ConnectionBuilder<'a> {
    builder: &'a mut ScriptBuilder,
    transition_index: usize,
}

impl<'a> ConnectionBuilder<'a> {
    pub fn route(self, directions: Vec<Direction>) -> Self {
        self.builder.explicit_connections[self.transition_index].2 = Some(directions);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::collection::storytelling::stepwise::{
        model::Direction,
        timeline::TimelineEngine,
    };

    // Feature: stepwise-component, Property 1: Step index assignment is sequential
    // Validates: Requirements 1.2
    proptest::proptest! {
        #[test]
        fn prop_step_index_sequential(labels in proptest::collection::vec(".*", 1usize..=20)) {
            let n = labels.len();
            let model = stepwise(|s| {
                for label in &labels {
                    s.step(label);
                }
            });
            proptest::prop_assert_eq!(model.steps.len(), n);
            for (i, label) in labels.iter().enumerate() {
                proptest::prop_assert_eq!(&model.steps[i].label, label);
            }
        }
    }

    // Feature: stepwise-component, Property 2: Auto-generated model is a linear chain
    // Validates: Requirements 1.3, 8.2, 8.3
    proptest::proptest! {
        #[test]
        fn prop_auto_linear_chain(n in 1usize..=20) {
            let model = stepwise(|s| {
                for i in 0..n {
                    s.step(&format!("step_{}", i));
                }
            });
            let expected_seq: Vec<usize> = (0..n).collect();
            proptest::prop_assert_eq!(&model.sequence, &expected_seq);
            proptest::prop_assert_eq!(model.transitions.len(), n.saturating_sub(1));
            for i in 0..n.saturating_sub(1) {
                proptest::prop_assert_eq!(model.transitions[i].from, i);
                proptest::prop_assert_eq!(model.transitions[i].to, i + 1);
            }
        }
    }

    // Feature: stepwise-component, Property 3: Sequence is a permutation of step indices
    // Validates: Requirements 1.4
    proptest::proptest! {
        #[test]
        fn prop_sequence_is_permutation(n in 1usize..=10) {
            let model = stepwise(|s| {
                for i in 0..n {
                    s.step(&format!("step_{}", i));
                }
                // Connect i -> i+1 to form a linear DAG
                for i in 0..n.saturating_sub(1) {
                    s.connect(i, i + 1);
                }
            });
            let mut sorted = model.sequence.clone();
            sorted.sort();
            let expected: Vec<usize> = (0..n).collect();
            proptest::prop_assert_eq!(sorted, expected);
        }
    }

    // Feature: stepwise-component, Property 4: Explicit connections suppress auto-generation
    // Validates: Requirements 2.2
    proptest::proptest! {
        #[test]
        fn prop_explicit_connections_suppress_auto(n in 2usize..=10) {
            let model = stepwise(|s| {
                for i in 0..n {
                    s.step(&format!("step_{}", i));
                }
                s.connect(0, 1);
            });
            // Only the one explicit connection should exist
            proptest::prop_assert_eq!(model.transitions.len(), 1);
            proptest::prop_assert_eq!(model.transitions[0].from, 0);
            proptest::prop_assert_eq!(model.transitions[0].to, 1);
        }
    }

    // Feature: stepwise-component, Property 5: Route is stored on the transition
    // Validates: Requirements 3.2
    proptest::proptest! {
        #[test]
        fn prop_route_stored_on_transition(dirs in proptest::collection::vec(0u8..4, 0usize..=10)) {
            let directions: Vec<Direction> = dirs.iter().map(|&d| match d {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => Direction::Right,
            }).collect();
            let dirs_clone = directions.clone();
            let model = stepwise(|s| {
                s.step("a");
                s.step("b");
                s.connect(0, 1).route(dirs_clone.clone());
            });
            proptest::prop_assert_eq!(&model.transitions[0].route, &Some(directions));
        }
    }

    // Feature: stepwise-component, Property 8: Script_API output is functionally equivalent to manual construction
    // Validates: Requirements 8.1
    proptest::proptest! {
        #[test]
        fn prop_script_api_equivalent_to_manual(
            labels in proptest::collection::vec("[a-z]{1,8}", 1usize..=8),
            p in 0.0f32..=1.0f32
        ) {
            use crate::frontend::collection::storytelling::stepwise::model::{Step, StepwiseModel, Transition};

            let n = labels.len();

            // Build via Script API
            let script_model = stepwise(|s| {
                for label in &labels {
                    s.step(label);
                }
            });

            // Build manually
            let manual_model = StepwiseModel {
                steps: labels.iter().map(|l| Step { label: l.clone(), content: None }).collect(),
                transitions: (0..n.saturating_sub(1))
                    .map(|i| Transition { from: i, to: i + 1, route: None })
                    .collect(),
                sequence: (0..n).collect(),
            };

            let script_state = TimelineEngine::compute(&script_model, p);
            let manual_state = TimelineEngine::compute(&manual_model, p);

            proptest::prop_assert_eq!(script_state, manual_state);
        }
    }
}
