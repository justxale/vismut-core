use crate::ScriptError;
#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Value {
    Int(i32),
    BigInt(i64),
    Float(f32),
    Bool(bool),
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    None,
    #[cfg(feature = "uuid")]
    Uuid(Uuid),
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Int(_) => ValueType::Int,
            Value::BigInt(_) => ValueType::BigInt,
            Value::Float(_) => ValueType::Float,
            Value::Bool(_) => ValueType::Bool,
            Value::String(_) => ValueType::String,
            Value::Object(_) => ValueType::Object,
            Value::Array(_) => ValueType::Array,
            Value::None => ValueType::None,
            #[cfg(feature = "uuid")]
            Value::Uuid(_) => ValueType::Uuid,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
pub enum ValueState {
    Set(Value),
    Default,
    Unset,
}

#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "snake_case"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int,
    BigInt,
    Float,
    Bool,
    String,
    Object,
    Array,
    Any,
    None,
    #[cfg(feature = "uuid")]
    Uuid,
}

impl ValueType {
    pub fn default_value(&self) -> Result<Value, ScriptError> {
        match self {
            ValueType::Int => Ok(Value::Int(0)),
            ValueType::Float => Ok(Value::Float(0.0)),
            ValueType::Bool => Ok(Value::Bool(false)),
            _ => Err(ScriptError::UnsupportedInput),
        }
    }

    pub fn is_default_supported(&self) -> bool {
        matches!(self, ValueType::Int | ValueType::Float | ValueType::Bool)
    }
}
