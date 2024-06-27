
use domain::light::{Light, ProviderID};
use super::{Strategy, StrategyResult};
use crate::facade::Managers;
use crate::managers::{fetch, local};

pub mod provider {
    use super::*;

    pub struct All(Option<fetch::Result<Vec<Light>>>);

    impl All {
        pub fn new() -> Self {
            Self(None)
        }
    }

    impl Strategy for All {
        fn execute(self: &mut Self, managers: Managers) {
            self.0 = Some(managers.fetch.fetch_all())
        }
    }

    impl StrategyResult for All {
        type Result = fetch::Result<Vec<Light>>;

        fn result(self: Self) -> Option<Self::Result> {
            self.0
        }
    }

    pub struct Single<'a>(&'a str, Option<fetch::Result<Vec<Light>>>);

    impl<'a> Single<'a> {
        pub fn new(provider: &'a str) -> Self {
            Self(provider, None)
        }
    }

    impl<'a> Strategy for Single<'a> {
        fn execute(self: &mut Self, managers: Managers) {
            self.1 = Some(managers.fetch.fetch_provider(self.0))
        }
    }

    impl<'a> StrategyResult for Single<'a> {
        type Result = fetch::Result<Vec<Light>>;

        fn result(self: Self) -> Option<Self::Result> {
            self.1
        }
    }

    pub struct Multiple<'a, I>(I, Option<fetch::Result<Vec<Light>>>)
    where I: Iterator<Item = &'a str> + Clone;

    impl<'a, I> Multiple<'a, I>
    where I: Iterator<Item = &'a str> + Clone {
        pub fn new(providers: I) -> Self {
            Self(providers, None)
        }
    }

    impl<'a, I> Strategy for Multiple<'a, I>
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

    impl<'a, I> StrategyResult for Multiple<'a, I>
    where I: Iterator<Item = &'a str> + Clone {
        type Result = fetch::Result<Vec<Light>>;

        fn result(self: Self) -> Option<Self::Result> {
            self.1
        }
    }

    fn getter(
        managers: &Managers,
        id: &ProviderID
    ) -> Result<Light, fetch::Error> {
        managers.fetch.fetch(id)
    }

    pub type GetById<'a, G> = misc::GetById<'a, ProviderID, G, fetch::Error>;

    pub fn get_by_id<'a>(
        id: &'a ProviderID
    ) -> GetById<
        'a,
        impl FnMut(&Managers, &ProviderID) -> Result<Light, fetch::Error>,
    > {
        misc::GetById::new(id, getter)
    }

    pub type GetByIds<'a, I, G> =
        misc::GetByIds<'a, I, ProviderID, G, fetch::Error>;

    pub fn get_by_ids<'a, I: Iterator<Item = &'a ProviderID> + Clone>(
        ids: I
    ) -> GetByIds<
        'a,
        I,
        impl FnMut(&Managers, &ProviderID) -> Result<Light, fetch::Error>,
    > {
        misc::GetByIds::new(ids, getter)
    }
}

pub mod registry {
    pub mod dumps {
        use super::super::*;

        pub struct All(Option<local::Result<Vec<Light>>>);

        impl All {
            pub fn new() -> Self {
                Self(None)
            }
        }

        impl Strategy for All {
            fn execute(self: &mut Self, managers: Managers) {
                self.0 = Some(managers.local.list_dumps())
            }
        }

        impl StrategyResult for All {
            type Result = local::Result<Vec<Light>>;

            fn result(self: Self) -> Option<Self::Result> {
                self.0
            }
        }

        fn getter(
            managers: &Managers,
            name: &str
        ) -> Result<Light, local::Error> {
            managers.local.load(name)
        }

        pub type GetById<'a, G> = misc::GetById<'a, str, G, local::Error>;

        pub fn get_by_name<'a>(
            name: &'a str
        ) -> GetById<
            'a,
            impl FnMut(&Managers, &str) -> Result<Light, local::Error>
        > {
            misc::GetById::new(name, getter)
        }

        pub type GetByIds<'a, I, G> =
            misc::GetByIds<'a, I, str, G, local::Error>;

        pub fn get_by_names<'a, I: Iterator<Item = &'a str> + Clone>(
            names: I
        ) -> GetByIds<
            'a,
            I,
            impl FnMut(&Managers, &str) -> Result<Light, local::Error>,
        > {
            misc::GetByIds::new(names, getter)
        }
    }

    pub mod defaults {
        use super::super::*;

        pub struct All(Option<local::Result<Vec<Light>>>);

        impl All {
            pub fn new() -> Self {
                Self(None)
            }
        }

        impl Strategy for All {
            fn execute(self: &mut Self, managers: Managers) {
                self.0 = Some(managers.local.list_defaults())
            }
        }

        impl StrategyResult for All {
            type Result = local::Result<Vec<Light>>;

            fn result(self: Self) -> Option<Self::Result> {
                self.0
            }
        }

        fn getter(
            managers: &Managers,
            name: &str
        ) -> Result<Light, local::Error> {
            managers.local.get_default(name)
        }

        pub type GetById<'a, G> = misc::GetById<'a, str, G, local::Error>;

        pub fn get_by_name<'a>(
            name: &'a str
        ) -> GetById<
            'a,
            impl FnMut(&Managers, &str) -> Result<Light, local::Error>,
        > {
            misc::GetById::new(name, getter)
        }

        pub type GetByIds<'a, I, G> =
            misc::GetByIds<'a, I, str, G, local::Error>;

        pub fn get_by_names<'a, I: Iterator<Item = &'a str> + Clone>(
            names: I
        ) -> GetByIds<
            'a,
            I,
            impl FnMut(&Managers, &str) -> Result<Light, local::Error>,
        > {
            misc::GetByIds::new(names, getter)
        }
    }
}

mod misc {
    use super::*;

    pub struct GetById<'a, ID, G, E>
    where ID: ?Sized,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        id: &'a ID,
        getter: G,
        result: Option<Result<Light, E>>
    }

    impl<'a, ID, G, E> GetById<'a, ID, G, E>
    where ID: ?Sized,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        pub fn new(id: &'a ID, getter: G) -> Self {
            Self {
                id,
                getter,
                result: None,
            }
        }
    }

    impl<'a, ID, G, E> Strategy for GetById<'a, ID, G, E>
    where ID: ?Sized,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        fn execute(self: &mut Self, managers: Managers) {
            self.result = Some((self.getter)(&managers, self.id))
        }
    }

    impl<'a, ID, G, E> StrategyResult for GetById<'a, ID, G, E>
    where ID: ?Sized,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        type Result = Result<Light, E>;

        fn result(self: Self) -> Option<Self::Result> {
            self.result
        }
    }

    pub struct GetByIds<'a, I, ID, G, E>
    where I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        ids: I,
        getter: G,
        result: Option<Result<Vec<Light>, E>>
    }

    impl<'a, I, ID, G, E> GetByIds<'a, I, ID, G, E>
    where I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        pub fn new(ids: I, getter: G) -> Self {
            Self {
                ids,
                getter,
                result: None,
            }
        }
    }

    impl<'a, I, ID, G, E> Strategy for GetByIds<'a, I, ID, G, E>
    where I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        fn execute(self: &mut Self, managers: Managers) {
            self.result = Some(
                self.ids.clone()
                    .map(|id| (self.getter)(&managers, id))
                    .try_fold(Vec::new(), |mut acc, light| {
                        light.map(|light| {
                            acc.push(light);
                            acc
                        })
                    })
            )
        }
    }

    impl<'a, I, ID, G, E> StrategyResult for GetByIds<'a, I, ID, G, E>
    where I: Iterator<Item = &'a ID> + Clone,
          ID: ?Sized + 'a,
          G: FnMut(&Managers, &ID) -> Result<Light, E>,
          E: std::error::Error {
        type Result = Result<Vec<Light>, E>;

        fn result(self: Self) -> Option<Self::Result> {
            self.result
        }
    }
}

