
use local_registry::dump::Dumpable;

pub mod rgb;
pub mod temperature;

pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct HSV {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

pub type Temperature = f64;

pub trait InnerColor {
    fn to_rgb(self: &Self) -> Option<RGB>;
    fn to_hsv(self: &Self) -> Option<HSV>;
    fn to_temperature(self: &Self) -> Option<Temperature>;
}

pub trait Color: InnerColor + Dumpable {}
impl<T> Color for T where T: InnerColor + Dumpable {}

