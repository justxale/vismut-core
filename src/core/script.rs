use crate::context::ExecutionContext;
use crate::graph::{Node, EdgeType};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use crate::{ScriptError, CompiledNode};

pub struct VismutScript {
    graph: StableDiGraph<Node, EdgeType>,
    entry: Option<NodeIndex>,
}

impl VismutScript {
    pub fn new() -> Self {
        Self {
            graph: StableDiGraph::new(),
            entry: None,
        }
    }

    pub fn set_entry(&mut self, name: &String, behavior: CompiledNode) -> NodeIndex {
        let idx = self.add_node(name, behavior);
        self.entry = Some(idx);
        idx
    }

    pub fn add_node(&mut self, name: &String, behavior: CompiledNode) -> NodeIndex {
        let idx = self.graph.add_node(Node {
            name: name.to_string(),
            behavior,
        });

        idx
    }

    pub fn connect_execution(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, EdgeType::Execution);
    }

    pub fn connect_data(&mut self, from: NodeIndex, to: NodeIndex, from_port: &str, to_port: &str) {
        self.graph.add_edge(
            from,
            to,
            EdgeType::Data {
                from_port: from_port.to_string(),
                to_port: to_port.to_string(),
            },
        );
    }

    pub fn run(&mut self) -> Result<u32, ScriptError> {
        let mut current = self.entry.ok_or_else(|| panic!("No entry"))?;
        let mut ctx = ExecutionContext {
            cache: HashMap::new(),
        };

        loop {
            let node = self.graph.node_weight(current);
            match node {
                Some(node) => {
                    if node.behavior.get_schema().is_executable() {
                        node.behavior.execute(&mut ctx, &self.graph, current)?;
                    }
                }
                _ => panic!(),
            }
            ctx.cache.clear();

            let next_exec = self
                .graph
                .edges_directed(current, Direction::Outgoing)
                .find(|e| matches!(e.weight(), EdgeType::Execution))
                .map(|e| e.target());
            match next_exec {
                Some(next) => current = next,
                None => break,
            }
        }

        Ok(0)
    }
}