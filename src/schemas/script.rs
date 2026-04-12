use std::collections::HashMap;
use crate::values::Value;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone)]
pub struct ScriptNode {
    pub node_id: String,
    pub id: String,
    pub defaults: Option<HashMap<String, Option<Value>>>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone)]
pub struct ScriptExecutionPath {
    pub from: String,
    pub to: String,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone)]
pub struct ScriptDataPath {
    pub from: String,
    pub from_port: String,
    pub to: String,
    pub to_port: String,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone)]
pub struct ScriptSchema {
    pub entry: ScriptNode,
    pub nodes: Vec<ScriptNode>,
    pub exec_paths: Vec<ScriptExecutionPath>,
    pub data_paths: Vec<ScriptDataPath>,
}