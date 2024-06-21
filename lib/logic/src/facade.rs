
use crate::managers::fetch::{FetchManager, SyncManager};
use crate::managers::local::LocalStateManager;

pub mod default;

pub struct Managers<'a> {
    pub fetch: &'a dyn FetchManager,
    pub sync:  &'a dyn SyncManager,
    pub local: &'a mut dyn LocalStateManager,
}

pub trait Strategy {
    fn execute(self: &mut Self, managers: Managers);
}

pub trait StrategyResult {
    type Result;

    fn result(self: Self) -> Option<Self::Result>;
}

pub trait Facade {
    fn accept(self: &mut Self, strategy: &mut dyn Strategy);
}

