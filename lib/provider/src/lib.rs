
use logic::light::Light;

pub trait Provider {
    fn name(self: &Self) -> &str;
    fn list(self: &Self) -> Vec<Light>;
    fn get(self: &Self, id: &str) -> Light;
    fn sync(self: &Self, light: &Light) -> Result<(), String>;
}

