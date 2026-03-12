use crate::core::{EdgeType, ExecutionContext, Node, ScriptError, Value};
use crate::register::NodeSchema;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use std::collections::HashMap;
pub trait NodeBehavior {
    fn new() -> Box<dyn NodeBehavior>
    where
        Self: Sized;

    fn set_values(&mut self, defaults: HashMap<String, Option<Value>>);

    fn execute(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<(), ScriptError>;

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        output_port: &str,
    ) -> Result<Value, ScriptError>;
    fn get_schema(&self) -> NodeSchema;
}
