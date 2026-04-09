use std::collections::HashMap;
use crate::graph::EdgeType;
use crate::graph::Node;
use crate::context::ExecutionContext;
use petgraph::prelude::StableDiGraph;
use petgraph::graph::NodeIndex;
use crate::CompiledNode;

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug)]
pub enum RegistryError {
    AlreadyRegistered,
    Failed,
    NotFound(String),
}

#[derive(Debug)]
pub enum ScriptError {
    MissingInput,
    UnsupportedInput,
    NotEvaluable,
    NotExecutable,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all="snake_case"))]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Object(HashMap<String, Value>),
    None,
}

pub type BoxedExecutableFn = Box<dyn Fn(&mut ExecutionContext, &StableDiGraph<Node, EdgeType>, NodeIndex) -> Result<(), ScriptError>>;
pub type BoxedEvaluableFn = Box<dyn Fn(&mut ExecutionContext, &StableDiGraph<Node, EdgeType>, NodeIndex, &str) -> Result<Value, ScriptError>>;
pub type BoxedNodeFn = Box<dyn FnMut() -> CompiledNode>;