
use std::rc::Rc;
use std::cell::RefCell;

use super::{Facade, Managers, Strategy};

use crate::context::Context;
use crate::managers::default::{
    ProviderManager,
    RegistryManager,
};

pub struct DefaultFacade {
    provider_manager: Box<ProviderManager>,
    registry_manager: Box<RegistryManager>,
}

impl DefaultFacade {
    pub fn new(context: Rc<RefCell<Context>>) -> Self {
        Self {
            provider_manager: Box::new(ProviderManager::new(context.clone())),
            registry_manager: Box::new(RegistryManager::new(context)),
        }
    }
}

impl Facade for DefaultFacade {
    fn accept(self: &mut Self, strategy: &mut dyn Strategy) {
        strategy.execute(Managers {
            fetch: self.provider_manager.as_ref(),
            sync: self.provider_manager.as_ref(),
            local: self.registry_manager.as_mut(),
        })
    }
}

