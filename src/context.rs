use crate::graph::{EdgeType, Node};
use std::collections::HashMap;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use petgraph::visit::EdgeRef;
use crate::{CompiledPort};
use crate::values::{Value, ValueState};

pub struct ExecutionContext {
    pub cache: HashMap<(NodeIndex, String), Value>,
}

impl ExecutionContext {
    pub fn get_input(
        &mut self,
        node: NodeIndex,
        graph: &StableDiGraph<Node, EdgeType>,
        port: &CompiledPort,
    ) -> ValueState {
        for edge in graph.edges_directed(node, Direction::Incoming) {
            if let EdgeType::Data { from_port, to_port } = edge.weight() {
                if *to_port == port.title() {
                    let source = edge.source();
                    return self.evaluate(source, &graph, from_port);
                }
            }
        }
        for t in port.types() {
            if t.is_default_supported() {
                return ValueState::Default;
            }
        }

        ValueState::Unset
    }

    pub fn evaluate(
        &mut self,
        node: NodeIndex,
        graph: &StableDiGraph<Node, EdgeType>,
        output_port: &String,
    ) -> ValueState {
        if let Some(v) = self.cache.get(&(node, output_port.to_string())) {
            return ValueState::Set(v.clone());
        }
        let behavior = &graph[node].node;
        match behavior.evaluate(self, graph, node, output_port) {
            Ok(v) => {
                self.cache.insert((node, output_port.to_string()), v.clone());
                ValueState::Set(v)
            },
            Err(_) => ValueState::Unset,
        }
    }
}