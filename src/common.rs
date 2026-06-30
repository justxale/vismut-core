use crate::CompiledNode;
use crate::core::NodeValues;
use crate::values::Value;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use crate::context::RuntimeContext;

#[derive(Debug)]
pub enum RegistryError {
    AlreadyRegistered,
    Failed,
    NotFound(String),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegistryError::AlreadyRegistered => f.write_str("AlreadyRegistered"),
            RegistryError::Failed => f.write_str("Failed"),
            RegistryError::NotFound(s) => f.write_fmt(format_args!("NotFound: {}", s)),
        }
    }
}

impl Error for RegistryError {}

#[derive(Debug)]
pub enum ScriptError {
    MissingInput(String),
    UnsupportedInput,
    NotEvaluable,
    NotExecutable,
    RuntimeError(String),
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptError::MissingInput(v) => f.write_fmt(format_args!("missing input for port {v}")),
            ScriptError::RuntimeError(v) => f.write_fmt(format_args!("runtime error occured: {v}")),
            ScriptError::UnsupportedInput => f.write_str("unsupported input"),
            ScriptError::NotEvaluable => f.write_str("node is not evaluable"),
            ScriptError::NotExecutable => f.write_str("node is not executable"),
        }
    }
}

impl Error for ScriptError {}

pub type ArcedExecutableFn = Arc<dyn Fn(&NodeValues, &RuntimeContext) -> Result<Option<&'static str>, ScriptError> + Send + Sync>;
pub type ArcedEvaluableFn =
    Arc<dyn Fn(&NodeValues, &String, &RuntimeContext) -> Result<Value, ScriptError> + Send + Sync>;
pub type BoxedNodeFn = Box<dyn Fn() -> CompiledNode + Send + Sync>;
