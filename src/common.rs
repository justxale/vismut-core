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

impl ToString for RegistryError {
    fn to_string(&self) -> String {
        match self {
            RegistryError::AlreadyRegistered => String::from("AlreadyRegistered"),
            RegistryError::Failed => String::from("Failed"),
            RegistryError::NotFound(name) => format!("NotFound: {name}"),
        }
    }
}

#[derive(Debug)]
pub enum ScriptError {
    MissingInput(String),
    UnsupportedInput,
    NotEvaluable,
    NotExecutable,
    RuntimeError(String)
}

pub type BoxedExecutableFn = Arc<dyn Fn(&NodeValues) -> Result<(), ScriptError>>;
pub type BoxedEvaluableFn = Arc<dyn Fn(&NodeValues, &String) -> Result<Value, ScriptError>>;
pub type BoxedNodeFn = Box<dyn Fn() -> CompiledNode>;