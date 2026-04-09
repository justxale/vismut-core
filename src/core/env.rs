use crate::schemas::ScriptSchema;
use crate::schemas::RegistrySchema;
use crate::schemas::NodeSchema;
use crate::core::script::VismutScript;
use crate::common::RegistryError;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use crate::common::BoxedNodeFn;

pub struct VismutExecutionEnvironment {
    nodes: HashMap<String, (NodeSchema, BoxedNodeFn)>,
    cached_schema: Option<RegistrySchema>,
}

impl VismutExecutionEnvironment {
    pub fn new() -> Self {
        log::debug!("Using new executor");
        Self {
            nodes: HashMap::new(),
            cached_schema: None,
        }
    }

    /*#[cfg(feature = "nodes")]
    pub fn default() -> Self {
        log::debug!("Using default executor");
        let mut registry = Self {
            nodes: HashMap::new(),
            cached_schema: None,
        };
        registry
            .include(&MATH_NODES_FACTORIES).unwrap()
            .include(&IO_NODES_FACTORIES).unwrap()
            .include(&RANDOM_NODE_FACTORIES).unwrap();
        log::info!("Default executor ready; loaded {} nodes", registry.nodes.len());
        registry
    }*/

    pub fn register(
        &mut self,
        schema: &NodeSchema,
        node_factory: &BoxedNodeFn,
    ) -> Result<&Self, RegistryError> {
        if self.nodes.contains_key(schema.get_id()) {
            return Err(RegistryError::AlreadyRegistered);
        }
        log::debug!("Registered {}", schema.get_id());
        self.nodes
            .insert(schema.get_id().to_string(), (schema.clone(), node_factory.clone()));
        Ok(self)
    }

    pub fn include(
        &mut self,
        node_array: &[(NodeSchema, BoxedNodeFn)],
    ) -> Result<&Self, RegistryError> {
        for (schema, boxed_fn) in node_array {
            if self.register(schema, boxed_fn).is_err() {
                return Err(RegistryError::Failed);
            }
        }
        log::info!("Included {} nodes", node_array.len());
        Ok(self)
    }

    pub fn get_node(
        &self,
        node_id: &String,
    ) -> Option<&(NodeSchema, BoxedNodeFn)> {
        self.nodes.get(node_id)
    }

    pub fn parse(&self, schema: &ScriptSchema) -> Result<VismutScript, RegistryError> {
        let mut script = VismutScript::new();
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
            if let Some((schema, behavior)) = self.get_node(&node.node_id) {
                let mut new_node = behavior();
                log::debug!("Created new node {}", schema.get_id());
                match node.defaults {
                    Some(ref defaults) => {
                        new_node.set_values(defaults);
                        log::debug!("Set defaults {:?} for node {}", defaults, schema.get_id());
                    }
                    None => {log::debug!("No defaults found for node {}", schema.get_id());}
                }
                let idx = script.add_node(&node.id, new_node);
                node_indexes.insert(node.id.clone(), idx);
            } else {
                log::error!("Node {} not found", node.node_id);
                return Err(RegistryError::NotFound(schema.entry.node_id.clone()));
            }
        }
        for path in &schema.exec_paths {
            if let (Some(from), Some(to)) =
                (node_indexes.get(&path.from), node_indexes.get(&path.to))
            {
                log::debug!("Execution connected; {} to {}", from.index(), to.index());
                script.connect_execution(*from, *to);
            }

        }
        for path in &schema.data_paths {
            if let (Some(from), Some(to)) =
                (node_indexes.get(&path.from), node_indexes.get(&path.to))
            {
                log::debug!("Data connected; {}:{} to {}:{}", from.index(), to.index(), &path.from_port, &path.to_port);
                script.connect_data(*from, *to, &path.from_port, &path.to_port);
            }
        }
        log::info!("Succesfully parsed script");
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

