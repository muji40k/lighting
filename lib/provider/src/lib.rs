
use std::error::Error;

use logic::light::Light;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Provider {
    fn name(self: &Self) -> &str;
    fn list(self: &Self) -> Vec<Light>;
    fn get(self: &Self, id: &str) -> Light;
    fn sync(self: &Self, light: &Light) -> Result<()>;
}

