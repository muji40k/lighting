
use serde::{Serialize, Deserialize};

use crate::color::Color;
use crate::mode::Mode;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Incapable(String, ProviderID, Capability),
    Unset(String, ProviderID, Capability),
    UnsuitableMode(String, ProviderID, String),
}

impl Error {
    fn incapable<T>(light: &Light, capability: Capability) -> Result<T> {
        Err(Error::Incapable(String::from(light.name.clone()),
                             light.provider.clone(),
                             capability))
    }

    fn unset<T>(light: &Light, capability: Capability) -> Result<T> {
        Err(Error::Unset(String::from(light.name.clone()),
                         light.provider.clone(),
                         capability))
    }

    fn unsuitable_mode<T>(light: &Light, wrong: String) -> Result<T> {
        Err(Error::UnsuitableMode(String::from(light.name.clone()),
                                  light.provider.clone(),
                                  wrong))
    }

}

impl std::fmt::Display for Error {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Incapable(name, provider, capability) => {
                write!(f, "Light \"{}\" ({}): incapable for \"{}\"",
                       name, provider, capability)
            },
            Self::Unset(name, provider, capability) => {
                write!(f, "Light \"{}\" ({}): value for \"{}\" unset",
                       name, provider, capability)
            },
            Self::UnsuitableMode(name, provider, wrong) => {
                write!(f, "Light \"{}\" ({}): attempt to set mode from another provider ({})",
                       name, provider, wrong)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderID {
    pub name: String, // Provider name
    pub id: String,   // Light id for provider
}

impl ProviderID {
    fn new(name: String, id: String) -> Self {
        Self {
            name,
            id,
        }
    }
}

impl std::fmt::Display for ProviderID {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}@{}", self.id, self.name)
    }
}

#[derive(Debug)]
pub enum Capability {
    Color,
    Brightness,
    Mode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum State {
    Color(Option<Color>),
    Brightness(Option<Brightness>),
    Mode(Option<Mode>),
}

fn cmp_capabilities(state: &State, capability: &Capability) -> bool {
    match (state, capability) {
        (State::Color(_), Capability::Color) => true,
        (State::Brightness(_), Capability::Brightness) => true,
        (State::Mode(_), Capability::Mode) => true,
        _ => false,
    }
}

impl From<&Capability> for State {
    fn from(value: &Capability) -> Self {
        match value {
            Capability::Color => Self::Color(None),
            Capability::Brightness => Self::Brightness(None),
            Capability::Mode => Self::Mode(None),
        }
    }
}

impl<'a> FromIterator<&'a Capability> for Vec<State> {
    fn from_iter<T: IntoIterator<Item = &'a Capability>>(iter: T) -> Self {
        let mut out = Self::new();

        for item in iter {
            out.push(item.into());
        }

        out
    }
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Capability::Color => write!(f, "Color"),
            Capability::Brightness => write!(f, "Brightness"),
            Capability::Mode => write!(f, "Mode"),
        }
    }
}

pub type Brightness = f64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Light {
    pub provider: ProviderID, // Light id for Provider
    pub name: String,         // Local name
    pub power: bool,
    state: Vec<State>,
}

impl Light {
    pub fn new(provider: String, provider_id: String,
               capabilities: Vec<Capability>) -> Self {
        Self {
            provider: ProviderID::new(provider, provider_id),
            name: String::new(),
            power: false,
            state: Vec::from_iter(capabilities.iter()),
        }
    }

    pub fn named(provider: String, provider_id: String,
                 capabilities: Vec<Capability>,
                 name: String) -> Self {
        Self {
            provider: ProviderID::new(provider, provider_id),
            name,
            power: false,
            state: Vec::from_iter(capabilities.iter()),
        }
    }

    pub fn is_capable(self: &Self, checked: &[Capability]) -> bool {
        checked.iter().all(|c| {
            self.state.iter().any(|item| cmp_capabilities(item, c))
        })
    }

    pub fn get_color(self: &Self) -> Result<&Color> {
        if let Some(State::Color(color)) = self.state.iter().find(|item| {
            match item {
                State::Color(_) => true,
                _ => false,
            }
        }) {
            if let Some(color) = color {
                Ok(color)
            } else {
                Error::unset(self, Capability::Color)
            }
        } else {
            Error::incapable(self, Capability::Color)
        }
    }

    pub fn set_color(self: &mut Self, color: Color) -> Result<()> {
        if let Some(State::Color(in_color)) = self.state.iter_mut().find(|item| {
            match item {
                State::Color(_) => true,
                _ => false,
            }
        }) {
            *in_color = Some(color);
            Ok(())
        } else {
            Error::incapable(self, Capability::Color)
        }
    }

    pub fn get_brightness(self: &Self) -> Result<&Brightness> {
        if let Some(State::Brightness(brightness)) = self.state.iter().find(|item| {
            match item {
                State::Brightness(_) => true,
                _ => false,
            }
        }) {
            if let Some(brightness) = brightness {
                Ok(brightness)
            } else {
                Error::unset(self, Capability::Brightness)
            }
        } else {
            Error::incapable(self, Capability::Brightness)
        }
    }

    pub fn set_brightness(self: &mut Self, brightness: Brightness) -> Result<()> {
        if let Some(State::Brightness(in_brightness)) = self.state.iter_mut().find(|item| {
            match item {
                State::Brightness(_) => true,
                _ => false,
            }
        }) {
            *in_brightness = Some(brightness);
            Ok(())
        } else {
            Error::incapable(self, Capability::Brightness)
        }
    }

    pub fn get_mode(self: &Self) -> Result<&Mode> {
        if let Some(State::Mode(mode)) = self.state.iter().find(|item| {
            match item {
                State::Mode(_) => true,
                _ => false,
            }
        }) {
            if let Some(mode) = mode {
                Ok(mode)
            } else {
                Error::unset(self, Capability::Mode)
            }
        } else {
            Error::incapable(self, Capability::Mode)
        }
    }

    pub fn set_mode(self: &mut Self, mode: Mode) -> Result<()> {
        if let Some(State::Mode(in_mode)) = self.state.iter_mut().find(|item| {
            match item {
                State::Mode(_) => true,
                _ => false,
            }
        }) {
            if mode.provider == self.provider.name {
                *in_mode = Some(mode);
                Ok(())
            } else {
                Error::unsuitable_mode(self, mode.provider)
            }
        } else {
            Error::incapable(self, Capability::Mode)
        }
    }
}

