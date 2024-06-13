
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Int(i64),
    UInt(u64),
    Float(f64),
    Group(HashMap<String, Parameter>),
    Array(Vec<Value>),
}

impl Parameter {
    pub fn new(name: String, value: Value) -> Self {
        Self {
            name,
            value,
        }
    }
}

