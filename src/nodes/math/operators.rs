use crate::NodeBuilder;
use crate::common::BoxedNodeFn;
use crate::schemas::NodeSchema;
use crate::values::ValueType;
use crate::{ScriptError, Value};

const NUMBER_TYPES: [ValueType; 3] = [ValueType::Int, ValueType::BigInt, ValueType::Float];

pub fn build_add_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.add")
        .with_input("a", &NUMBER_TYPES)
        .with_input("b", &NUMBER_TYPES)
        .with_output("res", &NUMBER_TYPES)
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            let b = values.get("b").unwrap_or(Value::Int(0));

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.overflowing_add(b).0)),
                (Value::BigInt(a), Value::BigInt(b)) => Ok(Value::BigInt(a.overflowing_add(b).0)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f32 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f32)),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_subtract_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.subtract")
        .with_input("a", &NUMBER_TYPES)
        .with_input("b", &NUMBER_TYPES)
        .with_output("res", &NUMBER_TYPES)
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            let b = values.get("b").unwrap_or(Value::Int(0));

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.overflowing_sub(b).0)),
                (Value::BigInt(a), Value::BigInt(b)) => Ok(Value::BigInt(a.overflowing_sub(b).0)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32) - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - (b as f32))),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_multiply_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.multiply")
        .with_input("a", &NUMBER_TYPES)
        .with_input("b", &NUMBER_TYPES)
        .with_output("res", &NUMBER_TYPES)
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            let b = values.get("b").unwrap_or(Value::Int(0));

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.overflowing_mul(b).0)),
                (Value::BigInt(a), Value::BigInt(b)) => Ok(Value::BigInt(a.overflowing_mul(b).0)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32) * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * (b as f32))),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_divide_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.divide")
        .with_input("a", &NUMBER_TYPES)
        .with_input("b", &NUMBER_TYPES)
        .with_output("res", &NUMBER_TYPES)
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            let b = values.get("b").unwrap_or(Value::Int(0));

            match b {
                Value::Int(0) => return Err(ScriptError::UnsupportedInput),
                Value::Float(0.0) => return Err(ScriptError::UnsupportedInput),
                _ => {}
            };

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.overflowing_div(b).0)),
                (Value::BigInt(a), Value::BigInt(b)) => Ok(Value::BigInt(a.overflowing_div(b).0)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32) / b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / (b as f32))),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_pow_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.pow")
        .with_input("a", &NUMBER_TYPES)
        .with_input("b", &[ValueType::Int, ValueType::Float])
        .with_output("res", &NUMBER_TYPES)
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            let b = values.get("b").unwrap_or(Value::Int(0));

            match (a, b) {
                (Value::Int(a), Value::Int(b)) => {
                    if b >= 0 {
                        Ok(Value::Int(a.pow(b as u32)))
                    } else {
                        Ok(Value::Float((a as f32).powi(b)))
                    }
                },
                (Value::BigInt(a), Value::Int(b)) => {
                    if b >= 0 {
                        Ok(Value::BigInt(a.pow(b as u32)))
                    } else {
                        Ok(Value::Float((a as f32).powi(b)))
                    }
                }
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f32).powf(b))),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powi(b))),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_rem_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.rem")
        .with_input("a", &[ValueType::Int, ValueType::BigInt])
        .with_input("b", &[ValueType::Int])
        .with_output("res", &[ValueType::Int, ValueType::BigInt])
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));
            let b = values.get("b").unwrap_or(Value::Int(1));

            if let Value::Int(0) = b {
                return Err(ScriptError::UnsupportedInput);
            }
            match (a, b) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.overflowing_rem(b).0)),
                (Value::BigInt(a), Value::Int(b)) => Ok(Value::BigInt(a.overflowing_rem(b as i64).0)),
                (Value::BigInt(a), Value::BigInt(b)) => Ok(Value::BigInt(a.overflowing_rem(b).0)),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_abs_node<C: Clone + 'static>() -> (NodeSchema, BoxedNodeFn<C>) {
    NodeBuilder::new("core.math.abs")
        .with_input("a", &NUMBER_TYPES)
        .with_output("res", &NUMBER_TYPES)
        .with_evaluation(|values, _, _| {
            let a = values.get("a").unwrap_or(Value::Int(0));

            match a {
                Value::Int(a) => Ok(Value::Int(a.abs())),
                Value::BigInt(a) => Ok(Value::BigInt(a.abs())),
                Value::Float(a) => Ok(Value::Float(a.abs())),
                _ => Err(ScriptError::UnsupportedInput),
            }
        })
        .build()
}

pub fn build_math_nodes<C: Clone + 'static>() -> Vec<(NodeSchema, BoxedNodeFn<C>)> {
    vec![
        build_add_node(),
        build_divide_node(),
        build_subtract_node(),
        build_multiply_node(),
        build_abs_node(),
        build_rem_node(),
        build_pow_node(),
    ]
}
