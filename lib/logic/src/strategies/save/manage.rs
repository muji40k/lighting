
use super::{Strategy, StrategyResult};
use crate::facade::Managers;
use crate::managers::local;

pub struct Rename<'a>{
    from: &'a str,
    to: &'a str,
    result: Option<Result<(), local::Error>>,
}

impl<'a> Rename<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Option<Self> {
        if from.is_empty() || to.is_empty() {
            None
        } else {
            Some(Self {
                from,
                to,
                result: None,
            })
        }
    }
}

impl<'a> Strategy for Rename<'a> {
    fn execute(self: &mut Self, managers: Managers) {
        self.result = Some(
            managers.local.rename(self.from, self.to)
        );
    }
}

impl<'a> StrategyResult for Rename<'a> {
    type Result = Result<(), local::Error>;

    fn result(self: Self) -> Option<Self::Result> {
        self.result
    }
}

pub mod delete {
    use super::*;

    pub struct Single<'a>{
        name: &'a str,
        result: Option<Result<(), local::Error>>,
    }

    impl<'a> Single<'a> {
        pub fn new(name: &'a str) -> Option<Self> {
            if name.is_empty() {
                None
            } else {
                Some(Self {
                    name,
                    result: None,
                })
            }
        }
    }

    impl<'a> Strategy for Single<'a> {
        fn execute(self: &mut Self, managers: Managers) {
            self.result = Some(
                managers.local.remove(self.name)
            );
        }
    }

    impl<'a> StrategyResult for Single<'a> {
        type Result = Result<(), local::Error>;

        fn result(self: Self) -> Option<Self::Result> {
            self.result
        }
    }

    pub struct Multiple<'a, I>
    where I: Iterator<Item = &'a str> + Clone {
        names: I,
        result: Option<Result<(), local::Error>>,
    }

    impl<'a, I> Multiple<'a, I>
    where I: Iterator<Item = &'a str> + Clone {
        pub fn new(names: I) -> Option<Self> {
            if names.clone().any(|name| name.is_empty()) {
                None
            } else {
                Some(Self {
                    names,
                    result: None,
                })
            }
        }
    }

    impl<'a, I> Strategy for Multiple<'a, I>
    where I: Iterator<Item = &'a str> + Clone {
        fn execute(self: &mut Self, managers: Managers) {
            self.result = Some(
                self.names.clone()
                    .try_for_each(|name| managers.local.remove(name))
            );
        }
    }

    impl<'a, I> StrategyResult for Multiple<'a, I>
    where I: Iterator<Item = &'a str> + Clone {
        type Result = Result<(), local::Error>;

        fn result(self: Self) -> Option<Self::Result> {
            self.result
        }
    }
}

