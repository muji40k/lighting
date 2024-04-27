
use serde_json as json;

use crate::dump::{Dumper, DumperResult};

pub struct JSONDumper {
    finished: bool,
    value: json::Value,
}

impl JSONDumper {
    pub fn new() -> Self {
        Self {
            finished: false,
            value: json::Value::Null,
        }
    }

    fn set(self: &mut Self, val: json::Value) {
        if self.finished {
            return;
        }

        self.value = val;
    }

    fn set_parameter(self: &mut Self, name: &str, val: json::Value) {
        if self.finished {
            return;
        }

        let name = String::from(name);
        match &mut self.value {
            json::Value::Object(map) => {
                map.insert(name, val);
            },
            _ => {
                let mut map: json::Map<String, json::Value> = json::Map::new();
                map.insert(name, val);
                self.value = json::Value::Object(map);
            }
        }
    }
}

fn value_array(val: &mut dyn Iterator<Item=&dyn crate::dump::Dumpable>) -> Vec<json::Value> {
    val.filter_map(|item| {
        let mut inner = JSONDumper::new();
        item.dump(&mut inner);
        inner.finish();

        inner.result()
    }).collect()
}

impl DumperResult for JSONDumper {
    type Item = json::Value;

    fn result(self: Self) -> Option<Self::Item> {
        if self.finished {
            Some(self.value)
        } else {
            None
        }
    }

    fn ready(self: &Self) -> bool {
        self.finished
    }
}

impl Dumper for JSONDumper {
    fn finish(self: &mut Self) {
        self.finished = true;
    }

    fn dump_u64(self: &mut Self, val: u64) {
        self.set(json::Value::Number(serde_json::Number::from(val)));
    }

    fn dump_i64(self: &mut Self, val: i64) {
        self.set(json::Value::Number(serde_json::Number::from(val)));
    }

    fn dump_f64(self: &mut Self, val: f64) {
        if let Some(val) = serde_json::Number::from_f64(val) {
            self.set(json::Value::Number(val));
        }
    }

    fn dump_str(self: &mut Self, val: &str) {
        self.set(json::Value::String(String::from(val)));
    }

    fn dump_arr(self: &mut Self, val: &mut dyn Iterator<Item=&dyn crate::dump::Dumpable>) {
        self.set(json::Value::Array(value_array(val)));
    }

    fn dump_bool(self: &mut Self, val: bool) {
        self.set(json::Value::Bool(val));
    }

    fn dump_u64_as_parameter(self: &mut Self, name: &str, val: u64) {
        self.set_parameter(name, json::Value::Number(json::Number::from(val)));
    }

    fn dump_i64_as_parameter(self: &mut Self, name: &str, val: i64) {
        self.set_parameter(name, json::Value::Number(json::Number::from(val)));
    }

    fn dump_f64_as_parameter(self: &mut Self, name: &str, val: f64) {
        if let Some(val) = serde_json::Number::from_f64(val) {
            self.set_parameter(name, json::Value::Number(val));
        }
    }

    fn dump_str_as_parameter(self: &mut Self, name: &str, val: &str) {
        self.set_parameter(name, json::Value::String(String::from(val)));
    }

    fn dump_arr_as_parameter(self: &mut Self, name: &str, val: &mut dyn Iterator<Item=&dyn crate::dump::Dumpable>) {
        let mut inner = Self::new();
        inner.dump_arr(val);
        inner.finish();

        self.set_parameter(name,
                           inner.result()
                           .expect("Finished dump so error shouldn't occur"));
    }

    fn dump_bool_as_parameter(self: &mut Self, name: &str, val: bool) {
        self.set_parameter(name, json::Value::Bool(val));
    }

    fn dump_fold_as_parameter(self: &mut Self, name: &str, val: &dyn crate::dump::Dumpable) {
        let mut inner = Self::new();
        val.dump(&mut inner);
        inner.finish();

        self.set_parameter(name,
                           inner.result()
                           .expect("Finished dump so error shouldn't occur"));
    }
}

