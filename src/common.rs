use crate::core::NodeValues;
use crate::values::Value;
use crate::CompiledNode;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub enum RegistryError {
    AlreadyRegistered,
    Failed,
    NotFound(String),
}

impl Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RegistryError::AlreadyRegistered => f.write_str("AlreadyRegistered"),
            RegistryError::Failed => f.write_str("Failed"),
            RegistryError::NotFound(s) => f.write_fmt(format_args!("NotFound: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum ScriptError {
    MissingInput(String),
    UnsupportedInput,
    NotEvaluable,
    NotExecutable,
    RuntimeError(String),
}

pub type ArcedExecutableFn = Arc<dyn Fn(&NodeValues) -> Result<(), ScriptError>>;
pub type ArcedEvaluableFn = Arc<dyn Fn(&NodeValues, &String) -> Result<Value, ScriptError>>;
pub type BoxedNodeFn = Box<dyn Fn() -> CompiledNode>;
