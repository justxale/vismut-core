use crate::CompiledNode;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub enum EdgeType {
    Execution(String),
    Data { from_port: String, to_port: String },
}

pub struct Node<C: Clone> {
    pub name: String,
    pub node: CompiledNode<C>,
}

impl<C: Clone> Debug for Node<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("name", &self.name).finish()
    }
}
