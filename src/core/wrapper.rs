use crate::graph::{Node, EdgeType};
use crate::context::ExecutionContext;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use crate::common::{BoxedEvaluableFn, BoxedExecutableFn, BoxedNodeFn, ScriptError, Value};

pub struct CompiledNode {
    execute_fn: BoxedExecutableFn,
    evaluate_fn: BoxedEvaluableFn,
}

impl CompiledNode {
    pub fn new(execute_fn: BoxedExecutableFn, evaluate_fn: BoxedEvaluableFn) -> Self {
        Self {
            evaluate_fn, execute_fn
        }
    }

    pub fn execute(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<(), ScriptError> {
        (self.execute_fn)(ctx, graph, node)
    }

    pub fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        output_port: &str,
    ) -> Result<Value, ScriptError> {
        (self.evaluate_fn)(ctx, graph, node, output_port)
    }
}


pub struct NodeBuilder {
    execute_fn: Option<BoxedExecutableFn>,
    evaluate_fn: Option<BoxedEvaluableFn>,
    node_id: &'static str,
}

impl NodeBuilder {
    pub fn new(node_id: &'static str) -> Self {
        Self {
            node_id,
            execute_fn: None,
            evaluate_fn: None,
        }
    }

    pub fn with_execution(&mut self, execute_fn: BoxedExecutableFn) -> &Self {
        self.execute_fn = Some(execute_fn);
        self
    }

    pub fn with_evaluate(&mut self, evaluate_fn: BoxedEvaluableFn) -> &Self {
        self.evaluate_fn = Some(evaluate_fn);
        self
    }

    pub fn build(self) -> BoxedNodeFn {
        let exec = self.execute_fn.unwrap_or(Box::new(|_, _, _| Err(ScriptError::NotExecutable)));

        let eval = self.evaluate_fn.unwrap_or(Box::new(|_, _, _, _| Err(ScriptError::NotEvaluable)));

        Box::new(move || {
            CompiledNode::new(exec, eval)
        })
    }
}
