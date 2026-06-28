use crate::context::ExecutionContext;
use crate::graph::{EdgeType, Node};
use crate::{CompiledNode, ScriptError};
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::prelude::StableDiGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

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
        self.graph.add_node(Node {
            name: name.to_string(),
            node: behavior,
        })
    }

    pub fn connect_execution(&mut self, from: NodeIndex, to: NodeIndex, from_port: String) {
        self.graph.add_edge(from, to, EdgeType::Execution(from_port));
    }

    pub fn connect_data(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        from_port: String,
        to_port: String,
    ) {
        self.graph
            .add_edge(from, to, EdgeType::Data { from_port, to_port });
    }

    pub fn run(&mut self) -> Result<u16, ScriptError> {
        let mut current = self.entry.ok_or_else(|| panic!("No entry"))?;
        let mut ctx = ExecutionContext {
            cache: HashMap::new(),
        };
        let mut nodes_passed: u16 = 0;

        loop {
            nodes_passed += 1;
            let next_node = match self.graph.node_weight(current) {
                Some(graph_node) => match graph_node.node.execute(&mut ctx, &self.graph, current)? {
                    Some(node) => node,
                    None => break,
                },
                _ => return Err(ScriptError::NotExecutable),
            };
            ctx.cache.clear();

            let next_exec = self
                .graph
                .edges_directed(current, Direction::Outgoing)
                .find(|e| matches!(e.weight(), EdgeType::Execution(id) if id == next_node))
                .map(|e| e.target());
            match next_exec {
                Some(next) => current = next,
                None => break,
            }
        }

        Ok(nodes_passed)
    }
}
