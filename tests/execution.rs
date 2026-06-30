#![cfg(feature = "nodes")]

use std::collections::HashMap;
use vismut_core::schemas::ScriptDataPath;
use vismut_core::schemas::ScriptExecutionPath;
use vismut_core::schemas::ScriptNode;
use vismut_core::schemas::ScriptSchema;
use vismut_core::{ScriptError, Value, VismutRuntime};

#[derive(Debug, Clone)]
struct TestContext {}

fn create_script() -> ScriptSchema {
    ScriptSchema {
        entry: ScriptNode {
            node_id: String::from("core.io.start"),
            id: String::from("7485b216-161d-425e-96ce-3e694b80fa9b"),
            defaults: None,
        },
        nodes: vec![
            ScriptNode {
                node_id: String::from("core.math.add"),
                id: String::from("203c691c-8f34-4340-b2c3-872e666bbbdb"),
                defaults: Some(HashMap::from([
                    (String::from("a"), Some(Value::Int(40))),
                    (String::from("b"), Some(Value::Int(30))),
                ])),
            },
            ScriptNode {
                node_id: String::from("core.io.stdout"),
                id: String::from("3cec649b-27e6-4cf0-b857-4eb6ae2d0692"),
                defaults: None,
            },
        ],
        exec_paths: vec![ScriptExecutionPath {
            from: String::from("7485b216-161d-425e-96ce-3e694b80fa9b"),
            from_port: String::from("start"),
            to: String::from("3cec649b-27e6-4cf0-b857-4eb6ae2d0692"),
        }],
        data_paths: vec![ScriptDataPath {
            from: String::from("203c691c-8f34-4340-b2c3-872e666bbbdb"),
            to: String::from("3cec649b-27e6-4cf0-b857-4eb6ae2d0692"),
            from_port: String::from("result"),
            to_port: String::from("value"),
        }],
    }
}

#[test]
fn run_script() -> Result<(), ScriptError> {
    let env = VismutRuntime::<()>::default().with_builtins();

    let script = create_script();
    match env.parse(&script) {
        Ok(mut ready) => {
            assert_eq!(ready.run().unwrap(), 2);
            Ok(())
        }
        Err(e) => Err(ScriptError::RuntimeError(e.to_string())),
    }
}

#[test]
fn run_script_with_context() -> Result<(), ScriptError> {
    let ctx = TestContext {};
    let env = VismutRuntime::<TestContext>::new(ctx).with_builtins();

    let script = create_script();
    match env.parse(&script) {
        Ok(mut ready) => {
            assert_eq!(ready.run().unwrap(), 2);
            Ok(())
        }
        Err(e) => Err(ScriptError::RuntimeError(e.to_string())),
    }
}
