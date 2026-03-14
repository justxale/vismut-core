use crate::core::{EdgeType, ExecutionContext, Node, Port, PortKind, ScriptError, Value, ValueType};
use crate::registry::NodeSchema;
use crate::traits::NodeBehavior;
use petgraph::prelude::StableDiGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;

pub struct StartNode;

impl NodeBehavior for StartNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self)
    }

    fn set_values(&mut self, _: HashMap<String, Option<Value>>) {}

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Ok(())
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            self.get_id().to_string(),
            true,
            false,
            vec![],
            vec![Port {
                name: String::from("exec"),
                kind: PortKind::Execution,
                types: vec![]
            }],
        )
    }
    
    fn get_id(&self) -> &str {
        "core.io.start"
    }
}

pub struct PrintNode;

impl NodeBehavior for PrintNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self)
    }

    fn set_values(&mut self, _: HashMap<String, Option<Value>>) {}

    fn execute(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<(), ScriptError> {
        match ctx.get_input(node, graph, "value") {
            Ok(val) => {
                println!("Print: {:?}", val);
            }
            Err(_) => {
                println!("Print failed");
            }
        }
        Ok(())
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            self.get_id().to_string(),
            true,
            false,
            vec![
                Port {
                    name: String::from("value"),
                    kind: PortKind::Data,
                    types: vec![ValueType::Any]
                },
                Port {
                    name: String::from("exec"),
                    kind: PortKind::Execution,
                    types: vec![]
                },
            ],
            vec![],
        )
    }
    
    fn get_id(&self) -> &str {
        "core.io.print"
    }
}
