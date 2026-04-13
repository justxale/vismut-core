use crate::core::NodeValues;
use std::sync::Arc;
use crate::CompiledNode;
use crate::values::Value;

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

pub type BoxedExecutableFn = Arc<dyn Fn(&NodeValues) -> Result<(), ScriptError>>;
pub type BoxedEvaluableFn = Arc<dyn Fn(&NodeValues, &String) -> Result<Value, ScriptError>>;
pub type BoxedNodeFn = Box<dyn Fn() -> CompiledNode>;