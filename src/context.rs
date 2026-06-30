use std::any::{Any, TypeId};
use crate::CompiledPort;
use crate::graph::{EdgeType, Node};
use crate::values::{Value, ValueState};
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

pub struct RuntimeContext {
    cache: HashMap<(NodeIndex, String), Value>,
    ctx: HashMap<TypeId, Box<dyn Any>>
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            ctx: HashMap::new()
        }
    }
    
    pub fn provide<T: Any>(&mut self, value: T) {
        self.ctx.insert(TypeId::of::<T>(), Box::new(value));
    }
    
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.ctx.get(&TypeId::of::<T>())?.downcast_ref()
    }
    
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.ctx.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }
    
    pub fn get_input(
        &mut self,
        node: NodeIndex,
        graph: &StableDiGraph<Node, EdgeType>,
        port: &CompiledPort,
    ) -> ValueState {
        log::debug!("Getting inputs for {}", node.index());
        for edge in graph.edges_directed(node, Direction::Incoming) {
            if let EdgeType::Data { from_port, to_port } = edge.weight()
                && *to_port == port.title()
            {
                return self.evaluate(edge.source(), graph, from_port);
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
