use crate::core::{EdgeType, ExecutionContext, Node, Port, PortKind, ScriptError, Value, ValueType};
use crate::register::NodeSchema;
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

    fn evaluate(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        Err(ScriptError::NotEvaluable)
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.io.start"),
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
        match ctx.get_input(node, graph, "value")? {
            val => {
                println!("Print: {:?}", val);
            }
        }
        Ok(())
    }

    fn evaluate(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        Err(ScriptError::NotEvaluable)
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.io.print"),
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
}
