use crate::core::{ScriptError, VisualScript};
use crate::nodes::{AddNode, PrintNode, StartNode};

#[test]
fn main() -> Result<(), ScriptError> {
    let mut script = VisualScript::new();

    let start = script.add_node("Start", Box::new(StartNode));
    let add = script.add_node("Add", Box::new(AddNode::new(Some(10), Some(30))));
    let add2 = script.add_node("Add", Box::new(AddNode::new(None, Some(30))));
    let print = script.add_node("Print1", Box::new(PrintNode));
    let print2 = script.add_node("Print2", Box::new(PrintNode));

    script.connect_execution(start, print);
    script.connect_execution(print, print2);
    //script.connect_execution(add, add2);
    //script.connect_execution(add, print);
    //script.connect_execution(add2, print2);

    script.connect_data(add, print, "result", "value");
    script.connect_data(add2, print2, "result", "value");
    script.connect_data(add, add2, "result", "a");

    script.run()?;

    Ok(())
}