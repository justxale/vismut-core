use crate::core::ExecutionContext;
use crate::core::{EdgeType, Node, Port, PortKind, ScriptError, Value};
use crate::register::NodeSchema;
use crate::traits::NodeBehavior;
use petgraph::prelude::StableDiGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;
pub struct AddNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for AddNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self {
            a: Value::Int(0),
            b: Value::Int(0),
        })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
        self.b = defaults[&String::from("b")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
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

        match (input_a.as_ref().unwrap_or(&self.a), input_b.as_ref().unwrap_or(&self.b)) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f32) + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + (*b as f32))),
            _ => Err(ScriptError::UnsupportedInput)
        }
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.add"),
            false,
            true,
            vec![
                Port {
                    name: String::from("a"),
                    kind: PortKind::Data,
                },
                Port {
                    name: String::from("b"),
                    kind: PortKind::Data,
                },
            ],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct SubtractNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for SubtractNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self {
            a: Value::Int(0),
            b: Value::Int(0),
        })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
        self.b = defaults[&String::from("b")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
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

        match (input_a.as_ref().unwrap_or(&self.a), input_b.as_ref().unwrap_or(&self.b)) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f32) - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - (*b as f32))),
            _ => Err(ScriptError::UnsupportedInput)
        }
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.subtract"),
            false,
            true,
            vec![
                Port {
                    name: String::from("a"),
                    kind: PortKind::Data,
                },
                Port {
                    name: String::from("b"),
                    kind: PortKind::Data,
                },
            ],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct MultiplyNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for MultiplyNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self {
            a: Value::Int(0),
            b: Value::Int(0),
        })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
        self.b = defaults[&String::from("b")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
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

        match (input_a.as_ref().unwrap_or(&self.a), input_b.as_ref().unwrap_or(&self.b)) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f32) * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * (*b as f32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(ScriptError::UnsupportedInput)
        }
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.multiply"),
            false,
            true,
            vec![
                Port {
                    name: String::from("a"),
                    kind: PortKind::Data,
                },
                Port {
                    name: String::from("b"),
                    kind: PortKind::Data,
                },
            ],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct DivideNode {
    a: Value,
    b: Value,
}

impl DivideNode {
    fn new(a: Option<Value>, b: Option<Value>) -> Self {
        Self {
            a: a.unwrap_or(Value::Int(0)),
            b: b.unwrap_or(Value::Int(1)),
        }
    }
}

impl NodeBehavior for DivideNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self {
            a: Value::Int(0),
            b: Value::Int(1),
        })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
        self.b = defaults[&String::from("b")]
            .as_ref()
            .unwrap_or(&Value::Int(1))
            .to_owned();
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

        match (input_a.as_ref().unwrap_or(&self.a), input_b.as_ref().unwrap_or(&self.b)) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f32) / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / (*b as f32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err(ScriptError::UnsupportedInput)
        }
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.divide"),
            false,
            true,
            vec![
                Port {
                    name: String::from("a"),
                    kind: PortKind::Data,
                },
                Port {
                    name: String::from("b"),
                    kind: PortKind::Data,
                },
            ],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct ModuloNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for ModuloNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self {
            a: Value::Int(0),
            b: Value::Int(1),
        })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
        self.b = defaults[&String::from("b")]
            .as_ref()
            .unwrap_or(&Value::Int(1))
            .to_owned();
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

        match (input_a.as_ref().unwrap_or(&self.a), input_b.as_ref().unwrap_or(&self.b)) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
            _ => Err(ScriptError::UnsupportedInput)
        }
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.modulo"),
            false,
            true,
            vec![
                Port {
                    name: String::from("a"),
                    kind: PortKind::Data,
                },
                Port {
                    name: String::from("b"),
                    kind: PortKind::Data,
                },
            ],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct PowNode {
    a: Value,
    b: Value,
}

impl NodeBehavior for PowNode {
    fn new() -> Box<dyn NodeBehavior> {
        Box::new(Self {
            a: Value::Int(0),
            b: Value::Int(1),
        })
    }

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>) {
        self.a = defaults[&String::from("a")]
            .as_ref()
            .unwrap_or(&Value::Int(0))
            .to_owned();
        self.b = defaults[&String::from("b")]
            .as_ref()
            .unwrap_or(&Value::Int(1))
            .to_owned();
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

        match (input_a.as_ref().unwrap_or(&self.a), input_b.as_ref().unwrap_or(&self.b)) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a ^ b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f32).powf(*b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float((*a as f32).powi(*b))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            _ => Err(ScriptError::UnsupportedInput)
        }
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.power"),
            false,
            true,
            vec![
                Port {
                    name: String::from("a"),
                    kind: PortKind::Data,
                },
                Port {
                    name: String::from("b"),
                    kind: PortKind::Data,
                },
            ],
            vec![Port {
                name: String::from("result"),
                kind: PortKind::Data,
            }],
        )
    }
}

pub struct AbsNode {
    a: Value,
}

impl NodeBehavior for AbsNode {
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

        match input_a.as_ref().unwrap_or(&self.a) {
            Value::Int(a) => Ok(Value::Int(a.abs())),
            Value::Float(a) => Ok(Value::Float(a.abs())),
            _ => Err(ScriptError::UnsupportedInput)
        }
    }

    fn get_schema(&self) -> NodeSchema {
        NodeSchema::new(
            String::from("core.math.absolute"),
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
