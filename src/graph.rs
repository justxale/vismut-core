use std::fmt::{Debug, Formatter};
use crate::CompiledNode;

#[derive(Debug, Clone)]
pub enum EdgeType {
    Execution,
    Data { from_port: String, to_port: String },
}

pub struct Node {
    pub name: String,
    pub behavior: CompiledNode,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("name", &self.name).finish()
    }
}