use crate::common::{ArcedEvaluableFn, ArcedExecutableFn, ScriptError};
use crate::context::ExecutionContext;
use crate::graph::{EdgeType, Node};
use crate::values::Value;
use crate::values::{ValueState, ValueType};
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use std::collections::HashMap;

pub struct NodeValues<'a> {
    ports: &'a HashMap<&'static str, CompiledPort>,
    map: HashMap<String, Value>,
}

impl<'a> NodeValues<'a> {
    pub fn new(ports: &'a HashMap<&'static str, CompiledPort>) -> Self {
        Self {
            map: HashMap::new(),
            ports,
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
                    return port.types[0].default_value().unwrap();
                }
                panic!("Unknown port: {}", key)
            }
        }
    }
}

#[derive(Clone)]
pub struct CompiledPort {
    title: &'static str,
    types: Vec<ValueType>,
    value: Option<Value>,
}

impl CompiledPort {
    pub fn new(title: &'static str, types: Vec<ValueType>) -> CompiledPort {
        Self {
            title,
            types,
            value: None,
        }
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
    execute_fn: ArcedExecutableFn,
    evaluate_fn: ArcedEvaluableFn,
    inputs: HashMap<&'static str, CompiledPort>,
    _outputs: HashMap<&'static str, CompiledPort>,
}

impl CompiledNode {
    pub fn new(
        execute_fn: ArcedExecutableFn,
        evaluate_fn: ArcedEvaluableFn,
        inputs: HashMap<&'static str, CompiledPort>,
        outputs: HashMap<&'static str, CompiledPort>,
    ) -> Self {
        Self {
            evaluate_fn,
            execute_fn,
            inputs,
            _outputs: outputs,
        }
    }

    fn get_values(
        &'_ self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
    ) -> Result<NodeValues<'_>, ScriptError> {
        let mut values = NodeValues::new(&self.inputs);
        for (port_title, port) in &self.inputs {
            match ctx.get_input(node, graph, port) {
                ValueState::Set(ref value) => {
                    if !port.validate(value) {
                        return Err(ScriptError::UnsupportedInput);
                    }
                    values.insert(String::from(*port_title), value.clone());
                }
                ValueState::Default => {
                    if let Some(default_value) = &port.value {
                        values.insert(String::from(*port_title), default_value.clone());
                    } else {
                        match port.types[0].default_value() {
                            Ok(value) => values.insert(String::from(*port_title), value),
                            Err(error) => return Err(error),
                        }
                    }
                }
                ValueState::Unset => return Err(ScriptError::MissingInput(port_title.to_string())),
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
            Ok(values) => (self.execute_fn)(&values),
            Err(error) => Err(error),
        }
    }

    pub fn evaluate(
        &self,
        ctx: &mut ExecutionContext,
        graph: &StableDiGraph<Node, EdgeType>,
        node: NodeIndex,
        output_port: &str,
    ) -> Result<Value, ScriptError> {
        match self.get_values(ctx, graph, node) {
            Ok(values) => (self.evaluate_fn)(&values, &output_port.to_owned()),
            Err(error) => Err(error),
        }
    }

    pub fn set_values(&mut self, values: &HashMap<String, Option<Value>>) -> Option<()> {
        for (title, value) in values {
            if let Some(port) = self.inputs.get_mut(title.as_str()) {
                port.set_value(value.clone())
            }
        }
        None
    }
}
