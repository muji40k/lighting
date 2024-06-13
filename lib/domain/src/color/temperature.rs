
use super::Color;

use crate::misc::Uf64;

pub type Temperature = Uf64;

impl From<Color> for Temperature {
    fn from(value: Color) -> Self {
        // XYZ to xy
        let s = *value.x + *value.y + *value.z;
        let xc = *value.x / s;
        let yc = *value.y / s;

        // xyz to cct
        let n = (xc - 0.3320) / (0.1858 - yc);
        Self::new(449.0 * n.powi(3) + 3525.0 * n.powi(2) + 6823.3 * n + 5520.33)
    }
}

impl From<Temperature> for Color {
    fn from(_value: Temperature) -> Self {
        Self::new(1.009794293297943, 1.0, 0.6444857332448575)
    }
}

