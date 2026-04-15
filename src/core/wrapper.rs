use std::sync::Arc;
use std::collections::HashMap;
use petgraph::stable_graph::NodeIndex;
use crate::graph::{Node, EdgeType};
use petgraph::stable_graph::StableDiGraph;
use crate::values::{ValueType, ValueState};
use crate::context::ExecutionContext;
use crate::common::{BoxedEvaluableFn, BoxedExecutableFn, BoxedNodeFn, ScriptError};
use crate::schemas::{NodeSchema, PortSchema};
use crate::values::Value;

pub struct NodeValues<'a> {
    ports: &'a HashMap<&'static str, CompiledPort>,
    map: HashMap<String, Value>
}

impl<'a> NodeValues<'a> {
    pub fn new(ports: &'a HashMap<&'static str, CompiledPort>) -> Self {
        Self {
            map: HashMap::new(),
            ports
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Value {
        match self.map.get(key) {
            Some(value) => value.clone(),
            None => {
                if let Some(port) = self.ports.get(key) {
                    return port.types[0].default_value().unwrap()
                }
                panic!("Unknown port: {}", key)
            },
        }
    }
}


#[derive(Clone)]
pub struct CompiledPort {
    title: &'static str,
    types: Vec<ValueType>,
    value: Option<Value>
}

impl CompiledPort {
    pub fn new(title: &'static str, types: Vec<ValueType>) -> CompiledPort {
        Self { title, types, value: None }
    }

    pub fn validate(&self, value: &Value) -> bool {
        self.types.contains(&value.value_type()) || self.types.contains(&ValueType::Any)
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn types(&self) -> &[ValueType] {
        &self.types
    }

    pub fn set_value(&mut self, value: Option<Value>) {
        self.value = value;
    }
}

pub struct CompiledNode {
    execute_fn: BoxedExecutableFn,
    evaluate_fn: BoxedEvaluableFn,
    inputs: HashMap<&'static str, CompiledPort>,
    outputs: HashMap<&'static str, CompiledPort>,
}

impl CompiledNode {
    pub fn new(
        execute_fn: BoxedExecutableFn,
        evaluate_fn: BoxedEvaluableFn,
        inputs: HashMap<&'static str, CompiledPort>,
        outputs: HashMap<&'static str, CompiledPort>
    ) -> Self {
        Self {
            evaluate_fn, execute_fn, inputs, outputs
        }
    }

    fn get_values(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<NodeValues, ScriptError> {
        let mut values = NodeValues::new(&self.inputs);
        for (port_title, port) in &self.inputs {
            match ctx.get_input(node, graph, port) {
                ValueState::Set(ref value) => {
                    if !port.validate(value) {
                        return Err(ScriptError::UnsupportedInput)
                    }
                    values.insert(String::from(*port_title), value.clone());
                },
                ValueState::Default => {
                    if let Some(default_value) = &port.value {
                        values.insert(String::from(*port_title), default_value.clone());
                    } else {
                        match port.types[0].default_value() {
                            Ok(value) => values.insert(String::from(*port_title), value),
                            Err(error) => return Err(error)
                        }
                    }
                },
                ValueState::Unset => return Err(ScriptError::MissingInput(format!("{}", port_title)))
            }
        }
        Ok(values)
    }

    pub fn execute(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<(), ScriptError> {
        match self.get_values(ctx, graph, node) {
            Ok(values) =>  (self.execute_fn)(&values),
            Err(error) => Err(error)
        }
    }

    pub fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        output_port: &String,
    ) -> Result<Value, ScriptError> {
        match self.get_values(ctx, graph, node) {
            Ok(values) =>  (self.evaluate_fn)(&values, &output_port.clone()),
            Err(error) => Err(error)
        }
    }

    pub fn set_values(&mut self, values: &HashMap<String, Option<Value>>) -> Option<()> {
        for (title, value) in values {
            self.inputs.get_mut(title.as_str()).map(|port| port.set_value(value.clone()));
        }
        None
    }
}

pub(crate) struct PortBuilder {
    title: &'static str,
    accepted_types: Vec<ValueType>
}

impl PortBuilder {
    pub fn new(title: &'static str) -> Self {
        Self {
            title, accepted_types: Vec::new()
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
    execute_fn: Option<BoxedExecutableFn>,
    evaluate_fn: Option<BoxedEvaluableFn>,
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
        F: Fn(&NodeValues) -> Result<(), ScriptError> + 'static
    {
        self.execute_fn = Some(Arc::new(execute_fn));
        self
    }

    pub fn with_evaluation<F>(mut self, evaluate_fn: F) -> Self
    where
        F: Fn(&NodeValues, &String) -> Result<Value, ScriptError> + 'static
    {
        self.evaluate_fn = Some(Arc::new(evaluate_fn));
        self
    }

    pub fn with_input(mut self, title: &'static str, accepted_types: &[ValueType]) -> Self {
        if accepted_types.is_empty() {
            panic!("There must be at least one ValueType specified")
        }
        self.input_ports.push(PortBuilder::new(title).with_types(accepted_types));
        self
    }

    pub fn with_exec_input(mut self, title: &'static str) -> Self {
        if self.exec_input_ports.is_none() {
            self.exec_input_ports = Some(Vec::new());
        }
        self.exec_input_ports.as_mut().unwrap().push(PortSchema::execution(title));
        self
    }

    pub fn with_no_exec_input(mut self) -> Self {
        self.exec_input_ports = None;
        self
    }

    pub fn with_output(mut self, title: &'static str, returned_types: &[ValueType]) -> Self {
        self.input_ports.push(PortBuilder::new(title).with_types(returned_types));
        self
    }

    pub fn with_exec_output(mut self, title: &'static str) -> Self {
        if self.exec_output_ports.is_none() {
            self.exec_output_ports = Some(Vec::new());
        }
        self.exec_output_ports.as_mut().unwrap().push(PortSchema::execution(title));
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

        if let Some(_) = self.execute_fn && let Some(ref v) = self.exec_input_ports {
            v.is_empty().then(|| inputs.push(PortSchema::execution("exec")));
        }

        if let Some(_) = self.execute_fn && let Some(ref v) = self.exec_output_ports {
            v.is_empty().then(|| outputs.push(PortSchema::execution("exec")));
        }

        NodeSchema::new(
            self.node_id, is_executable, is_evaluable,
            inputs, outputs
        )
    }

    pub fn build(self) -> (NodeSchema, BoxedNodeFn) {
        let schema = self.schema();
        let inputs: Vec<(&'static str, CompiledPort)> = self.input_ports.into_iter().map(|port| {
            (port.title, port.build())
        }).collect();
        let outputs: Vec<(&'static str, CompiledPort)> = self.output_ports.into_iter().map(|port| {
            (port.title, port.build())
        }).collect();
  
        (schema, Box::new(move || {
            CompiledNode::new(
                self.execute_fn.clone().unwrap_or(
                    Arc::new(|_| Err(ScriptError::NotExecutable))
                ),
                self.evaluate_fn.clone().unwrap_or(
                    Arc::new(|_, _| Err(ScriptError::NotEvaluable))
                ),
                HashMap::from_iter(inputs.iter().cloned()),
                HashMap::from_iter(outputs.iter().cloned()),
            )
        }))
    }
}
