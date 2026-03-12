use crate::core::Port;
use crate::traits::NodeBehavior;
use serde::Serialize;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Debug)]
pub enum RegisterError {
    AlreadyRegistered,
    Failed,
}

#[derive(Serialize)]
pub struct RegisterSchema {
    nodes: Vec<NodeSchema>,
    total: u32,
}

impl RegisterSchema {
    fn new(nodes: Iter<String, (NodeSchema, fn() -> Box<dyn NodeBehavior>)>) -> Self {
        let mut schema = RegisterSchema {
            nodes: vec![],
            total: 0,
        };
        for (_, (node, _)) in nodes.into_iter() {
            schema.nodes.push(node.clone());
            schema.total += 1;
        }
        schema
    }
}

pub struct ExecutionEnvironment {
    nodes: HashMap<String, (NodeSchema, fn() -> Box<dyn NodeBehavior>)>,
    cached_schema: Option<RegisterSchema>,
}

impl ExecutionEnvironment {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            cached_schema: None,
        }
    }

    pub fn register(
        &mut self,
        node_factory: fn() -> Box<dyn NodeBehavior>,
    ) -> Result<&mut Self, RegisterError> {
        let schema = node_factory().get_schema();
        if self.nodes.contains_key(&schema.get_id()) {
            return Err(RegisterError::AlreadyRegistered);
        }
        self.nodes.insert(schema.get_id(), (schema, node_factory));
        Ok(self)
    }

    pub fn include(
        &mut self,
        node_factories: &[fn() -> Box<dyn NodeBehavior>],
    ) -> Result<&mut Self, RegisterError> {
        for factory in node_factories {
            if self.register(*factory).is_err() {
                return Err(RegisterError::Failed);
            }
        }
        Ok(self)
    }

    pub fn get_schema(&mut self) -> &RegisterSchema {
        match self.cached_schema {
            Some(ref schema) => schema,
            None => {
                self.cached_schema = Some(RegisterSchema::new(self.nodes.iter()));
                self.cached_schema.as_ref().unwrap()
            }
        }
    }
}

#[derive(Serialize, Clone)]
pub struct NodeSchema {
    node_id: String,
    is_executable: bool,
    is_evaluable: bool,
    outputs: Vec<Port>,
    inputs: Vec<Port>,
}

impl NodeSchema {
    pub fn new(
        node_id: String,
        is_executable: bool,
        is_evaluable: bool,
        inputs: Vec<Port>,
        outputs: Vec<Port>,
    ) -> Self {
        Self {
            node_id,
            is_executable,
            is_evaluable,
            inputs,
            outputs,
        }
    }

    pub fn get_id(&self) -> String {
        self.node_id.clone()
    }

    pub fn is_executable(&self) -> bool {
        self.is_executable
    }
}
