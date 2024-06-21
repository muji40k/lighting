
use std::rc::Rc;
use std::cell::RefCell;

use domain::light::{
    Light,
    ProviderID,
};
use crate::context::Context;
use crate::managers::fetch::{
    self,
    FetchManager,
    SyncManager
};
use crate::managers::local::LocalStateManager;

pub struct ProviderManager {
    context: Rc<RefCell<Context>>,
}

pub struct RegistryManager {
    context: Rc<RefCell<Context>>,
}

impl ProviderManager {
    pub fn new(context: Rc<RefCell<Context>>) -> Self {
        Self {
            context
        }
    }
}

impl RegistryManager {
    pub fn new(context: Rc<RefCell<Context>>) -> Self {
        Self {
            context
        }
    }
}

impl FetchManager for ProviderManager {
    fn fetch_all(self: &Self) -> fetch::Result<Vec<Light>> {
        self.context.borrow().providers.values()
            .map(|provider| provider.list())
            .try_fold(Vec::new(), |mut vec, list| {
                match list {
                    Ok(mut list) => {
                        vec.append(&mut list);
                        Ok(vec)
                    },
                    Err(err) => Err(fetch::Error::Provider(err)),
                }
            })
    }

    fn fetch_provider(self: &Self, provider: &str) -> fetch::Result<Vec<Light>> {
        self.context.borrow().get_provider_by_name(provider)
            .and_then(|provider| Some(provider.list()))
            .map_or_else(
                ||     Err(fetch::Error::NotFound(provider.to_string())),
                |item| item.map_err(|err| fetch::Error::Provider(err))
            )
    }

    fn fetch(self: &Self, id: &ProviderID) -> fetch::Result<Light> {
        self.context.borrow().get_provider_by_id(id)
            .and_then(|provider| Some(provider.get(&id.id)))
            .map_or_else(
                ||     Err(fetch::Error::NotFound(id.name.clone())),
                |item| item.map_err(|err| fetch::Error::Provider(err))
            )
    }
}

impl SyncManager for ProviderManager {
    fn sync(self: &Self, light: &Light) -> fetch::Result<()> {
        self.context.borrow().get_provider_by_id(&light.provider)
            .and_then(|provider| Some(provider.sync(light)))
            .map_or_else(
                ||     Err(fetch::Error::NotFound(light.provider.name.clone())),
                |item| item.map_err(|err| fetch::Error::Provider(err))
            )
    }
}

impl LocalStateManager for RegistryManager {
    fn list_dumps(self: &Self) -> local_registry::Result<Vec<Light>> {
        self.context.borrow().registry.list_dumps()
    }

    fn list_defaults(self: &Self) -> local_registry::Result<Vec<Light>> {
        self.context.borrow().registry.list_defaults()
    }

    fn save(self: &mut Self, light: &Light) -> local_registry::Result<()> {
        self.context.borrow_mut().registry.dump(light)
    }

    fn load(self: &Self, name: &str) -> local_registry::Result<Light> {
        self.context.borrow().registry.load_dump(name)
    }

    fn set_default(self: &mut Self, light: &Light) -> local_registry::Result<()> {
        self.context.borrow_mut().registry.default(light)
    }

    fn get_default(self: &Self, name: &str) -> local_registry::Result<Light> {
        self.context.borrow().registry.load_default(name)
    }

    fn remove(self: &mut Self, name: &str) -> local_registry::Result<()> {
        self.context.borrow_mut().registry.remove(name)
    }

    fn rename(self: &mut Self, old: &str, new: &str) -> local_registry::Result<()> {
        self.context.borrow_mut().registry.rename(old, new)
    }
}

