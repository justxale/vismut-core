use crate::core::{EdgeType, ExecutionContext, Node, Port, PortKind, ScriptError, Value};
use crate::register::NodeSchema;
use crate::traits::NodeBehavior;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use std::collections::HashMap;
pub struct SinNode {
    a: Value,
}

impl NodeBehavior for SinNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self { a: Value::Int(0) })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
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

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.sin"),
            false,
            true,
            vec![Port {
                name: String::from("a"),
                kind: PortKind::Data,
            }],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct CosNode {
    a: Value,
}

impl NodeBehavior for CosNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self { a: Value::Int(0) })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
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

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.cos"),
            false,
            true,
            vec![Port {
                name: String::from("a"),
                kind: PortKind::Data,
            }],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct TanNode {
    a: Value,
}

impl NodeBehavior for TanNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self { a: Value::Int(0) })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
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
        output_port: &str,
    ) -> Result<Value, ScriptError> {
        let input_a = ctx.get_input(node, graph, "a");
        if let Ok(Value::Int(a)) = input_a {
            return Ok(Value::Float((a as f32).tan()));
        }
        if let Ok(Value::Float(a)) = input_a {
            return Ok(Value::Float(a.tan()));
        }
        Err(ScriptError::UnsupportedInput)
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.tan"),
            false,
            true,
            vec![Port {
                name: String::from("a"),
                kind: PortKind::Data,
            }],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct CotNode {
    a: Value,
}

impl NodeBehavior for CotNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self { a: Value::Int(0) })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
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

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.cot"),
            false,
            true,
            vec![Port {
                name: String::from("a"),
                kind: PortKind::Data,
            }],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}
