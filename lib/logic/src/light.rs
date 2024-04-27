
use local_registry::dump::{Dumper, Dumpable};
use crate::color::Color;
use crate::mode::Mode;

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

pub enum Capability {
    Color,
    Brightness,
    Mode,
}

pub type Brightness = f64;

enum State {
    Color(Box<dyn Color>),
    Brightness(Brightness),
    Mode(Mode),
    Composite(Vec<Box<State>>),
}

pub struct Light {
    provider: ProviderID,
    name: String,

    capabilities: Vec<Capability>,

    power: bool,
    state: Option<State>,
}

impl Light {
    fn new(provider: String, provider_id: String, capabilities: Vec<Capability>) -> Self {
        Self {
            provider: ProviderID::new(provider, provider_id),
            name: String::new(),
            capabilities,
            power: false,
            state: None,
        }
    }

    fn named(provider: String, provider_id: String, capabilities: Vec<Capability>,
             name: String) -> Self {
        Self {
            provider: ProviderID::new(provider, provider_id),
            name,
            capabilities,
            power: false,
            state: None,
        }
    }

    fn provider(self: &Self) -> &ProviderID {
        &self.provider
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

impl Dumpable for Box<State> {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.as_ref().dump(dumper)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self.as_ref());
    }
}

impl Dumpable for State {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        match self {
            State::Color(_) => "color",
            State::Brightness(_) => "brightness",
            State::Mode(_) => "mode",
            State::Composite(_) => "composite",
        }.dump_as_parameter(dumper, "type");

        match self {
            State::Color(color) => color.dump_as_parameter(dumper, "value"), // upcasting...
            State::Brightness(value) => value.dump_as_parameter(dumper, "value"),
            State::Mode(mode) => mode.dump_as_parameter(dumper, "value"),
            State::Composite(arr) => arr.dump_as_parameter(dumper, "value"),
        }
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl Dumpable for Light {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.provider.dump_as_parameter(dumper, "provider");
        self.provider_id.dump_as_parameter(dumper, "provider_id");
        self.name.dump_as_parameter(dumper, "name");
        self.capabilities.dump_as_parameter(dumper, "capabilities");
        self.state.dump_as_parameter(dumper, "state");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}


