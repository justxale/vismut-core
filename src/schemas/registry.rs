use crate::schemas::NodeSchema;
use std::collections::hash_map::Iter;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct RegistrySchema {
    nodes: Vec<NodeSchema>,
    total: u32,
}

impl RegistrySchema {
    pub fn new(nodes: Iter<String, NodeSchema>) -> Self {
        let mut schema = RegistrySchema {
            nodes: vec![],
            total: 0,
        };
        for (_, node) in nodes.into_iter() {
            schema.nodes.push(node.clone());
            schema.total += 1;
        }
        log::debug!("Prepared RegistrySchema with {} nodes", schema.total);
        schema
    }
}
