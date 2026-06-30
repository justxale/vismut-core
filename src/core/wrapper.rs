use crate::common::{ArcedEvaluableFn, ArcedExecutableFn, ScriptError};
use crate::context::RuntimeCache;
use crate::graph::{EdgeType, Node};
use crate::values::Value;
use crate::values::{ValueState, ValueType};
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use std::collections::HashMap;

pub struct NodeValues {
    // TODO: maybe use ports for some default value evaluation
    // ports: &'a HashMap<&'static str, CompiledPort>,
    map: HashMap<String, Value>,
}

impl NodeValues {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.map.get(key).map(|v| v.clone())
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

pub struct CompiledNode<C: Clone> {
    execute_fn: ArcedExecutableFn<C>,
    evaluate_fn: ArcedEvaluableFn<C>,
    inputs: HashMap<&'static str, CompiledPort>,
    _outputs: HashMap<&'static str, CompiledPort>,
}

impl<C: Clone> CompiledNode<C> {
    pub fn new(
        execute_fn: ArcedExecutableFn<C>,
        evaluate_fn: ArcedEvaluableFn<C>,
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
        cache: &mut RuntimeCache,
        graph: &StableDiGraph<Node<C>, EdgeType>,
        node: NodeIndex,
        ctx: &C
    ) -> Result<NodeValues, ScriptError> {
        let mut values = NodeValues::new();
        for (port_title, port) in &self.inputs {
            match cache.get_input(node, graph, port, ctx) {
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
        cache: &mut RuntimeCache,
        graph: &StableDiGraph<Node<C>, EdgeType>,
        node: NodeIndex,
        ctx: C
    ) -> Result<Option<&'static str>, ScriptError> { 
        self.get_values(cache, graph, node, &ctx)
            .map(|values| (self.execute_fn)(&values, ctx))?
    }

    pub fn evaluate(
        &self,
        cache: &mut RuntimeCache,
        graph: &StableDiGraph<Node<C>, EdgeType>,
        node: NodeIndex,
        output_port: &str,
        ctx: C
    ) -> Result<Value, ScriptError> {
        match self.get_values(cache, graph, node, &ctx) {
            Ok(values) => (self.evaluate_fn)(&values, &output_port.to_owned(), ctx),
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
