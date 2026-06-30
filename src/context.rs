use crate::CompiledPort;
use crate::graph::{EdgeType, Node};
use crate::values::{Value, ValueState};
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

pub struct RuntimeCache {
    cache: HashMap<(NodeIndex, String), Value>,
}

impl RuntimeCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    pub fn get_input<C: Clone>(
        &mut self,
        node: NodeIndex,
        graph: &StableDiGraph<Node<C>, EdgeType>,
        port: &CompiledPort,
        ctx: &C
    ) -> ValueState {
        log::debug!("Getting inputs for {}", node.index());
        for edge in graph.edges_directed(node, Direction::Incoming) {
            if let EdgeType::Data { from_port, to_port } = edge.weight()
                && *to_port == port.title()
            {
                return self.evaluate(edge.source(), graph, from_port, &ctx);
            }
        }
        for t in port.types() {
            if t.is_default_supported() {
                return ValueState::Default;
            }
        }

        ValueState::Unset
    }

    pub fn evaluate<C: Clone>(
        &mut self,
        node: NodeIndex,
        graph: &StableDiGraph<Node<C>, EdgeType>,
        output_port: &String,
        ctx: &C
    ) -> ValueState {
        if let Some(v) = self.cache.get(&(node, output_port.to_string())) {
            return ValueState::Set(v.clone());
        }
        let behavior = &graph[node].node;
        match behavior.evaluate(self, graph, node, output_port, ctx.clone()) {
            Ok(v) => {
                self.cache
                    .insert((node, output_port.to_string()), v.clone());
                ValueState::Set(v)
            }
            Err(_) => ValueState::Unset,
        }
    }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
