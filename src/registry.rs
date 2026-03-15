use crate::core::Port;
#[cfg(feature = "nodes")]
use crate::nodes::{IO_NODES_FACTORIES, MATH_NODES_FACTORIES, RANDOM_NODE_FACTORIES};
use crate::traits::NodeBehavior;
use crate::{Value, VisualScript};
use petgraph::stable_graph::NodeIndex;
#[cfg(feature = "serde")]
use serde::Serialize;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Debug)]
pub enum RegistryError {
    AlreadyRegistered,
    Failed,
    NotFound(String),
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug)]
pub struct RegistrySchema {
    nodes: Vec<NodeSchema>,
    total: u32,
}

impl RegistrySchema {
    fn new(nodes: Iter<String, (NodeSchema, fn() -> Box<dyn NodeBehavior>)>) -> Self {
        let mut schema = RegistrySchema {
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
    cached_schema: Option<RegistrySchema>,
}

impl ExecutionEnvironment {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            cached_schema: None,
        }
    }

    #[cfg(feature = "nodes")]
    pub fn default() -> Self {
        let mut registry = Self {
            nodes: HashMap::new(),
            cached_schema: None,
        };
        registry
            .include(&MATH_NODES_FACTORIES).unwrap()
            .include(&IO_NODES_FACTORIES).unwrap()
            .include(&RANDOM_NODE_FACTORIES).unwrap();
        registry
    }

    pub fn register(
        &mut self,
        node_factory: fn() -> Box<dyn NodeBehavior>,
    ) -> Result<&mut Self, RegistryError> {
        let schema = node_factory().get_schema();
        if self.nodes.contains_key(schema.get_id()) {
            return Err(RegistryError::AlreadyRegistered);
        }
        self.nodes
            .insert(schema.get_id().to_string(), (schema, node_factory));
        Ok(self)
    }

    pub fn include(
        &mut self,
        node_factories: &[fn() -> Box<dyn NodeBehavior>],
    ) -> Result<&mut Self, RegistryError> {
        for factory in node_factories {
            if self.register(*factory).is_err() {
                return Err(RegistryError::Failed);
            }
        }
        Ok(self)
    }

    pub fn get_node(
        &self,
        node_id: &String,
    ) -> Option<&(NodeSchema, fn() -> Box<dyn NodeBehavior>)> {
        self.nodes.get(node_id)
    }

    pub fn parse(&self, schema: &ScriptSchema) -> Result<VisualScript, RegistryError> {
        let mut script = VisualScript::new();
        let mut node_indexes: HashMap<String, NodeIndex> = HashMap::new();
        if let None = self.get_node(&schema.entry.node_id) {
            return Err(RegistryError::NotFound(schema.entry.node_id.clone()));
        }
        node_indexes.insert(
            schema.entry.id.clone(),
            script.set_entry(
                &schema.entry.id,
                self.get_node(&schema.entry.node_id).unwrap().1(),
            ),
        );

        for node in &schema.nodes {
            if let Some((_, behavior)) = self.get_node(&node.node_id) {
                let mut new_node = behavior();
                match node.defaults {
                    Some(ref defaults) => {
                        new_node.set_values(defaults);
                    }
                    None => {}
                }

                let idx = script.add_node(&node.id, new_node);
                node_indexes.insert(node.id.clone(), idx);
            } else {
                return Err(RegistryError::NotFound(schema.entry.node_id.clone()));
            }
        }
        for path in &schema.exec_paths {
            if let (Some(from), Some(to)) =
                (node_indexes.get(&path.from), node_indexes.get(&path.to))
            {
                script.connect_execution(*from, *to);
            }

        }
        for path in &schema.data_paths {
            if let (Some(from), Some(to)) =
                (node_indexes.get(&path.from), node_indexes.get(&path.to))
            {
                script.connect_data(*from, *to, &path.from_port, &path.to_port);
            }

        }

        Ok(script)
    }

    pub fn get_schema_mut(&mut self) -> &RegistrySchema {
        match self.cached_schema {
            Some(ref schema) => schema,
            None => {
                self.cached_schema = Some(RegistrySchema::new(self.nodes.iter()));
                self.cached_schema.as_ref().unwrap()
            }
        }
    }

    pub fn get_schema(&self) -> &Option<RegistrySchema> {
        &self.cached_schema
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
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

    pub fn get_id(&self) -> &String {
        &self.node_id
    }

    pub fn is_executable(&self) -> bool {
        self.is_executable
    }

    pub fn is_evaluable(&self) -> bool {
        self.is_evaluable
    }

    pub fn get_outputs(&self) -> &Vec<Port> {
        &self.outputs
    }

    pub fn get_inputs(&self) -> &Vec<Port> {
        &self.inputs
    }
}

pub struct ScriptNode {
    pub node_id: String,
    pub id: String,
    pub defaults: Option<HashMap<String, Option<Value>>>,
}

pub struct ScriptExecutionPath {
    pub from: String,
    pub to: String,
}

pub struct ScriptDataPath {
    pub from: String,
    pub from_port: String,
    pub to: String,
    pub to_port: String,
}

pub struct ScriptSchema {
    pub entry: ScriptNode,
    pub nodes: Vec<ScriptNode>,
    pub exec_paths: Vec<ScriptExecutionPath>,
    pub data_paths: Vec<ScriptDataPath>,
}
