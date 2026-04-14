use crate::common::BoxedNodeFn;
use crate::{NodeBuilder, ValueType};
use crate::schemas::NodeSchema;

pub fn build_stdout_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.io.stdout")
        .with_input("value", &[ValueType::Any])
        .with_execution(|values| {
            let v = values.get("value");
            log::debug!("core.io.stdout: {:?}", v);
            Ok(())
        })
        .build()
}

pub fn build_start_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.io.start")
        .with_execution(|_| Ok(()))
        .with_no_exec_input()
        .build()
}

pub fn build_io_nodes() -> Vec<(NodeSchema, BoxedNodeFn)> {
    vec![build_stdout_node(), build_start_node()]
}