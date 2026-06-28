use crate::common::{ArcedEvaluableFn, ArcedExecutableFn, BoxedNodeFn};
use crate::core::NodeValues;
use crate::schemas::{NodeSchema, PortSchema};
use crate::{CompiledNode, CompiledPort, ScriptError, Value, ValueType};
use std::collections::HashMap;
use std::sync::Arc;

pub type BuiltNode = (NodeSchema, BoxedNodeFn);

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

pub struct NodeBuilder {
    execute_fn: Option<ArcedExecutableFn>,
    evaluate_fn: Option<ArcedEvaluableFn>,
    input_ports: Vec<PortBuilder>,
    output_ports: Vec<PortBuilder>,
    exec_input_ports: Option<Vec<PortSchema>>,
    exec_output_ports: Option<Vec<PortSchema>>,
    node_id: &'static str,
}

impl NodeBuilder {
    pub fn new(node_id: &'static str) -> Self {
        Self {
            node_id,
            execute_fn: None,
            evaluate_fn: None,
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            exec_input_ports: Some(Vec::new()),
            exec_output_ports: Some(Vec::new()),
        }
    }

    pub fn with_execution<F>(mut self, execute_fn: F) -> Self
    where
        F: Fn(&NodeValues) -> Result<Option<&'static str>, ScriptError> + Send + Sync + 'static,
    {
        self.execute_fn = Some(Arc::new(execute_fn));
        self
    }

    pub fn with_evaluation<F>(mut self, evaluate_fn: F) -> Self
    where
        F: Fn(&NodeValues, &String) -> Result<Value, ScriptError> + Send + Sync + 'static,
    {
        self.evaluate_fn = Some(Arc::new(evaluate_fn));
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

    pub fn with_exec_input(mut self, title: &'static str) -> Self {
        if self.exec_input_ports.is_none() {
            self.exec_input_ports = Some(Vec::new());
        }
        self.exec_input_ports
            .as_mut()
            .unwrap()
            .push(PortSchema::execution(title));
        self
    }

    pub fn with_no_exec_input(mut self) -> Self {
        self.exec_input_ports = None;
        self
    }

    pub fn with_output(mut self, title: &'static str, returned_types: &[ValueType]) -> Self {
        self.input_ports
            .push(PortBuilder::new(title).with_types(returned_types));
        self
    }

    pub fn with_exec_output(mut self, title: &'static str) -> Self {
        if self.exec_output_ports.is_none() {
            self.exec_output_ports = Some(Vec::new());
        }
        self.exec_output_ports
            .as_mut()
            .unwrap()
            .push(PortSchema::execution(title));
        self
    }

    pub fn with_no_exec_output(mut self) -> Self {
        self.exec_output_ports = None;
        self
    }

    fn schema(&self) -> NodeSchema {
        let is_executable = self.execute_fn.is_some();
        let is_evaluable = self.evaluate_fn.is_some();
        let mut inputs: Vec<PortSchema> = self.input_ports.iter().map(PortSchema::from).collect();
        let mut outputs: Vec<PortSchema> = self.input_ports.iter().map(PortSchema::from).collect();

        if let Some(_) = self.execute_fn
            && let Some(ref v) = self.exec_input_ports
        {
            v.is_empty()
                .then(|| inputs.push(PortSchema::execution("exec")));
        }

        if let Some(_) = self.execute_fn
            && let Some(ref v) = self.exec_output_ports
        {
            v.is_empty()
                .then(|| outputs.push(PortSchema::execution("exec")));
        }

        NodeSchema::new(self.node_id, is_executable, is_evaluable, inputs, outputs)
    }

    pub fn build(self) -> BuiltNode {
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
                        .unwrap_or(Arc::new(|_| Err(ScriptError::NotExecutable))),
                    self.evaluate_fn
                        .clone()
                        .unwrap_or(Arc::new(|_, _| Err(ScriptError::NotEvaluable))),
                    HashMap::from_iter(inputs.iter().cloned()),
                    HashMap::from_iter(outputs.iter().cloned()),
                )
            }),
        )
    }
}
