use crate::common::BoxedNodeFn;
use std::io::Write;
use crate::{NodeBuilder, ScriptError, ValueType};
use crate::schemas::NodeSchema;

pub fn build_stdout_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.io.stdout")
        .with_input("value", &[ValueType::Any])
        .with_execution(|values| {
            let v = values.get("value");
            match std::io::stdout().write(format!("{:?}", v).as_bytes()) {
                Ok(_) => Ok(()),
                Err(err) => Err(ScriptError::RuntimeError(String::from(err.to_string()))),
            }
        })
        .build()
}

pub fn build_start_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.io.start")
        .with_input("value", &[ValueType::Any])
        .with_execution(|_| Ok(()))
        .with_no_exec_input()
        .build()
}
