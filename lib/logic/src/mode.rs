
pub mod parameter;

use parameter::{Parameter, Value};
use dump::{Dumpable, Dumper};

pub struct Mode {
    provider: String,
    name: String,
    parameters: Vec<Parameter>,
}

impl Mode {
    pub fn new_empty(provider: String, name: String) -> Self {
        Self {
            provider,
            name,
            parameters: Vec::new(),
        }
    }

    pub fn new(provider: String, name: String, parameters: Vec<Parameter>) -> Self {
        Self {
            provider,
            name,
            parameters,
        }
    }

    pub fn provider(self: &Self) -> &str {
        self.provider.as_str()
    }

    pub fn name(self: &Self) -> &str {
        self.name.as_str()
    }

    pub fn parameter_names(self: &Self) -> impl Iterator<Item=&str> {
        self.parameters.iter().map(|item| item.name())
    }

    pub fn parameters(self: &Self) -> impl Iterator<Item=&Parameter> {
        self.parameters.iter()
    }

    pub fn parameter(self: &Self, name: &str) -> Option<&Value> {
        match self.parameters.iter().find(|item| item.name() == name) {
            Some(param) => Some(param.value()),
            _ => None
        }
    }
}

impl Dumpable for Mode {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.provider.dump_as_parameter(dumper, "provider");
        self.name.dump_as_parameter(dumper, "name");
        self.parameters.dump_as_parameter(dumper, "parameters");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let mode = Mode::new_empty(String::from("provider"),
                                   String::from("name"));

        assert_eq!(mode.name(), "name")
    }

    #[test]
    fn test_get_provider() {
        let mode = Mode::new_empty(String::from("provider"),
                                   String::from("name"));

        assert_eq!(mode.provider(), "provider")
    }

    #[test]
    fn test_get_parameter_names() {
        let mode = Mode::new(String::from("provider"), String::from("name"),
                             vec![
                                Parameter::new(String::from("name1"), Value::Int(1)),
                                Parameter::new(String::from("name2"), Value::Int(2)),
                                Parameter::new(String::from("name3"), Value::Int(3)),
                             ]);

        assert_eq!(mode.parameter_names().collect::<Vec<&str>>(), &["name1", "name2", "name3"])
    }

    #[test]
    fn test_get_parameter_exists() {
        let mode = Mode::new(String::from("provider"), String::from("name"),
                             vec![
                                Parameter::new(String::from("name1"), Value::Int(1)),
                                Parameter::new(String::from("name2"), Value::Int(2)),
                                Parameter::new(String::from("name3"), Value::Int(3)),
                             ]);

        assert!(mode.parameter("name2").is_some_and(|param| {
            match param {
                Value::Int(2) => true,
                _ => false,
            }
        }));
    }

    #[test]
    fn test_get_parameter_not_exists() {
        let mode = Mode::new(String::from("provider"), String::from("name"),
                             vec![
                                Parameter::new(String::from("name1"), Value::Int(1)),
                                Parameter::new(String::from("name2"), Value::Int(2)),
                                Parameter::new(String::from("name3"), Value::Int(3)),
                             ]);

        assert!(mode.parameter("name4").is_none())
    }

    #[test]
    fn test_get_parameters() {
        let parameters = vec![
            Parameter::new(String::from("name1"), Value::Int(1)),
            Parameter::new(String::from("name2"), Value::Int(2)),
            Parameter::new(String::from("name3"), Value::Int(3)),
        ];

        let mode = Mode::new(String::from("provider"), String::from("name"),
                             parameters.clone());

        assert_eq!(mode.parameters().collect::<Vec<&Parameter>>(),
                   parameters.iter().collect::<Vec<&Parameter>>())
    }
}

