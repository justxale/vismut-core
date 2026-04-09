use crate::graph::{Node, EdgeType};
use std::collections::HashMap;
use petgraph::{Direction};
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use petgraph::visit::EdgeRef;
use crate::{ScriptError, Value};

pub struct ExecutionContext {
    pub cache: HashMap<(NodeIndex, String), Value>,
}

impl ExecutionContext {
    pub fn get_input(
        &mut self,
        node: NodeIndex,
        graph: &StableDiGraph<Node, EdgeType>,
        input_port: &str,
    ) -> Result<Value, ScriptError> {
        for edge in graph.edges_directed(node, Direction::Incoming) {
            if let EdgeType::Data { from_port, to_port } = edge.weight() {
                if to_port == input_port {
                    let source = edge.source();

                    return self.evaluate(source, graph, &from_port);
                }
            }
        }

        Err(ScriptError::MissingInput)
    }

    pub fn evaluate(
        &mut self,
        node: NodeIndex,
        graph: &StableDiGraph<Node, EdgeType>,
        output_port: &str,
    ) -> Result<Value, ScriptError> {
        if let Some(v) = self.cache.get(&(node, output_port.to_string())) {
            return Ok(v.clone());
        }
        let behavior = &graph[node].behavior;
        let value = behavior.evaluate(self, graph, node, output_port)?;
        self.cache
            .insert((node, output_port.to_string()), value.clone());

        Ok(value)
    }
}