use crate::nodes::random::random::RandomIntegerNode;
use crate::NodeBehavior;

#[cfg(feature = "nodes")]
pub mod random {
    use crate::core::ValueType;
    use crate::core::{EdgeType, ExecutionContext, Node, Port, PortKind, ScriptError, Value};
    use crate::registry::NodeSchema;
    use crate::traits::NodeBehavior;
    use petgraph::graph::NodeIndex;
    use petgraph::prelude::StableDiGraph;
    use rand::RngExt;
    use std::collections::HashMap;

    pub struct RandomIntegerNode {
        a: Value,
        b: Value,
    }

    impl NodeBehavior for RandomIntegerNode {
        fn new() -> Box<dyn NodeBehavior> {
            Box::new(Self {
                a: Value::Int(0),
                b: Value::Int(1),
            })
        }

        fn set_values(&mut self, _: &HashMap<String, Option<Value>>) {}

        fn execute(
            &self,
            _ctx: &mut ExecutionContext,
            _graph: &StableDiGraph<Node, EdgeType>,
            _node: NodeIndex,
        ) -> Result<(), ScriptError> {
            Err(ScriptError::NotExecutable)
        }

        fn evaluate(
            &self,
            ctx: &mut ExecutionContext,
            graph: &StableDiGraph<Node, EdgeType>,
            node: NodeIndex,
            _output_port: &str,
        ) -> Result<Value, ScriptError> {
            let input_a = ctx.get_input(node, graph, "a");
            let input_b = ctx.get_input(node, graph, "b");

            match (
                input_a.as_ref().unwrap_or(&self.a),
                input_b.as_ref().unwrap_or(&self.b),
            ) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(rand::rng().random_range(*a..=*b))),
                _ => Err(ScriptError::UnsupportedInput),
            }
        }

        fn get_schema(&self) -> NodeSchema {
            NodeSchema::new(
                self.get_id().to_string(),
                false,
                true,
                vec![
                    Port {
                        name: String::from("a"),
                        kind: PortKind::Data,
                        types: vec![ValueType::Int],
                    },
                    Port {
                        name: String::from("a"),
                        kind: PortKind::Data,
                        types: vec![ValueType::Int],
                    },
                ],
                vec![Port {
                    name: String::from("result"),
                    kind: PortKind::Data,
                    types: vec![ValueType::Int],
                }],
            )
        }

        fn get_id(&self) -> &str {
            "core.random.integer"
        }
    }
}

pub(crate) static RANDOM_NODE_FACTORIES: [fn() -> Box<dyn NodeBehavior>; 1] =
    [RandomIntegerNode::new];
