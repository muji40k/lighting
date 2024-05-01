
use local_registry::dump::{Dumper, Dumpable};
use crate::color::Color;
use crate::mode::Mode;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Incapable(String, ProviderID, Capability),
    Unset(String, ProviderID, Capability),
}

impl Error {
    fn incapable<T>(light: &Light, capability: Capability) -> Result<T> {
        Err(Error::Incapable(String::from(light.get_name()),
                             light.provider().clone(),
                             capability))
    }

    fn unset<T>(light: &Light, capability: Capability) -> Result<T> {
        Err(Error::Unset(String::from(light.get_name()),
                         light.provider().clone(),
                         capability))
    }
}

impl std::fmt::Display for Error {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Incapable(name, provider, capability) => {
                write!(f, "Light {} ({}[{}]): incapable for \"{}\"", name,
                       provider.id(), provider.provider(), capability)
            },
            Self::Unset(name, provider, capability) => {
                write!(f, "Light {} ({}[{}]): value for \"{}\" unset", name,
                       provider.id(), provider.provider(), capability)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProviderID {
    name: String,
    id: String,
}

impl ProviderID {
    fn new(name: String, id: String) -> Self {
        Self {
            name,
            id,
        }
    }

    pub fn provider(self: &Self) -> &str {
        &self.name
    }

    pub fn id(self: &Self) -> &str {
        &self.id
    }
}

#[derive(Debug)]
pub enum Capability {
    Color,
    Brightness,
    Mode,
}

pub enum State {
    Color(Option<Box<dyn Color>>),
    Brightness(Option<Brightness>),
    Mode(Option<Mode>),
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Capability::Color => "color",
            Capability::Brightness => "brightness",
            Capability::Mode => "mode",
        };

        write!(f, "{}", msg)
    }
}

pub type Brightness = f64;

pub struct Light {
    provider: ProviderID,
    name: String,

    power: bool,
    state: Vec<State>,
}

impl Light {
    fn capability_to_state(capabilities: Vec<Capability>) -> Vec<State> {
        capabilities.iter().map(|item| {
            match item {
                Capability::Color => State::Color(None),
                Capability::Brightness => State::Brightness(None),
                Capability::Mode => State::Mode(None),
            }
        }).collect()
    }

    pub fn new(provider: String, provider_id: String, capabilities: Vec<Capability>) -> Self {
        Self {
            provider: ProviderID::new(provider, provider_id),
            name: String::new(),
            // capabilities,
            power: false,
            // state: None,
            state: Light::capability_to_state(capabilities),
        }
    }

    pub fn named(provider: String, provider_id: String, capabilities: Vec<Capability>,
             name: String) -> Self {
        Self {
            provider: ProviderID::new(provider, provider_id),
            name,
            // capabilities,
            power: false,
            // state: None,
            state: Light::capability_to_state(capabilities),
        }
    }

    pub fn provider(self: &Self) -> &ProviderID {
        &self.provider
    }

    pub fn get_name(self: &Self) -> &str {
        &self.name
    }

    pub fn set_name(self: &mut Self, name: &str) {
        self.name = String::from(name);
    }

    pub fn power(self: &Self) -> bool {
        self.power
    }

    pub fn turn(self: &mut Self, state: bool) {
        self.power = state;
    }

    fn cmp_capabilities(state: &State, capability: &Capability) -> bool {
        match (state, capability) {
            (State::Color(_), Capability::Color) => true,
            (State::Brightness(_), Capability::Brightness) => true,
            (State::Mode(_), Capability::Mode) => true,
            _ => false,
        }
    }

    pub fn is_capable(self: &Self, checked: &[Capability]) -> bool {
        checked.iter().all(|c| {
            self.state.iter().any(|item| Light::cmp_capabilities(item, c))
        })
    }

    pub fn get_color(self: &Self) -> Result<&dyn Color> {
        if let Some(State::Color(color)) = self.state.iter().find(|item| {
            match item {
                State::Color(_) => true,
                _ => false,
            }
        }) {
            if let Some(color) = color {
                Ok(color.as_ref())
            } else {
                Error::unset(self, Capability::Color)
            }
        } else {
            Error::incapable(self, Capability::Color)
        }
    }

    pub fn set_color(self: &mut Self, color: Box<dyn Color>) -> Result<()> {
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
            *in_mode = Some(mode);
            Ok(())
        } else {
            Error::incapable(self, Capability::Mode)
        }
    }
}

impl Dumpable for ProviderID {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.name.dump_as_parameter(dumper, "name");
        self.id.dump_as_parameter(dumper, "id");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl Dumpable for Capability {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        match self {
            Self::Color => "color",
            Self::Brightness => "brightness",
            Self::Mode => "mode",
        }.dump(dumper);
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl Dumpable for State {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        match self {
            State::Color(_) => "color",
            State::Brightness(_) => "brightness",
            State::Mode(_) => "mode",
        }.dump_as_parameter(dumper, "type");

        match self {
            State::Color(color) => color as &dyn Dumpable,
            State::Brightness(value) => value as &dyn Dumpable,
            State::Mode(mode) => mode as &dyn Dumpable,
        }.dump_as_parameter(dumper, "value");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl Dumpable for Light {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.provider.dump_as_parameter(dumper, "provider");
        self.name.dump_as_parameter(dumper, "name");
        self.state.dump_as_parameter(dumper, "state");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

