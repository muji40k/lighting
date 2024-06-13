
use serde::{Serialize, Deserialize};

use super::misc::Uf64;

pub mod rgb;
pub mod temperature;
pub mod hsv;

#[derive(Debug, Serialize, Deserialize)]
pub struct Color { // Default color in XYZ space
    pub x: Uf64,
    pub y: Uf64,
    pub z: Uf64,
}

impl Color {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Uf64::new(x),
            y: Uf64::new(y),
            z: Uf64::new(z),
        }
    }
}

