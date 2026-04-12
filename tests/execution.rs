use std::collections::HashMap;
use vismut_core::{ExecutionEnvironment, ScriptDataPath, ScriptExecutionPath, ScriptNode, ScriptSchema};
use vismut_core::RegistryError;
use vismut_core::values::Value;

#[test]
#[cfg(feature = "nodes")]
fn run_script() -> Result<(), RegistryError> {
    let env = ExecutionEnvironment::default();

    let script = ScriptSchema {
        entry: ScriptNode {
            node_id: String::from("core.io.start"),
            id: String::from("7485b216-161d-425e-96ce-3e694b80fa9b"),
            defaults: None
        },
        nodes: vec![
            ScriptNode {
                node_id: String::from("core.math.add"),
                id: String::from("203c691c-8f34-4340-b2c3-872e666bbbdb"),
                defaults: Some(
                    HashMap::from(
                        [
                            (String::from("a"), Some(Value::Int(40))),
                            (String::from("b"), Some(Value::Int(30)))
                        ]
                    )
                )
            },
            ScriptNode {
                node_id: String::from("core.io.print"),
                id: String::from("3cec649b-27e6-4cf0-b857-4eb6ae2d0692"),
                defaults: None
            }
        ],
        exec_paths: vec![
            ScriptExecutionPath {
                from: String::from("7485b216-161d-425e-96ce-3e694b80fa9b"),
                to: String::from("3cec649b-27e6-4cf0-b857-4eb6ae2d0692")
            }
        ],
        data_paths: vec![
            ScriptDataPath {
                from: String::from("203c691c-8f34-4340-b2c3-872e666bbbdb"),
                to: String::from("3cec649b-27e6-4cf0-b857-4eb6ae2d0692"),
                from_port: String::from("result"),
                to_port: String::from("value"),
            },
        ]
    };
    match env.parse(&script) {
        Ok(mut ready) => {let _ = ready.run(); Ok(())},
        Err(e) => Err(e)
    }
}