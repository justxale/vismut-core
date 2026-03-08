use crate::core::ExecutionContext;
use crate::core::{EdgeType, Node, NodeBehavior, Port, PortKind, ScriptError, Value};
use petgraph::prelude::StableDiGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;

pub struct AddNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for AddNode {
    fn new() -> Self {
        Self { a: Value::Int(0), b: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
        self.b = defaults[&String::from("b")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a").unwrap_or(self.a.to_owned());
        let input_b = ctx.get_input(node, graph, "b").unwrap_or(self.b.to_owned());

        println!("{:?} {:?}", input_a, input_b);

        if let Value::Int(a) = input_a && let Value::Int(b) = input_b {
            return Ok(Value::Int(a + b));
        } else if let Value::Float(a) = input_a && let Value::Float(b) = input_b {
            return Ok(Value::Float(a + b))
        } else if let Value::Float(a) = input_a && let Value::Int(b) = input_b {
            return Ok(Value::Float(a + b as f32))
        } else if let Value::Int(a) = input_a && let Value::Float(b) = input_b {
            return Ok(Value::Float(b + a as f32))
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: String::from("a"), kind: PortKind::Data, constant: Some(self.a.to_owned()) },
            Port { name: String::from("b"), kind: PortKind::Data, constant: Some(self.b.to_owned()) },
        ]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: String::from("result"), kind: PortKind::Data, constant: None }]
    }
    fn is_pure(&self) -> bool { true }
}

pub struct SubtractNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for SubtractNode {
    fn new() -> Self {
        Self { a: Value::Int(0), b: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
        self.b = defaults[&String::from("b")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a").unwrap_or(self.a.to_owned());
        let input_b = ctx.get_input(node, graph, "b").unwrap_or(self.b.to_owned());

        if let Value::Int(a) = input_a && let Value::Int(b) = input_b {
            return Ok(Value::Int(a - b));
        } else if let Value::Float(a) = input_a && let Value::Float(b) = input_b {
            return Ok(Value::Float(a - b))
        } else if let Value::Float(a) = input_a && let Value::Int(b) = input_b {
            return Ok(Value::Float(a - b as f32))
        } else if let Value::Int(a) = input_a && let Value::Float(b) = input_b {
            return Ok(Value::Float(b - a as f32))
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "a".into(), kind: PortKind::Data, constant: Some(self.a.to_owned()) },
            Port { name: "b".into(), kind: PortKind::Data, constant: Some(self.b.to_owned()) },
        ]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: "result".into(), kind: PortKind::Data, constant: None }]
    }
    fn is_pure(&self) -> bool { true }
}

pub struct MultiplyNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for MultiplyNode {
    fn new() -> Self {
        Self { a: Value::Int(0), b: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
        self.b = defaults[&String::from("b")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a");
        let input_b = ctx.get_input(node, graph, "b");

        if let Ok(Value::Int(a)) = input_a && let Ok(Value::Int(b)) = input_b {
            return Ok(Value::Int(a * b));
        } else if let Ok(Value::Float(a)) = input_a && let Ok(Value::Float(b)) = input_b {
            return Ok(Value::Float(a * b))
        } else if let Ok(Value::Float(a)) = input_a && let Ok(Value::Int(b)) = input_b {
            return Ok(Value::Float(a * b as f32))
        } else if let Ok(Value::Int(a)) = input_a && let Ok(Value::Float(b)) = input_b {
            return Ok(Value::Float(b * a as f32))
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "a".into(), kind: PortKind::Data, constant: Some(self.a.to_owned()) },
            Port { name: "b".into(), kind: PortKind::Data, constant: Some(self.b.to_owned()) },
        ]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: "result".into(), kind: PortKind::Data, constant: None }]
    }
    fn is_pure(&self) -> bool { true }
}

pub struct DivideNode {
    a: Value,
    b: Value,
}

impl DivideNode {
    fn new(a: Option<Value>, b: Option<Value>) -> Self {
        Self { a: a.unwrap_or(Value::Int(0)), b: b.unwrap_or(Value::Int(1)) }
    }
}

impl NodeBehavior for DivideNode {
    fn new() -> Self {
        Self { a: Value::Int(0), b: Value::Int(1) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
        self.b = defaults[&String::from("b")].as_ref().unwrap_or(&Value::Int(1)).to_owned();
    }

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a").unwrap_or(self.a.to_owned());
        let input_b = ctx.get_input(node, graph, "b").unwrap_or(self.b.to_owned());

        if let Value::Int(a) = input_a && let Value::Int(b) = input_b {
            return Ok(Value::Int(a / b));
        } else if let Value::Float(a) = input_a && let Value::Float(b) = input_b {
            return Ok(Value::Float(a / b))
        } else if let Value::Float(a) = input_a && let Value::Int(b) = input_b {
            return Ok(Value::Float(a / b as f32))
        } else if let Value::Int(a) = input_a && let Value::Float(b) = input_b {
            return Ok(Value::Float(b / a as f32))
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "a".into(), kind: PortKind::Data, constant: Some(self.a.to_owned()) },
            Port { name: "b".into(), kind: PortKind::Data, constant: Some(self.b.to_owned()) },
        ]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: "result".into(), kind: PortKind::Data, constant: None }]
    }
    fn is_pure(&self) -> bool { true }
}

pub struct ModuloNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for ModuloNode {
    fn new() -> Self {
        Self { a: Value::Int(0), b: Value::Int(1) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
        self.b = defaults[&String::from("b")].as_ref().unwrap_or(&Value::Int(1)).to_owned();
    }

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a").unwrap_or(self.a.to_owned());
        let input_b = ctx.get_input(node, graph, "b").unwrap_or(self.b.to_owned());

        if let Value::Int(a) = input_a && let Value::Int(b) = input_b {
            return Ok(Value::Int(a % b));
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "a".into(), kind: PortKind::Data, constant: Some(self.a.to_owned()) },
            Port { name: "b".into(), kind: PortKind::Data, constant: Some(self.b.to_owned()) },
        ]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: "result".into(), kind: PortKind::Data, constant: None }]
    }
    fn is_pure(&self) -> bool { true }
}

pub struct PowNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for PowNode {
    fn new() -> Self {
        Self { a: Value::Int(0), b: Value::Int(1) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
        self.b = defaults[&String::from("b")].as_ref().unwrap_or(&Value::Int(1)).to_owned();
    }

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a");
        let input_b = ctx.get_input(node, graph, "b");

        if let Ok(Value::Int(a)) = input_a && let Ok(Value::Int(b)) = input_b {
            return Ok(Value::Int(a.pow(b as u32)));
        } else if let Ok(Value::Float(a)) = input_a && let Ok(Value::Float(b)) = input_b {
            return Ok(Value::Float(a.powf(b)))
        } else if let Ok(Value::Float(a)) = input_a && let Ok(Value::Int(b)) = input_b {
            return Ok(Value::Float(a.powi(b)))
        } else if let Ok(Value::Int(a)) = input_a && let Ok(Value::Float(b)) = input_b {
            return Ok(Value::Float(b.powf(b)))
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "a".into(), kind: PortKind::Data, constant: Some(self.a.to_owned()) },
            Port { name: "b".into(), kind: PortKind::Data, constant: Some(self.b.to_owned()) },
        ]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: "result".into(), kind: PortKind::Data, constant: None }]
    }
    fn is_pure(&self) -> bool { true }
}

pub struct AbsNode {
    a: Value,
}

impl NodeBehavior for AbsNode {
    fn new() -> Self {
        Self { a: Value::Int(0) }
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")].as_ref().unwrap_or(&Value::Int(0)).to_owned();
    }

    fn execute(
        &self,
        _: &mut ExecutionContext,
        _: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a");

        if let Ok(Value::Int(a)) = input_a {
            return Ok(Value::Int(a.abs()));
        }
        if let Ok(Value::Float(a)) = input_a {
            return Ok(Value::Float(a.abs()));
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "a".into(), kind: PortKind::Data, constant: Some(self.a.to_owned()) },
        ]
    }
    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: "result".into(), kind: PortKind::Data, constant: None }]
    }
    fn is_pure(&self) -> bool { true }
}
