use crate::core::{EdgeType, ExecutionContext, Node, NodeBehavior, Port, PortKind, ScriptError, Value};
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use std::collections::HashMap;

pub struct SinNode {
    a: Value,
}

impl NodeBehavior for SinNode {
    fn new() -> Self {
        Self { a: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }

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
        if let Ok(Value::Int(a)) = input_a {
            return Ok(Value::Float((a as f32).sin()));
        }
        if let Ok(Value::Float(a)) = input_a {
            return Ok(Value::Float(a.sin()));
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("a"), kind: PortKind::Data, constant: Some(self.a.to_owned()) }]
    }

    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("result"), kind: PortKind::Data, constant: None }]
    }

    fn is_pure(&self) -> bool {
        true
    }
}

pub struct CosNode {
    a: Value,
}

impl NodeBehavior for CosNode {
    fn new() -> Self {
        Self { a: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }
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
        if let Ok(Value::Int(a)) = input_a {
            return Ok(Value::Float((a as f32).cos()));
        }
        if let Ok(Value::Float(a)) = input_a {
            return Ok(Value::Float(a.cos()));
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("a"), kind: PortKind::Data, constant: Some(self.a.to_owned()) }]
    }

    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("result"), kind: PortKind::Data, constant: None }]
    }

    fn is_pure(&self) -> bool {
        true
    }
}

pub struct TanNode {
    a: Value,
}

impl NodeBehavior for TanNode {
    fn new() -> Self {
        Self { a: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }

    fn execute(
        &self,
        _ctx: &mut ExecutionContext,
        _graph: &StableDiGraph<Node, EdgeType>,
        _node: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(&self, ctx: &mut ExecutionContext, graph: &StableDiGraph<Node, EdgeType>, node: NodeIndex, output_port: &str) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a");
        if let Ok(Value::Int(a)) = input_a {
            return Ok(Value::Float((a as f32).tan()));
        }
        if let Ok(Value::Float(a)) = input_a {
            return Ok(Value::Float(a.tan()));
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("a"), kind: PortKind::Data, constant: Some(self.a.to_owned()) }]
    }

    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("result"), kind: PortKind::Data, constant: None }]
    }

    fn is_pure(&self) -> bool {
        true
    }
}

pub struct CotNode {
    a: Value,
}

impl NodeBehavior for CotNode {
    fn new() -> Self {
        Self { a: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }

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
        if let Ok(Value::Int(a)) = input_a {
            return Ok(Value::Float((a as f32).tan().powi(-1)));
        }
        if let Ok(Value::Float(a)) = input_a {
            return Ok(Value::Float(a.tan().powi(-1)));
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("a"), kind: PortKind::Data, constant: Some(self.a.to_owned()) }]
    }

    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("result"), kind: PortKind::Data, constant: None }]
    }

    fn is_pure(&self) -> bool {
        true
    }
}
