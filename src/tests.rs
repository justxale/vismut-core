use crate::core::{ScriptError, Value, VisualScript};
use crate::nodes::{AddNode, PrintNode, StartNode, SubtractNode};
use crate::register::{ExecutionEnvironment, RegisterError};
use crate::traits::NodeBehavior;
use serde::Serialize;
use std::collections::HashMap;
#[test]
fn main() -> Result<(), ScriptError> {
    let mut script = VisualScript::new();
    let mut add_node1 = AddNode::new();
    add_node1.set_values(HashMap::from([
        (String::from("a"), Some(Value::Int(10))),
        (String::from("b"), Some(Value::Int(30))),
    ]));
    let mut add_node2 = AddNode::new();
    add_node2.set_values(HashMap::from([
        (String::from("a"), None),
        (String::from("b"), Some(Value::Int(30))),
    ]));

    let start = script.add_node("Start", Box::new(StartNode));
    let add = script.add_node("Add", add_node1);

    let add2 = script.add_node("Add", add_node2);
    let print = script.add_node("Print1", Box::new(PrintNode));
    let print2 = script.add_node("Print2", Box::new(PrintNode));

    script.connect_execution(start, print);
    script.connect_execution(print, print2);

    script.connect_data(add, print, "result", "value");
    script.connect_data(add2, print2, "result", "value");
    script.connect_data(add, add2, "result", "a");

    let result = script.run()?;
    println!("{:?}", result);
    Ok(())
}

#[test]
fn test_register() -> Result<(), RegisterError> {
    let mut env = ExecutionEnvironment::new();
    env.register(AddNode::new)?.register(SubtractNode::new)?;
    println!("{:?}", serde_json::to_string(&env.get_schema()));
    Ok(())
}
