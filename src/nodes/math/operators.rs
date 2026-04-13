use crate::{ScriptError, Value};
use crate::common::BoxedNodeFn;
use crate::NodeBuilder;
use crate::schemas::NodeSchema;
use crate::values::{ValueType};

pub fn build_add_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.math.add")
        .with_input("a", &[ValueType::Int, ValueType::Float])
        .with_input("b", &[ValueType::Int, ValueType::Float])
        .with_output("res", &[ValueType::Int, ValueType::Float])
        .with_evaluation(|values, _| {
            let a = values.get("a");
            let b = values.get("b");

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32) + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + (b as f32))),
                _ => Err(ScriptError::UnsupportedInput)
            }
        })
        .build()
}

pub fn build_subtract_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.math.subtract")
        .with_input("a", &[ValueType::Int, ValueType::Float])
        .with_input("b", &[ValueType::Int, ValueType::Float])
        .with_output("res", &[ValueType::Int, ValueType::Float])
        .with_evaluation(|values, _| {
            let a = values.get("a");
            let b = values.get("b");

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32) - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - (b as f32))),
                _ => Err(ScriptError::UnsupportedInput)
            }
        })
        .build()
}

pub fn build_multiply_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.math.multiply")
        .with_input("a", &[ValueType::Int, ValueType::Float])
        .with_input("b", &[ValueType::Int, ValueType::Float])
        .with_output("res", &[ValueType::Int, ValueType::Float])
        .with_evaluation(|values, _| {
            let a = values.get("a");
            let b = values.get("b");

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32) * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * (b as f32))),
                _ => Err(ScriptError::UnsupportedInput)
            }
        })
        .build()
}

pub fn build_divide_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.math.divide")
        .with_input("a", &[ValueType::Int, ValueType::Float])
        .with_input("b", &[ValueType::Int, ValueType::Float])
        .with_output("res", &[ValueType::Int, ValueType::Float])
        .with_evaluation(|values, _| {
            let a = values.get("a");
            let b = values.get("b");

            match b {
                Value::Int(0) => return Err(ScriptError::UnsupportedInput),
                Value::Float(0.0) => return Err(ScriptError::UnsupportedInput),
                _ => {}
            };

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32) / b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / (b as f32))),
                _ => Err(ScriptError::UnsupportedInput)
            }
        })
        .build()
}

pub fn build_pow_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.math.pow")
        .with_input("a", &[ValueType::Int, ValueType::Float])
        .with_input("b", &[ValueType::Int, ValueType::Float])
        .with_output("res", &[ValueType::Int, ValueType::Float])
        .with_evaluation(|values, _| {
            let a = values.get("a");
            let b = values.get("b");

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => {
                    if b >= 0 {
                        Ok(Value::Int(a.pow(b as u32)))
                    } else {
                        Ok(Value::Float((a as f32).powi(b)))
                    }
                },
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32).powf(b))),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powi(b))),
                _ => Err(ScriptError::UnsupportedInput)
            }
        })
        .build()
}

pub fn build_mod_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.math.mod")
        .with_input("a", &[ValueType::Int])
        .with_input("b", &[ValueType::Int])
        .with_output("res", &[ValueType::Int])
        .with_evaluation(|values, _| {
            let a = values.get("a");
            let b = values.get("b");

            if let Value::Int(0) = b {
                return Err(ScriptError::UnsupportedInput)
            }
            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
                _ => Err(ScriptError::UnsupportedInput)
            }
        })
        .build()
}

pub fn build_abs_node() -> (NodeSchema, BoxedNodeFn) {
    NodeBuilder::new("core.math.abs")
        .with_input("a", &[ValueType::Int, ValueType::Float])
        .with_output("res", &[ValueType::Int])
        .with_evaluation(|values, _| {
            let a = values.get("a");

            match a {
                Value::Int(a) => Ok(Value::Int(a.abs())),
                Value::Float(a) => Ok(Value::Float(a.abs())),
                _ => Err(ScriptError::UnsupportedInput)
            }
        })
        .build()
}

pub fn build_math_nodes() -> Vec<(NodeSchema, BoxedNodeFn)> {
    vec![
        build_add_node(),
        build_divide_node(),
        build_subtract_node(),
        build_multiply_node(),
        build_abs_node(),
        build_mod_node(),
        build_pow_node()
    ]
}