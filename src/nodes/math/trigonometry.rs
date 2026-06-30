use crate::common::BoxedNodeFn;
use crate::schemas::NodeSchema;
use crate::{NodeBuilder, ScriptError, Value, ValueType};

pub fn build_sin_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.sin")
        .with_input("a", &[ValueType::Float, ValueType::Int])
        .with_output("res", &[ValueType::Float])
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            match a {
                Value::Int(a) => Ok(Value::Float((a as f32).sin())),
                Value::Float(a) => Ok(Value::Float(a.sin())),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_cos_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.cos")
        .with_input("a", &[ValueType::Float, ValueType::Int])
        .with_output("res", &[ValueType::Float])
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            match a {
                Value::Int(a) => Ok(Value::Float((a as f32).cos())),
                Value::Float(a) => Ok(Value::Float(a.cos())),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_tan_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.tan")
        .with_input("a", &[ValueType::Float, ValueType::Int])
        .with_output("res", &[ValueType::Float])
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            match a {
                Value::Int(a) => Ok(Value::Float((a as f32).tan())),
                Value::Float(a) => Ok(Value::Float(a.tan())),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_cot_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.cot")
        .with_input("a", &[ValueType::Float, ValueType::Int])
        .with_output("res", &[ValueType::Float])
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            match a {
                Value::Int(a) => Ok(Value::Float(1.0 / (a as f32).tan())),
                Value::Float(a) => Ok(Value::Float(1.0 / a.tan())),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_trigonometry_nodes<C: Clone + 'static>() -> Vec<(NodeSchema, BoxedNodeFn<C>)> {
    vec![
        build_sin_node(),
        build_cos_node(),
        build_tan_node(),
        build_cot_node(),
    ]
}
