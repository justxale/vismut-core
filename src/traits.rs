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
        _ctx: &mut ExecutionContext,
        _graph: &StableDiGraph<Node, EdgeType>,
        _node: NodeIndex,
    ) -> Result<(), ScriptError> {
        Err(ScriptError::NotExecutable)
    }

    fn evaluate(
        &self,
        _ctx: &mut ExecutionContext,
        _graph: &StableDiGraph<Node, EdgeType>,
        _node: NodeIndex,
        _output_port: &str,
    ) -> Result<Value, ScriptError> {
        Err(ScriptError::NotEvaluable)
    }
    fn get_schema(&self) -> NodeSchema;
    
    fn get_id(&self) -> &str;
}
