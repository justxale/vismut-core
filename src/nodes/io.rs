use crate::core::{EdgeType, ExecutionContext, Node, NodeBehavior, Port, PortKind, ScriptError, Value};
use petgraph::prelude::StableDiGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;

pub struct StartNode;

impl NodeBehavior for StartNode {
    fn new() -> Self {
        Self
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {}

    fn execute(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<(), ScriptError> {
        Ok(())
    }

    fn evaluate(
        &self,
        _: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        Err(ScriptError::NotEvaluable)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port {
            name: "exec".into(),
            kind: PortKind::Execution,
            constant: None
        }]
    }

    fn is_pure(&self) -> bool {
        false
    }
}

pub struct PrintNode;

impl NodeBehavior for PrintNode {
    fn new() -> Self {
        Self
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {}

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
        println!("Printed");
        Ok(())
    }

    fn evaluate(
        &self,
        _: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        Err(ScriptError::NotEvaluable)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "value".into(), kind: PortKind::Data, constant: None }
        ]
    }

    fn output_ports(&self) -> Vec<Port> {
        vec![]
    }
    fn is_pure(&self) -> bool {
        false
    }
}