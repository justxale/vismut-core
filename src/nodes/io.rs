use crate::Value;
use crate::extract;
use crate::common::BoxedNodeFn;
use crate::schemas::NodeSchema;
use crate::{NodeBuilder, ValueType};

pub fn build_stdout_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.io.stdout")
        .with_input("value", &[ValueType::Any])
        .with_execution(|values, _| {
            let v = values.get("value");
            log::debug!("core.io.stdout: {:?}", v);
            Ok(None)
        }, "exec", Some(&["exec"]))
        .build()
}

pub fn build_start_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::raw("core.io.start", |_, _| Ok(Some("start")), "start")
        .build()
}

pub fn build_branching_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.io.if")
        .with_input("condition", &[ValueType::Bool])
        .with_execution(|values, _| {
            let v = extract!(values, Value::Bool, "condition");
            if v {
                Ok(Some("true"))
            } else {
                Ok(Some("false"))
            }

        }, "exec", Some(&["true", "false"]))
        .build()
}

pub fn build_io_nodes<C: Clone + 'static>() -> Vec<(NodeSchema, BoxedNodeFn<C>)> {
    vec![build_stdout_node(), build_start_node()]
}
