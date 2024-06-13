
use super::Color;
use super::rgb::RGB;

use crate::misc::Nf64;

#[derive(Debug)]
pub struct HSV {
    pub hue: Nf64,
    pub saturation: Nf64,
    pub value: Nf64,
}

impl HSV {
    pub fn new(hue: f64, saturation: f64, value: f64) -> Self {
        Self {
            hue: Nf64::new(hue),
            saturation: Nf64::new(saturation),
            value: Nf64::new(value),
        }
    }
}

impl From<Color> for HSV {
    fn from(value: Color) -> Self {
        Self::from(RGB::from(value))
    }
}

impl From<HSV> for Color {
    fn from(value: HSV) -> Self {
        Self::from(RGB::from(value))
    }
}

fn reduce<F: Fn(u8, u8) -> u8>(rgb: &RGB, f: F) -> u8 {
    f(f(rgb.red, rgb.green), rgb.blue)
}

impl From<RGB> for HSV {
    fn from(value: RGB) -> Self {
        let max = reduce(&value, u8::max) as f64;
        let min = reduce(&value, u8::min) as f64;
        let diff = max - min;
        let diff6 = 6.0 * diff;
        let r = value.red as f64;
        let g = value.green as f64;
        let b = value.blue as f64;

        let hue = if diff.abs() < f64::EPSILON {
            0.0
        } else if (max - r).abs() < f64::EPSILON {
            if g >= b {
                (g - b) / diff6
            } else {
                1.0 - (b - g) / diff6
            }
        } else if (max - g).abs() < f64::EPSILON {
            1.0 / 3.0 + (b - r) / diff6
        } else {
            2.0 / 3.0 + (r - g) / diff6
        };

        let saturation = if max.abs() < f64::EPSILON {
            0.0
        } else {
            diff / max
        };

        Self::new(
            hue,
            saturation,
            max / (u8::max_value() as f64),
        )
    }
}

impl From<HSV> for RGB {
    fn from(value: HSV) -> Self {
        let max = *value.value * (u8::max_value() as f64);
        let diff = *value.saturation * max;
        let diff6 = 6.0 * diff;

        let r: f64;
        let g: f64;
        let b: f64;

        if diff.abs() < f64::EPSILON {
            r = max;
            g = max;
            b = max;
        } else if 1f64 / 6f64 > *value.hue {
            r = max;
            b = r - diff;
            g = b + *value.hue * diff6;
        } else if 0.5 > *value.hue {
            g = max;
            let min = g - diff;
            let diff = (*value.hue - 1.0 / 3.0) * diff6;

            if 0.0 <= diff {
                r = min;
                b = r + diff;
            } else {
                b = min;
                r = b - diff;
            }
        } else if 5f64 / 6f64 > *value.hue {
            b = max;
            let min = b - diff;
            let diff = (*value.hue - 2.0 / 3.0) * diff6;

            if 0.0 <= diff {
                g = min;
                r = g + diff;
            } else {
                r = min;
                g = r - diff;
            }
        } else {
            r = max;
            g = r - diff;
            b = g - (*value.hue - 1.0) * diff6;
        }

        Self::new(
            r.round() as u8,
            g.round() as u8,
            b.round() as u8
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_xyz {
        use super::*;
        use crate::assert_float_eq;

        fn check(hsv: HSV, values: (f64, f64, f64)) {
            let xyz: Color = hsv.into();

            assert_float_eq!(*xyz.x, values.0);
            assert_float_eq!(*xyz.y, values.1);
            assert_float_eq!(*xyz.z, values.2);
        }

        #[test]
        fn white() {
            check(
                HSV::new(0.0, 0.0, 1.0),
                (0.950470, 1f64, 1.088830)
            );
        }

        #[test]
        fn red() {
            check(
                HSV::new(0.0, 1.0, 1.0),
                (0.412456, 0.212673, 0.019334)
            );
        }

        #[test]
        fn green() {
            check(
                HSV::new(1.0 / 3.0, 1.0, 1.0),
                (0.357576, 0.715152, 0.119192)
            );
        }

        #[test]
        fn blue() {
            check(
                HSV::new(2.0 / 3.0, 1.0, 1.0),
                (0.180437, 0.072175, 0.950304)
            );
        }

        #[test]
        fn black() {
            check(
                HSV::new(0.0, 0.0, 0.0),
                (0f64, 0f64, 0f64)
            );
        }

        #[test]
        fn random1() {
            check(
                HSV::new(0.538462, 0.681223, 0.898039),
                (0.359547, 0.452095, 0.809450)
            );
        }

        #[test]
        fn random2() {
            check(
                HSV::new(1.0 / 9.0, 1.0, 1.0),
                (0.556194, 0.500148, 0.067246)
            );
        }

        #[test]
        fn frac16b() {
            check(
                HSV::new(1.0 / 360.0, 1.0, 1.0),
                (0.412891, 0.213541, 0.019479)
            );
        }

        #[test]
        fn frac16e() {
            check(
                HSV::new(59.0 / 360.0, 1.0, 1.0),
                (0.757405, 0.902570, 0.134317)
            );
        }

        #[test]
        fn frac26b() {
            check(
                HSV::new(61.0 / 360.0, 1.0, 1.0),
                (0.755467, 0.920315, 0.137843)
            );
        }

        #[test]
        fn frac26e() {
            check(
                HSV::new(119.0 / 360.0, 1.0, 1.0),
                (0.358077, 0.715410, 0.119215)
            );
        }

        #[test]
        fn frac36b() {
            check(
                HSV::new(121.0 / 360.0, 1.0, 1.0),
                (0.357795, 0.715240, 0.120346)
            );
        }

        #[test]
        fn frac36e() {
            check(
                HSV::new(179.0 / 360.0, 1.0, 1.0),
                (0.531642, 0.784778, 1.035937)
            );
        }

        #[test]
        fn frac46b() {
            check(
                HSV::new(181.0 / 360.0, 1.0, 1.0),
                (0.525386, 0.762072, 1.065287)
            );
        }

        #[test]
        fn frac46e() {
            check(
                HSV::new(239.0 / 360.0, 1.0, 1.0),
                (0.180872, 0.073043, 0.950449)
            );
        }

        #[test]
        fn frac56b() {
            check(
                HSV::new(241.0 / 360.0, 1.0, 1.0),
                (0.180938, 0.072433, 0.950328)
            );
        }

        #[test]
        fn frac56e() {
            check(
                HSV::new(299.0 / 360.0, 1.0, 1.0),
                (0.578329, 0.277338, 0.968955)
            );
        }

        #[test]
        fn frac66b() {
            check(
                HSV::new(301.0 / 360.0, 1.0, 1.0),
                (0.586522, 0.282299, 0.936079)
            );
        }

        #[test]
        fn frac66e() {
            check(
                HSV::new(359.0 / 360.0, 1.0, 1.0),
                (0.412676, 0.212760, 0.020488)
            );
        }
    }

    mod to_hsv {
        use super::*;
        use crate::assert_float_eq;

        fn check(xyz: Color, values: (f64, f64, f64)) {
            let hsv: HSV = xyz.into();

            assert_float_eq!(*hsv.hue, values.0, 1e-3);
            assert_float_eq!(*hsv.saturation, values.1, 1e-3);
            assert_float_eq!(*hsv.value, values.2, 1e-3);
        }

        #[test]
        fn white() {
            check(
                Color::new(0.950470, 1f64, 1.088830),
                (0.0, 0.0, 1.0)
            );
        }

        #[test]
        fn red() {
            check(
                Color::new(0.412456, 0.212673, 0.019334),
                (0.0, 1.0, 1.0)
            );
        }

        #[test]
        fn green() {
            check(
                Color::new(0.357576, 0.715152, 0.119192),
                (1.0 / 3.0, 1.0, 1.0)
            );
        }

        #[test]
        fn blue() {
            check(
                Color::new(0.180437, 0.072175, 0.950304),
                (2.0 / 3.0, 1.0, 1.0)
            );
        }

        #[test]
        fn black() {
            check(
                Color::new(0f64, 0f64, 0f64),
                (0.0, 0.0, 0.0)
            );
        }

        #[test]
        fn random1() {
            check(
                Color::new(0.359547, 0.452095, 0.809450),
                (0.538462, 0.681223, 0.898039)
            );
        }

        #[test]
        fn random2() {
            check(
                Color::new(0.556194, 0.500148, 0.067246),
                (1.0 / 9.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac16b() {
            check(
                Color::new(0.412891, 0.213541, 0.019479),
                (1.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac16e() {
            check(
                Color::new(0.757405, 0.902570, 0.134317),
                (59.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac26b() {
            check(
                Color::new(0.755467, 0.920315, 0.137843),
                (61.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac26e() {
            check(
                Color::new(0.358077, 0.715410, 0.119215),
                (119.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac36b() {
            check(
                Color::new(0.357795, 0.715240, 0.120346),
                (121.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac36e() {
            check(
                Color::new(0.531642, 0.784778, 1.035937),
                (179.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac46b() {
            check(
                Color::new(0.525386, 0.762072, 1.065287),
                (181.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac46e() {
            check(
                Color::new(0.180872, 0.073043, 0.950449),
                (239.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac56b() {
            check(
                Color::new(0.180938, 0.072433, 0.950328),
                (241.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac56e() {
            check(
                Color::new(0.578329, 0.277338, 0.968955),
                (299.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac66b() {
            check(
                Color::new(0.586522, 0.282299, 0.936079),
                (301.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn frac66e() {
            check(
                Color::new(0.412676, 0.212760, 0.020488),
                (359.0 / 360.0, 1.0, 1.0)
            );
        }

        #[test]
        fn out_of_bound() {
            check(
                Color::new(1f64, 1f64, 1f64),
                (0.075758, 0.043137, 1.0)
            );
        }
    }
}

