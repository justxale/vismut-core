use crate::common::BoxedNodeFn;
use crate::common::RegistryError;
use crate::core::script::VismutScript;
#[cfg(feature = "nodes")]
use crate::nodes::{build_io_nodes, build_math_nodes};
use crate::schemas::NodeSchema;
use crate::schemas::RegistrySchema;
use crate::schemas::ScriptSchema;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct VismutRuntime<C: Clone = ()> {
    node_fns: HashMap<String, BoxedNodeFn<C>>,
    node_schemas: HashMap<String, NodeSchema>,
    cached_schema: Option<RegistrySchema>,
    ctx: C
}

impl<C: Clone + 'static> VismutRuntime<C> {
    pub fn new(ctx: C) -> Self {
        Self {
            node_fns: HashMap::new(),
            node_schemas: HashMap::new(),
            cached_schema: None,
            ctx
        }
    }

    #[cfg(feature = "nodes")]
    pub fn with_builtins(mut self) -> Self {
        log::debug!("Using default executor");
        self.include(build_math_nodes()).unwrap();
        self.include(build_io_nodes()).unwrap();
        //.include(&RANDOM_NODE_FACTORIES).unwrap();
        log::info!(
            "Default executor ready; loaded {} nodes",
            self.node_fns.len()
        );
        self
    }

    pub fn register(
        &mut self,
        schema: NodeSchema,
        node_factory: BoxedNodeFn<C>,
    ) -> Result<&Self, RegistryError> {
        if self.node_fns.contains_key(schema.get_id()) {
            return Err(RegistryError::AlreadyRegistered);
        }
        log::debug!("Registered {}", schema.get_id());
        self.node_fns
            .insert(schema.get_id().to_string(), node_factory);
        self.node_schemas
            .insert(schema.get_id().to_string(), schema);
        Ok(self)
    }

    pub fn include(
        &mut self,
        node_array: Vec<(NodeSchema, BoxedNodeFn<C>)>,
    ) -> Result<&Self, RegistryError> {
        let len = node_array.len();
        for (schema, boxed_fn) in node_array {
            if self.register(schema, boxed_fn).is_err() {
                return Err(RegistryError::Failed);
            }
        }
        log::info!("Included {} nodes", len);
        Ok(self)
    }

    pub fn get_node_factory(&self, node_id: &String) -> Result<&BoxedNodeFn<C>, RegistryError> {
        self.node_fns
            .get(node_id)
            .ok_or(RegistryError::NotFound(String::from(node_id)))
    }

    pub fn get_node_schema(&self, node_id: &String) -> Result<&NodeSchema, RegistryError> {
        self.node_schemas
            .get(node_id)
            .ok_or(RegistryError::NotFound(String::from(node_id)))
    }

    pub fn parse(&self, schema: &ScriptSchema) -> Result<VismutScript<C>, RegistryError> {
        let mut script = VismutScript::new(self.ctx.clone());
        let mut node_indexes: HashMap<String, NodeIndex> = HashMap::new();
        match self.get_node_factory(&schema.entry.node_id) {
            Err(e) => return Err(e),
            Ok(node_fn) => {
                node_indexes.insert(
                    schema.entry.id.clone(),
                    script.set_entry(&schema.entry.id, node_fn()),
                );
            }
        }

        for node in &schema.nodes {
            match self.get_node_factory(&node.node_id) {
                Ok(node_fn) => {
                    let mut new_node = node_fn();
                    let schema = self.get_node_schema(&node.node_id)?;
                    log::debug!("Created new node {}", schema.get_id());
                    match &node.defaults {
                        Some(defaults) => {
                            new_node.set_values(defaults);
                            log::debug!("Set defaults {:?} for node {}", defaults, schema.get_id());
                        }
                        None => {
                            log::debug!("No defaults found for node {}", schema.get_id());
                        }
                    }
                    let idx = script.add_node(&node.id, new_node);
                    node_indexes.insert(node.id.clone(), idx);
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    return Err(e);
                }
            }
        }

        for path in &schema.exec_paths {
            if let (Some(from), Some(to)) =
                (node_indexes.get(&path.from), node_indexes.get(&path.to))
            {
                script.connect_execution(*from, *to, path.from_port.clone());
                log::debug!("Execution connected; {} to {}", from.index(), to.index());
            }
        }

        for path in &schema.data_paths {
            if let (Some(from), Some(to)) =
                (node_indexes.get(&path.from), node_indexes.get(&path.to))
            {
                script.connect_data(*from, *to, path.from_port.clone(), path.to_port.clone());
                log::debug!(
                    "Data connected; {}:{} to {}:{}",
                    from.index(),
                    to.index(),
                    &path.from_port,
                    &path.to_port
                );
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

impl<C: Clone + Default + 'static> Default for VismutRuntime<C> {
    fn default() -> Self {
        Self::new(C::default())
    }
}

impl Debug for VismutRuntime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VismutExecutionEnvironment")
            .field("schema", &self.get_schema())
            .field("nodes", &self.node_fns.len())
            .finish()
    }
}
