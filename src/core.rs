use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
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
    None,
}

#[derive(Debug, Clone)]
pub enum PortKind {
    Execution,
    Data,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub name: String,
    pub kind: PortKind,
    pub constant: Option<Value>,
}

pub trait NodeBehavior {
    fn execute(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex
    ) -> Result<(), ScriptError>;

    fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        output_port: &str
    ) -> Result<Value, ScriptError>;

    fn input_ports(&self) -> Vec<Port>;

    fn output_ports(&self) -> Vec<Port>;

    fn is_pure(&self) -> bool;
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
        let _ = f.debug_struct("Node")
            .field("name", &self.name)
            .finish();
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

        Ok(Value::None)
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

        self.cache.insert((node, output_port.to_string()), value.clone());

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
            entry: None
        }
    }

    pub fn add_node(
        &mut self,
        name: &str,
        behavior: Box<dyn NodeBehavior>,
    ) -> NodeIndex {
        let idx = self.graph.add_node(Node {
            name: name.to_string(),
            behavior,
        });

        if self.entry.is_none() {
            self.entry = Some(idx);
        }

        idx
    }

    pub fn connect_execution(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
    ) {
        self.graph.add_edge(from, to, EdgeType::Execution);
    }

    pub fn connect_data(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        from_port: &str,
        to_port: &str,
    ) {
        self.graph.add_edge(
            from,
            to,
            EdgeType::Data {
                from_port: from_port.to_string(),
                to_port: to_port.to_string(),
            },
        );
    }

    pub fn run(&mut self) -> Result<(), ScriptError> {
        let mut current = self.entry.ok_or_else(|| panic!("No entry"))?;
        let mut ctx = ExecutionContext {
            cache: HashMap::new(),
        };

        loop {
            let node = self.graph.node_weight(current);

            match node {
                Some(node) => {
                    if !node.behavior.is_pure() {
                        node.behavior.execute(&mut ctx, &self.graph, current)?;
                    }
                }
                _ => panic!()
            }


            // clear cache per execution step (optional, like Blueprint tick)
            ctx.cache.clear();

            let next_exec = self.graph
                .edges_directed(current, Direction::Outgoing)
                .find(|e| matches!(e.weight(), EdgeType::Execution))
                .map(|e| e.target());

            match next_exec {
                Some(next) => current = next,
                None => break,
            }
        }

        Ok(())
    }
}
