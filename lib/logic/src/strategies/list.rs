
use domain::light::{Light, ProviderID};
use super::{Strategy, StrategyResult};
use crate::facade::Managers;
use crate::managers::fetch::Result;

pub struct ListAll(Option<Result<Vec<Light>>>);

impl ListAll {
    pub fn new() -> Self {
        Self(None)
    }
}

impl Strategy for ListAll {
    fn execute(self: &mut Self, managers: Managers) {
        self.0 = Some(managers.fetch.fetch_all())
    }
}

impl StrategyResult for ListAll {
    type Result = Result<Vec<Light>>;

    fn result(self: Self) -> Option<Self::Result> {
        self.0
    }
}

pub struct ListProvider<'a>(&'a str, Option<Result<Vec<Light>>>);

impl<'a> ListProvider<'a> {
    pub fn new(provider: &'a str) -> Self {
        Self(provider, None)
    }
}

impl<'a> Strategy for ListProvider<'a> {
    fn execute(self: &mut Self, managers: Managers) {
        self.1 = Some(managers.fetch.fetch_provider(self.0))
    }
}

impl<'a> StrategyResult for ListProvider<'a> {
    type Result = Result<Vec<Light>>;

    fn result(self: Self) -> Option<Self::Result> {
        self.1
    }
}

pub struct ListProviders<'a, I>(I, Option<Result<Vec<Light>>>)
where I: Iterator<Item = &'a str> + Clone;

impl<'a, I> ListProviders<'a, I>
where I: Iterator<Item = &'a str> + Clone {
    pub fn new(providers: I) -> Self {
        Self(providers, None)
    }
}

impl<'a, I> Strategy for ListProviders<'a, I>
where I: Iterator<Item = &'a str> + Clone {
    fn execute(self: &mut Self, managers: Managers) {
        self.1 = Some(
            self.0.clone()
                .map(|name| managers.fetch.fetch_provider(name))
                .try_fold(Vec::new(), |mut vec, list| {
                    list.map(|mut list| {
                        vec.append(&mut list);
                        vec
                    })
                })
        )
    }
}

impl<'a, I> StrategyResult for ListProviders<'a, I>
where I: Iterator<Item = &'a str> + Clone {
    type Result = Result<Vec<Light>>;

    fn result(self: Self) -> Option<Self::Result> {
        self.1
    }
}

pub struct GetById<'a>(&'a ProviderID, Option<Result<Light>>);

impl<'a> GetById<'a> {
    pub fn new(id: &'a ProviderID) -> Self {
        Self(id, None)
    }
}

impl<'a> Strategy for GetById<'a> {
    fn execute(self: &mut Self, managers: Managers) {
        self.1 = Some(managers.fetch.fetch(self.0))
    }
}

impl<'a> StrategyResult for GetById<'a> {
    type Result = Result<Light>;

    fn result(self: Self) -> Option<Self::Result> {
        self.1
    }
}

pub struct GetByIds<'a, I>(I, Option<Result<Vec<Light>>>)
where I: Iterator<Item = &'a ProviderID> + Clone;

impl<'a, I> GetByIds<'a, I>
where I: Iterator<Item = &'a ProviderID> + Clone {
    pub fn new(ids: I) -> Self {
        Self(ids, None)
    }
}

impl<'a, I> Strategy for GetByIds<'a, I>
where I: Iterator<Item = &'a ProviderID> + Clone {
    fn execute(self: &mut Self, managers: Managers) {
        self.1 = Some(
            self.0.clone()
                .map(|id| managers.fetch.fetch(id))
                .try_fold(Vec::new(), |mut acc, light| {
                    light.map(|light| {
                        acc.push(light);
                        acc
                    })
                })
        )
    }
}

impl<'a, I> StrategyResult for GetByIds<'a, I>
where I: Iterator<Item = &'a ProviderID> + Clone {
    type Result = Result<Vec<Light>>;

    fn result(self: Self) -> Option<Self::Result> {
        self.1
    }
}

