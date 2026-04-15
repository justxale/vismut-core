#[cfg(feature = "serde")]
use serde::Serialize;
use crate::core::PortBuilder;
use crate::values::ValueType;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
pub struct NodeSchema {
    node_id: &'static str,
    is_executable: bool,
    is_evaluable: bool,
    outputs: Vec<PortSchema>,
    inputs: Vec<PortSchema>,
}

impl NodeSchema {
    pub fn new(
        node_id: &'static str,
        is_executable: bool,
        is_evaluable: bool,
        inputs: Vec<PortSchema>,
        outputs: Vec<PortSchema>,
    ) -> Self {
        Self {
            node_id,
            is_executable,
            is_evaluable,
            inputs,
            outputs,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.node_id
    }

    pub fn is_executable(&self) -> bool {
        self.is_executable
    }

    pub fn is_evaluable(&self) -> bool {
        self.is_evaluable
    }

    pub fn get_outputs(&self) -> &Vec<PortSchema> {
        &self.outputs
    }

    pub fn get_inputs(&self) -> &Vec<PortSchema> {
        &self.inputs
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct PortSchema {
    pub name: &'static str,
    pub kind: PortType,
    pub types: Vec<ValueType>
}

impl PortSchema {
    pub fn execution(name: &'static str) -> Self {
        Self {
            kind: PortType::Execution,
            name, types: vec![]
        }
    }
    
    pub fn data(name: &'static str, types: &Vec<ValueType>) -> Self {
        Self {
            kind: PortType::Data,
            name, types: types.clone()
        }
    }
}

impl From<&PortBuilder> for PortSchema {
    fn from(value: &PortBuilder) -> Self {
        Self::data(value.title(), value.types())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all="snake_case"))]
#[derive(Debug, Clone)]
pub enum PortType {
    Execution,
    Data,
}