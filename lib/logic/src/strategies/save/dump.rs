
use domain::light::Light;
use super::{Strategy, StrategyResult};
use crate::facade::Managers;
use crate::managers::local;

pub type Dump<'a, S> = misc::saver::Single<'a, S, local::Error>;
pub type Dumps<'a, I, S> = misc::saver::Multiple<'a, I, S, local::Error>;

fn s_dump(managers: &mut Managers, light: &Light) -> Result<(), local::Error> {
    managers.local.save(light)
}

pub fn dump<'a>(
    light: &'a Light
) -> Option<
    Dump<'a, impl FnMut(&mut Managers, &Light) -> Result<(), local::Error>>
> {
    misc::saver::Single::new(light, s_dump)
}

pub fn dumps<'a, I: Iterator<Item = &'a Light> + Clone>(
    lights: I
) -> Option<
    Dumps<'a, I, impl FnMut(&mut Managers, &Light) -> Result<(), local::Error>>
> {
    misc::saver::Multiple::new(lights, s_dump)
}

pub type Default<'a, S> = misc::saver::Single<'a, S, local::Error>;
pub type Defaults<'a, I, S> = misc::saver::Multiple<'a, I, S, local::Error>;

fn s_default(managers: &mut Managers, light: &Light) -> Result<(), local::Error> {
    managers.local.set_default(light)
}

pub fn default<'a>(
    light: &'a Light
) -> Option<
    Default<'a, impl FnMut(&mut Managers, &Light) -> Result<(), local::Error>>
> {
    misc::saver::Single::new(light, s_default)
}

pub fn defaults<'a, I: Iterator<Item = &'a Light> + Clone>(
    lights: I
) -> Option<
    Defaults<'a, I, impl FnMut(&mut Managers, &Light) -> Result<(), local::Error>>
> {
    misc::saver::Multiple::new(lights, s_default)
}

pub type Save<'a, S> = misc::saver::Single<'a, S, local::Error>;
pub type Saves<'a, I, S> = misc::saver::Multiple<'a, I, S, local::Error>;

fn s_save(managers: &mut Managers, light: &Light) -> Result<(), local::Error> {
    managers.local.save(light).and_then(|_| managers.local.set_default(light))
}

pub fn save<'a>(
    light: &'a Light
) -> Option<
    Save<'a, impl FnMut(&mut Managers, &Light) -> Result<(), local::Error>>
> {
    misc::saver::Single::new(light, s_save)
}

pub fn saves<'a, I: Iterator<Item = &'a Light> + Clone>(
    lights: I
) -> Option<
    Saves<'a, I, impl FnMut(&mut Managers, &Light) -> Result<(), local::Error>>
> {
    misc::saver::Multiple::new(lights, s_save)
}

mod misc {
    use super::*;

    pub mod saver {
        use super::*;

        pub struct Single<'a, S, E>
        where S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            light: &'a Light,
            saver: S,
            result: Option<Result<(), E>>,
        }

        impl<'a, S, E> Single<'a, S, E>
        where S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            pub fn new(light: &'a Light, saver: S) -> Option<Self> {
                if light.name.is_empty() {
                    None
                } else {
                    Some(Self {
                        light,
                        saver,
                        result: None,
                    })
                }
            }
        }

        impl<'a, S, E> Strategy for Single<'a, S, E>
        where S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            fn execute(self: &mut Self, mut managers: Managers) {
                self.result = Some(
                    (self.saver)(&mut managers, self.light)
                )
            }
        }

        impl<'a, S, E> StrategyResult for Single<'a, S, E>
        where S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            type Result = Result<(), E>;

            fn result(self: Self) -> Option<Self::Result> {
                self.result
            }
        }

        pub struct Multiple<'a, I, S, E>
        where I: Iterator<Item = &'a Light> + Clone,
              S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            lights: I,
            saver: S,
            result: Option<Result<(), E>>,
        }

        impl<'a, I, S, E> Multiple<'a, I, S, E>
        where I: Iterator<Item = &'a Light> + Clone,
              S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            pub fn new(lights: I, saver: S) -> Option<Self> {
                if lights.clone().any(|light| light.name.is_empty()) {
                    None
                } else {
                    Some(Self {
                        lights,
                        saver,
                        result: None,
                    })
                }
            }
        }

        impl<'a, I, S, E> Strategy for Multiple<'a, I, S, E>
        where I: Iterator<Item = &'a Light> + Clone,
              S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            fn execute(self: &mut Self, mut managers: Managers) {
                self.result = Some(
                    self.lights.clone()
                        .try_for_each(|light| {
                            (self.saver)(&mut managers, light)
                        })
                )
            }
        }

        impl<'a, I, S, E> StrategyResult for Multiple<'a, I, S, E>
        where I: Iterator<Item = &'a Light> + Clone,
              S: FnMut(&mut Managers, &Light) -> Result<(), E>,
              E: std::error::Error {
            type Result = Result<(), E>;

            fn result(self: Self) -> Option<Self::Result> {
                self.result
            }
        }
    }
}

