use crate::traits::NodeBehavior;
use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum ScriptError {
    MissingInput,
    UnsupportedInput,
    NotEvaluable,
    NotExecutable,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Object(HashMap<String, Value>),
    None,
}

#[derive(Serialize, Debug, Clone)]
pub enum ValueType {
    Int,
    Float,
    Bool,
    String,
    Object,
    Any,
    None,
}

#[derive(Debug, Clone, Serialize)]
pub enum PortKind {
    Execution,
    Data,
}

#[derive(Debug, Clone, Serialize)]
pub struct Port {
    pub name: String,
    pub kind: PortKind,
    pub types: Vec<ValueType>
}

#[derive(Debug, Clone)]
pub enum EdgeType {
    Execution,
    Data { from_port: String, to_port: String },
}

pub struct Node {
    pub name: String,
    pub behavior: Box<dyn NodeBehavior>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = f.debug_struct("Node").field("name", &self.name).finish();
        Ok(())
    }
}

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

                    return self.evaluate(source, graph, from_port);
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

pub struct VisualScript {
    graph: StableDiGraph<Node, EdgeType>,
    entry: Option<NodeIndex>,
}

impl VisualScript {
    pub fn new() -> Self {
        Self {
            graph: StableDiGraph::new(),
            entry: None,
        }
    }

    pub fn add_node(&mut self, name: &str, behavior: Box<dyn NodeBehavior>) -> NodeIndex {
        let idx = self.graph.add_node(Node {
            name: name.to_string(),
            behavior,
        });
        if self.entry.is_none() {
            self.entry = Some(idx);
        }

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
