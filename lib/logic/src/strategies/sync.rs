
use domain::light::{Light, ProviderID};
use super::{Strategy, StrategyResult};
use crate::facade::Managers;
use crate::managers::{fetch, local};

pub struct General<'a>(&'a Light, Option<Result<(), Error>>);

impl<'a> General<'a> {
    pub fn new(light: &'a Light) -> Self {
        Self(light, None)
    }
}

impl<'a> Strategy for General<'a> {
    fn execute(self: &mut Self, managers: Managers) {
        self.1 = Some(
            managers.sync.sync(self.0)
                .map_err(|err| Error::Fetch(err))
                .and_then(|_| {
                    if self.0.name.is_empty() {
                        Ok(())
                    } else {
                        managers.local.save(&self.0)
                            .map_err(|err| Error::Local(err))
                    }
                })
        )
    }
}

impl<'a> StrategyResult for General<'a> {
    type Result = Result<(), Error>;

    fn result(self: Self) -> Option<Self::Result> {
        self.1
    }
}

pub mod fetch_and_sync {
    use super::*;

    fn transform(
        id: &ProviderID,
        map: &mut dyn FnMut(&mut Light),
        managers: &mut Managers
    ) -> Result<(), fetch::Error> {
        managers.fetch.fetch(id)
            .and_then(|mut light| {
                map(&mut light);
                managers.sync.sync(&light)
            })
    }

    pub type Single<'a, F, T> =
        misc::Single<'a, F, T, ProviderID, (), fetch::Error>;

    pub fn single<'a>(
        id: &'a ProviderID,
        map: impl FnMut(&mut Light)
    ) -> Single<
        'a,
        impl FnMut(&mut Light),
        impl FnMut(
            &ProviderID,
            &mut dyn FnMut(&mut Light),
            &mut Managers
        ) -> Result<(), fetch::Error>
    > {
        misc::Single::new(
            transform,
            map,
            id
        )
    }

    pub type Multiple<'a, F, T, I> =
        misc::Multiple<'a, F, T, ProviderID, I, (), fetch::Error>;

    pub fn multiple<'a>(
        ids: impl Iterator<Item = &'a ProviderID> + Clone,
        map: impl FnMut(&mut Light)
    ) -> Multiple<
        'a,
        impl FnMut(&mut Light),
        impl FnMut(
            &ProviderID,
            &mut dyn FnMut(&mut Light),
            &mut Managers
        ) -> Result<(), fetch::Error>,
        impl Iterator<Item = &'a ProviderID> + Clone
    > {
        misc::Multiple::new(
            transform,
            map,
            ids
        )
    }
}

pub mod load_and_sync {
    use super::*;

    fn transform(
        name: &str,
        map: &mut dyn FnMut(&mut Light),
        managers: &mut Managers
    ) -> Result<(), Error> {
        managers.local.load(name)
            .map_err(|err| Error::Local(err))
            .map(|mut light| { map(&mut light); light })
            .and_then(|mut light| {
                managers.sync.sync(&light)
                    .map_err(|err| Error::Fetch(err))
                    .and_then(|_| {
                        if light.name != name {
                            light.name = name.to_string();
                        }

                        managers.local.save(&light)
                            .map_err(|err| Error::Local(err))
                    })
            })
    }

    pub type Single<'a, F, T> = misc::Single<'a, F, T, str, (), Error>;

    pub fn single<'a>(
        name: &'a str,
        map: impl FnMut(&mut Light)
    ) -> Single<
        'a,
        impl FnMut(&mut Light),
        impl FnMut(
            &str,
            &mut dyn FnMut(&mut Light),
            &mut Managers
        ) -> Result<(), Error>
    > {
        misc::Single::new(
            transform,
            map,
            name
        )
    }

    pub type Multiple<'a, F, T, I> =
        misc::Multiple<'a, F, T, str, I, (), Error>;

    pub fn multiple<'a>(
        names: impl Iterator<Item = &'a str> + Clone,
        map: impl FnMut(&mut Light)
    ) -> Multiple<
        'a,
        impl FnMut(&mut Light),
        impl FnMut(
            &str,
            &mut dyn FnMut(&mut Light),
            &mut Managers
        ) -> Result<(), Error>,
        impl Iterator<Item = &'a str> + Clone
    > {
        misc::Multiple::new(
            transform,
            map,
            names
        )
    }
}

pub mod default_and_sync {
    use super::*;

    fn transform(
        name: &str,
        map: &mut dyn FnMut(&mut Light),
        managers: &mut Managers
    ) -> Result<(), Error> {
        managers.local.get_default(name)
            .map_err(|err| Error::Local(err))
            .map(|mut light| { map(&mut light); light })
            .and_then(|mut light| {
                managers.sync.sync(&light)
                    .map_err(|err| Error::Fetch(err))
                    .and_then(|_| {
                        if light.name != name {
                            light.name = name.to_string();
                        }

                        managers.local.save(&light)
                            .map_err(|err| Error::Local(err))
                    })
            })
    }

    pub type Single<'a, F, T> = misc::Single<'a, F, T, str, (), Error>;

    pub fn single<'a>(
        name: &'a str,
        map: impl FnMut(&mut Light)
    ) -> Single<
        'a,
        impl FnMut(&mut Light),
        impl FnMut(
            &str,
            &mut dyn FnMut(&mut Light),
            &mut Managers
        ) -> Result<(), Error>
    > {
        misc::Single::new(
            transform,
            map,
            name
        )
    }

    pub type Multiple<'a, F, T, I> =
        misc::Multiple<'a, F, T, str, I, (), Error>;

    pub fn multiple<'a>(
        names: impl Iterator<Item = &'a str> + Clone,
        map: impl FnMut(&mut Light)
    ) -> Multiple<
        'a,
        impl FnMut(&mut Light),
        impl FnMut(
            &str,
            &mut dyn FnMut(&mut Light),
            &mut Managers
        ) -> Result<(), Error>,
        impl Iterator<Item = &'a str> + Clone
    > {
        misc::Multiple::new(
            transform,
            map,
            names
        )
    }
}

mod misc {
    use super::*;

    pub struct Single<'a, F, T, ID, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          ID: ?Sized,
          E: std::error::Error {
        transformer: T,
        map: F,
        id: &'a ID,
        result: Option<Result<R, E>>,
    }

    pub struct Multiple<'a, F, T, ID, I, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          R: Default,
          E: std::error::Error {
        transformer: T,
        map: F,
        ids: I,
        result: Option<Result<R, E>>,
    }

    impl<'a, F, T, ID, R, E> Single<'a, F, T, ID, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          ID: ?Sized,
          E: std::error::Error {
        pub fn new(transformer: T, map: F, id: &'a ID) -> Self {
            Self {
                transformer,
                map,
                id,
                result: None
            }
        }
    }

    impl<'a, F, T, ID, I, R, E> Multiple<'a, F, T, ID, I, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          R: Default,
          E: std::error::Error {
        pub fn new(transformer: T, map: F, ids: I) -> Self {
            Self {
                transformer,
                map,
                ids,
                result: None,
            }
        }
    }

    impl<'a, F, T, ID, R, E> Strategy for Single<'a, F, T, ID, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          ID: ?Sized,
          E: std::error::Error {
        fn execute(self: &mut Self, mut managers: Managers) {
            self.result = Some(
                (self.transformer)(self.id, &mut self.map, &mut managers)
            )
        }
    }

    impl<'a, F, T, ID, R, E> StrategyResult for Single<'a, F, T, ID, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          ID: ?Sized,
          E: std::error::Error {
        type Result = Result<R, E>;

        fn result(self: Self) -> Option<Self::Result> {
            self.result
        }
    }

    impl<'a, F, T, ID, I, R, E> Strategy for Multiple<'a, F, T, ID, I, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          R: Default,
          E: std::error::Error {
        fn execute(self: &mut Self, mut managers: Managers) {
            self.result = Some({
                let mut clone = self.ids.clone();
                let mut res = Ok(R::default());

                while let (Ok(_), Some(id)) = (&res, clone.next()) {
                    res = (self.transformer)(id, &mut self.map, &mut managers);
                }

                res
            })
        }
    }

    impl<'a, F, T, ID, I, R, E> StrategyResult for Multiple<'a, F, T, ID, I, R, E>
    where F: FnMut(&mut Light),
          T: FnMut(&ID, &mut dyn FnMut(&mut Light), &mut Managers) -> Result<R, E>,
          I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          R: Default,
          E: std::error::Error {
        type Result = Result<R, E>;

        fn result(self: Self) -> Option<Self::Result> {
            self.result
        }
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

