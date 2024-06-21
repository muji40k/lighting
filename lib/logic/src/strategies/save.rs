
use domain::light::{Light, ProviderID};
use super::{Strategy, StrategyResult};
use crate::facade::Managers;
use crate::managers::{fetch, local};

pub struct ListDefault(Option<local::Result<Vec<Light>>>);

impl ListDefault {
    pub fn new() -> Self {
        Self(None)
    }
}

impl Strategy for ListDefault {
    fn execute(self: &mut Self, managers: Managers) {
        self.0 = Some(managers.local.list_defaults())
    }
}

impl StrategyResult for ListDefault {
    type Result = local::Result<Vec<Light>>;

    fn result(self: Self) -> Option<Self::Result> {
        self.0
    }
}

pub struct ListDumps(Option<local::Result<Vec<Light>>>);

impl ListDumps {
    pub fn new() -> Self {
        Self(None)
    }
}

impl Strategy for ListDumps {
    fn execute(self: &mut Self, managers: Managers) {
        self.0 = Some(managers.local.list_dumps())
    }
}

impl StrategyResult for ListDumps {
    type Result = local::Result<Vec<Light>>;

    fn result(self: Self) -> Option<Self::Result> {
        self.0
    }
}

#[derive(Debug)]
pub enum Error {
    Fetch(fetch::Error),
    Local(local::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Fetch(err) => Some(err),
            Error::Local(err) => Some(err),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Fetch(err) => err.fmt(f),
            Error::Local(err) => err.fmt(f),
        }
    }
}

