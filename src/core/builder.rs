use crate::common::{ArcedEvaluableFn, ArcedExecutableFn, BoxedNodeFn};
use crate::core::NodeValues;
use crate::schemas::{NodeSchema, PortSchema};
use crate::{CompiledNode, CompiledPort, ScriptError, Value, ValueType};
use std::collections::HashMap;
use std::sync::Arc;

pub type BuiltNode<C> = (NodeSchema, BoxedNodeFn<C>);

pub struct PortBuilder {
    title: &'static str,
    accepted_types: Vec<ValueType>,
}

impl PortBuilder {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            accepted_types: Vec::new(),
        }
    }

    pub fn with_types(mut self, types: &[ValueType]) -> Self {
        self.accepted_types.extend_from_slice(types);
        self
    }

    pub fn build(self) -> CompiledPort {
        CompiledPort::new(self.title, self.accepted_types)
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn types(&self) -> &Vec<ValueType> {
        &self.accepted_types
    }
}

pub struct NodeBuilder<C: Clone = ()> {
    execute_fn: Option<ArcedExecutableFn<C>>,
    evaluate_fn: Option<ArcedEvaluableFn<C>>,
    input_ports: Vec<PortBuilder>,
    output_ports: Vec<PortBuilder>,
    exec_input_port: Option<PortSchema>,
    exec_output_ports: Option<Vec<PortSchema>>,
    node_id: &'static str,
    is_raw: bool
}

impl<C: Clone + 'static> NodeBuilder<C> {
    pub fn new(node_id: &'static str) -> Self {
        Self {
            node_id,
            execute_fn: None,
            evaluate_fn: None,
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            exec_input_port: None,
            exec_output_ports: Some(Vec::new()),
            is_raw: false
        }
    }

    pub fn raw<F>(node_id: &'static str, execute_fn: F, output_title: &'static str) -> Self<>
    where
        F: Fn(&NodeValues, C) -> Result<Option<&'static str>, ScriptError> + Send + Sync + 'static
    {
        let mut builder = Self {
            node_id,
            execute_fn: None,
            evaluate_fn: None,
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            exec_input_port: None,
            exec_output_ports: Some(Vec::new()),
            is_raw: true
        };
        builder.execute_fn = Some(Arc::new(execute_fn));
        builder.exec_output_ports = Some(vec![PortSchema::execution(output_title)]);
        builder
    }

    pub fn with_evaluation<F>(
        mut self, evaluate_fn: F,
        outputs: &[(&'static str, &[ValueType])]
    ) -> Self
    where
        F: Fn(&NodeValues, &String, C) -> Result<Value, ScriptError> + Send + Sync + 'static,
    {
        self.output_ports.extend(outputs.iter().map(|(title, types)| PortBuilder::new(title).with_types(types)));
        self.evaluate_fn = Some(Arc::new(evaluate_fn));
        self
    }

    pub fn with_execution<F>(mut self, execute_fn: F, input_title: &'static str, outputs: Option<&[&'static str]>) -> Self
    where
        F: Fn(&NodeValues, C) -> Result<Option<&'static str>, ScriptError> + Send + Sync + 'static,
    {
        if self.is_raw {
            panic!("Cannot add execution workflow to node, that marked as raw")
        }
        self.execute_fn = Some(Arc::new(execute_fn));
        self.exec_input_port = Some(PortSchema::execution(input_title));
        self.exec_output_ports = outputs.map(|v| v.iter().map(|v| PortSchema::execution(v)).collect());
        self
    }

    pub fn with_input(mut self, title: &'static str, accepted_types: &[ValueType]) -> Self {
        if accepted_types.is_empty() {
            panic!("There must be at least one ValueType specified")
        }
        self.input_ports
            .push(PortBuilder::new(title).with_types(accepted_types));
        self
    }

    fn schema(&self) -> NodeSchema {
        let is_executable = self.execute_fn.is_some();
        let is_evaluable = self.evaluate_fn.is_some();
        let mut inputs: Vec<PortSchema> = self.input_ports.iter().map(PortSchema::from).collect();
        if let Some(exec) = &self.exec_input_port {
            inputs.push(exec.clone());
        }

        let mut outputs: Vec<PortSchema> = self.output_ports.iter().map(PortSchema::from).collect();
        if let Some(exec) = &self.exec_output_ports {
            outputs.extend_from_slice(exec)
        }

        NodeSchema::new(self.node_id, is_executable, is_evaluable, inputs, outputs)
    }

    pub fn build(self) -> BuiltNode<C> {
        let schema = self.schema();
        let inputs: Vec<(&'static str, CompiledPort)> = self
            .input_ports
            .into_iter()
            .map(|port| (port.title, port.build()))
            .collect();
        let outputs: Vec<(&'static str, CompiledPort)> = self
            .output_ports
            .into_iter()
            .map(|port| (port.title, port.build()))
            .collect();

        (
            schema,
            Box::new(move || {
                CompiledNode::new(
                    self.execute_fn
                        .clone()
                        .unwrap_or(Arc::new(|_, _| Err(ScriptError::NotExecutable))),
                    self.evaluate_fn
                        .clone()
                        .unwrap_or(Arc::new(|_, _, _| Err(ScriptError::NotEvaluable))),
                    HashMap::from_iter(inputs.iter().cloned()),
                    HashMap::from_iter(outputs.iter().cloned()),
                )
            }),
        )
    }
}