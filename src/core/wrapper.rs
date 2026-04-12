use std::sync::Arc;
use std::collections::HashMap;
use petgraph::stable_graph::NodeIndex;
use crate::graph::{Node, EdgeType};
use petgraph::stable_graph::StableDiGraph;
use crate::values::{ValueType, ValueState};
use crate::context::ExecutionContext;
use crate::common::{BoxedEvaluableFn, BoxedExecutableFn, BoxedNodeFn, ScriptError};
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

    pub fn set_value(&mut self, value: Value) {
        self.value = Some(value);
    }
}

pub struct CompiledNode {
    execute_fn: BoxedExecutableFn,
    evaluate_fn: BoxedEvaluableFn,
    inputs: HashMap<&'static str, CompiledPort>,
    outputs: HashMap<&'static str, CompiledPort>,
}

impl CompiledNode {
    pub fn new(execute_fn: BoxedExecutableFn, evaluate_fn: BoxedEvaluableFn) -> Self {
        Self {
            evaluate_fn, execute_fn,
            inputs: HashMap::new(), outputs: HashMap::new()
        }
    }

    pub fn execute(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<(), ScriptError> {
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
                    match port.types[0].default_value() {
                        Err(error) => return Err(error),
                        Ok(value) => {values.insert(String::from(*port_title), value);}
                    }
                },
                ValueState::Unset => return Err(ScriptError::MissingInput)
            }
        }
        (self.execute_fn)(&values)
    }

    pub fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        output_port: &String,
    ) -> Result<Value, ScriptError> {
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
                    match port.types[0].default_value() {
                        Err(error) => return Err(error),
                        Ok(value) => {values.insert(String::from(*port_title), value);}
                    }
                },
                ValueState::Unset => return Err(ScriptError::MissingInput)
            }
        }
        (self.evaluate_fn)(&values, &output_port.clone())
    }

    pub fn set_values(&mut self, values: &HashMap<String, Option<Value>>) {
        for (title, value) in values {

        }
    }
}

struct PortBuilder {
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
}

pub struct NodeBuilder {
    execute_fn: Option<BoxedExecutableFn>,
    evaluate_fn: Option<BoxedEvaluableFn>,
    input_ports: Vec<CompiledPort>,
    output_ports: Vec<CompiledPort>,
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
        }
    }

    pub fn with_execution(mut self, execute_fn: BoxedExecutableFn) -> Self {
        self.execute_fn = Some(execute_fn);
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
        self.input_ports.push(PortBuilder::new(title).with_types(accepted_types).build());
        self
    }

    pub fn with_output(mut self, title: &'static str, returned_types: &[ValueType]) -> Self {
        self.input_ports.push(PortBuilder::new(title).with_types(returned_types).build());
        self
    }

    pub fn build(self) -> BoxedNodeFn {
        Box::new(move || {
            CompiledNode::new(
                self.execute_fn.clone().unwrap_or(
                    Arc::new(|_| Err(ScriptError::NotExecutable))
                ),
                self.evaluate_fn.clone().unwrap_or(
                    Arc::new(|_, _| Err(ScriptError::NotEvaluable))
                ),
            )
        })
    }
}
