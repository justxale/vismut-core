use crate::core::ExecutionContext;
use crate::core::{EdgeType, Node, NodeBehavior, Port, PortKind, ScriptError, Value};
use petgraph::prelude::StableDiGraph;
use petgraph::stable_graph::NodeIndex;

pub struct AddNode {
    a: i32,
    b: i32,
}

impl AddNode {
    pub fn new(a: Option<i32>, b: Option<i32>) -> Self {
        Self { a: a.unwrap_or(0), b: b.unwrap_or(0) }
    }
}

impl NodeBehavior for AddNode {
    fn execute(
        &self,
        _: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        _: NodeIndex
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn input_ports(&self) -> Vec<Port> {
        vec![
            Port { name: "a".into(), kind: PortKind::Data, constant: Some(Value::Int(self.a)) },
            Port { name: "b".into(), kind: PortKind::Data, constant: Some(Value::Int(self.b)) },
        ]
    }

    fn output_ports(&self) -> Vec<Port> {
        vec![Port { name: "result".into(), kind: PortKind::Data, constant: None }]
    }
    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        _: &str
    ) -> Result<Value, ScriptError> {
        let a = match ctx.get_input(node, graph,"a") {
            Ok(Value::Int(v)) => v,
            Ok(Value::None) => self.a,
            Err(e) => {println!("{:?}", e); unimplemented!()},
            Ok(Value::Float(_)) | Ok(Value::Bool(_)) | Ok(Value::String(_)) => todo!(),
        };

        let b = match ctx.get_input(node, graph, "b") {
            Ok(Value::Int(v)) => v,
            Ok(Value::None) => self.b,
            _ => unimplemented!(),
        };

        println!("Added");
        Ok(Value::Int(a + b))
    }
    fn is_pure(&self) -> bool { true }
}