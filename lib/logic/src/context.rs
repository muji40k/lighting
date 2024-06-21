
use std::collections::HashMap;

use domain::light::ProviderID;
use local_registry::Registry;
use provider::Provider;

pub struct Context {
    pub providers: HashMap<String, Box<dyn Provider>>,
    pub registry: Box<dyn Registry>,
}

impl Context {
    pub fn new(mut providers: Vec<Box<dyn Provider>>, registry: Box<dyn Registry>) -> Self {
        let mut map = HashMap::new();

        while let Some(provider) = providers.pop() {
            map.insert(provider.name().to_string(), provider);
        }

        Self {
            providers: map,
            registry,
        }
    }

    pub fn get_provider_by_name(self: &Self, provider: &str) -> Option<&dyn Provider> {
        self.providers.get(provider).map(Box::as_ref)
    }

    pub fn get_provider_by_id(self: &Self, id: &ProviderID) -> Option<&dyn Provider> {
        self.providers.get(&id.name).map(Box::as_ref)
    }
}

