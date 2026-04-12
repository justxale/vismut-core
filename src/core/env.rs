use crate::schemas::ScriptSchema;
use crate::schemas::RegistrySchema;
use crate::schemas::NodeSchema;
use crate::core::script::VismutScript;
use crate::common::RegistryError;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use crate::common::BoxedNodeFn;

pub struct VismutExecutionEnvironment {
    node_fns: HashMap<String, BoxedNodeFn>,
    node_schemas: HashMap<String, NodeSchema>,
    cached_schema: Option<RegistrySchema>,
}

impl VismutExecutionEnvironment {
    pub fn new() -> Self {
        log::debug!("Using new executor");
        Self {
            node_fns: HashMap::new(),
            node_schemas: HashMap::new(),
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
        schema: NodeSchema,
        node_factory: BoxedNodeFn,
    ) -> Result<&Self, RegistryError> {
        if self.node_fns.contains_key(schema.get_id()) {
            return Err(RegistryError::AlreadyRegistered);
        }
        log::debug!("Registered {}", schema.get_id());
        self.node_fns.insert(schema.get_id().to_string(), node_factory);
        self.node_schemas.insert(schema.get_id().to_string(), schema);
        Ok(self)
    }

    pub fn include(
        &mut self,
        node_array: Vec<(NodeSchema, BoxedNodeFn)>,
    ) -> Result<&Self, RegistryError> {
        for (schema, boxed_fn) in node_array {
            if self.register(schema, boxed_fn).is_err() {
                return Err(RegistryError::Failed);
            }
        }
        log::info!("Included {} nodes", self.node_fns.len());
        Ok(self)
    }

    pub fn get_node_factory(&self, node_id: &String, ) -> Result<&BoxedNodeFn, RegistryError> {
        self.node_fns.get(node_id).ok_or(RegistryError::NotFound(String::from(node_id)))
    }

    pub fn get_node_schema(&self, node_id: &String) -> Result<&NodeSchema, RegistryError> {
        self.node_schemas.get(node_id).ok_or(RegistryError::NotFound(String::from(node_id)))

    }

    pub fn parse(&self, schema: &ScriptSchema) -> Result<VismutScript, RegistryError> {
        let mut script = VismutScript::new();
        let mut node_indexes: HashMap<String, NodeIndex> = HashMap::new();
        match self.get_node_factory(&schema.entry.node_id) {
            Err(e) => return Err(e),
            Ok(node_fn) => {
                node_indexes.insert(
                    schema.entry.id.clone(),
                    script.set_entry(
                        &schema.entry.id,
                        node_fn(),
                    ),
                );
            }
        }

        for node in &schema.nodes {
            match self.get_node_factory(&node.node_id) {
                Ok(node_fn) => {
                    let mut new_node = node_fn();
                    let schema = self.get_node_schema(&node.node_id)?;
                    log::debug!("Created new node {}", schema.get_id());
                    match node.defaults {
                        Some(ref defaults) => {
                            new_node.set_values(&defaults);
                            log::debug!("Set defaults {:?} for node {}", defaults, schema.get_id());
                        }
                        None => {log::debug!("No defaults found for node {}", schema.get_id());}
                    }
                    let idx = script.add_node(&node.id, new_node);
                    node_indexes.insert(node.id.clone(), idx);
                },

                Err(e) => log::error!("{:?}", e)
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
                script.connect_data(*from, *to, path.from_port.clone(), path.to_port.clone());
            }
        }
        log::info!("Succesfully parsed script");
        Ok(script)
    }

    pub fn get_schema_mut(&mut self) -> &RegistrySchema {
        match self.cached_schema {
            Some(ref schema) => schema,
            None => {
                self.cached_schema = Some(RegistrySchema::new(self.node_schemas.iter()));
                self.cached_schema.as_ref().unwrap()
            }
        }
    }

    pub fn get_schema(&self) -> &Option<RegistrySchema> {
        &self.cached_schema
    }
}

