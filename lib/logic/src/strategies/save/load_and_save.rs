
use domain::light::Light;
use super::{Strategy, StrategyResult, Error};
use crate::facade::Managers;

fn default_name(light: &Light) -> String {
    light.provider.to_string()
}

fn apply<NF: FnMut(&Light) -> String>(
    name_function: &mut NF,
    managers: &mut Managers,
    light: &mut Light
) -> Result<(), Error> {
    light.name = name_function(light);
    managers.local.save(light)
        .and_then(|_| managers.local.set_default(light))
        .map_err(|err| Error::Local(err))
}

pub struct All<NF>(NF, Option<Result<(), Error>>)
where NF: FnMut(&Light) -> String;

impl<NF> All<NF>
where NF: FnMut(&Light) -> String {
    pub fn new(name_function: NF) -> Self {
        Self(name_function, None)
    }
}

pub fn all_default() -> All<impl FnMut(&Light) -> String> {
    All(default_name, None)
}

impl<NF> Strategy for All<NF>
where NF: FnMut(&Light) -> String {
    fn execute(self: &mut Self, mut managers: Managers) {
        self.1 = Some(
            managers.fetch.fetch_all()
                .map_err(|err| Error::Fetch(err))
                .and_then(|mut lights| {
                    lights.iter_mut()
                        .try_for_each(|light| {
                            apply(&mut self.0, &mut managers, light)
                        })
                })
        )
    }
}

impl<NF> StrategyResult for All<NF>
where NF: FnMut(&Light) -> String {
    type Result = Result<(), Error>;

    fn result(self: Self) -> Option<Self::Result> {
        self.1
    }
}

pub struct Provider<'a, NF>(&'a str, NF, Option<Result<(), Error>>)
where NF: FnMut(&Light) -> String;

impl<'a, NF> Provider<'a, NF>
where NF: FnMut(&Light) -> String {
    pub fn new(provider: &'a str, name_function: NF) -> Self {
        Self(provider, name_function, None)
    }
}

pub fn provider_default<'a>(
    provider: &'a str
) -> Provider<'a, impl FnMut(&Light) -> String> {
    Provider(provider, default_name, None)
}

impl<'a, NF> Strategy for Provider<'a, NF>
where NF: FnMut(&Light) -> String {
    fn execute(self: &mut Self, mut managers: Managers) {
        self.2 = Some(
            managers.fetch.fetch_provider(self.0)
                .map_err(|err| Error::Fetch(err))
                .and_then(|mut lights| {
                    lights.iter_mut()
                        .try_for_each(|light| {
                            apply(&mut self.1, &mut managers, light)
                        })
                })
        )
    }
}

impl<'a, NF> StrategyResult for Provider<'a, NF>
where NF: FnMut(&Light) -> String {
    type Result = Result<(), Error>;

    fn result(self: Self) -> Option<Self::Result> {
        self.2
    }
}

pub struct Providers<'a, I, NF>(I, NF, Option<Result<(), Error>>)
where NF: FnMut(&Light) -> String,
      I: Iterator<Item = &'a str> + Clone;

impl<'a, I, NF> Providers<'a, I, NF>
where NF: FnMut(&Light) -> String,
      I: Iterator<Item = &'a str> + Clone {
    pub fn new(providers: I, name_function: NF) -> Self {
        Self(providers, name_function, None)
    }
}

pub fn providers_default<'a, I: Iterator<Item = &'a str> + Clone>(
    providers: I
) -> Providers<'a, I, impl FnMut(&Light) -> String> {
    Providers(providers, default_name, None)
}

impl<'a, I, NF> Strategy for Providers<'a, I, NF>
where NF: FnMut(&Light) -> String,
      I: Iterator<Item = &'a str> + Clone {
    fn execute(self: &mut Self, mut managers: Managers) {
        self.2 = Some(
            self.0.clone()
                .try_for_each(|provider| {
                    managers.fetch.fetch_provider(provider)
                        .map_err(|err| Error::Fetch(err))
                        .and_then(|mut lights| {
                            lights.iter_mut()
                                .try_for_each(|light| {
                                    apply(&mut self.1, &mut managers, light)
                                })
                        })
                })
        )
    }
}

impl<'a, I, NF> StrategyResult for Providers<'a, I, NF>
where NF: FnMut(&Light) -> String,
      I: Iterator<Item = &'a str> + Clone {
    type Result = Result<(), Error>;

    fn result(self: Self) -> Option<Self::Result> {
        self.2
    }
}

