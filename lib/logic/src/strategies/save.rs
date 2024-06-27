
use super::{Strategy, StrategyResult};
use crate::managers::{fetch, local};

pub mod dump;
pub mod load_and_save;
pub mod manage;

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

