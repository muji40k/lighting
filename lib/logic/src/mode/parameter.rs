
use std::collections::HashMap;

use local_registry::dump::{Dumpable, Dumper};

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    name: String,
    value: Value,
}

#[derive(Clone, Debug, PartialEq)]
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

    pub fn name(self: &Self) -> &str {
        self.name.as_str()
    }

    pub fn value(self: &Self) -> &Value {
        &self.value
    }

    pub fn set_name(self: &mut Self, name: String) {
        self.name = name;
    }

    pub fn set_value(self: &mut Self, value: Value) {
        self.value = value;
    }
}

impl Dumpable for Parameter {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.name.dump_as_parameter(dumper, "name");
        self.value.dump_as_parameter(dumper, "value");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl Dumpable for Value {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        match self {
            Value::String(_) => "string",
            Value::Int(_) => "int",
            Value::UInt(_) => "uint",
            Value::Float(_) => "float",
            Value::Group(_) => "group",
            Value::Array(_) => "array",
        }.dump_as_parameter(dumper, "type");

        match self {
            Value::String(value) => value as &dyn Dumpable,
            Value::Int(value) => value as &dyn Dumpable,
            Value::UInt(value) => value as &dyn Dumpable,
            Value::Float(value) => value as &dyn Dumpable,
            Value::Group(value) => value as &dyn Dumpable,
            Value::Array(value) => value as &dyn Dumpable,
        }.dump_as_parameter(dumper, "value");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

