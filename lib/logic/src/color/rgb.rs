
use super::*;

#[derive(Debug)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

const EPS: f64 = 1e-8;

impl RGBColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }

    fn reduce<F>(self: &Self, mut func: F) -> u8
    where F: FnMut(u8, u8) -> bool {
        let mut wrap = |a: u8, b: u8| if func(a, b) { a } else { b };

        let tmp = wrap(self.red, self.green);
        wrap(tmp, self.blue)
    }
}

impl Dumpable for RGBColor {
    fn dump(self: &Self, dumper: &mut dyn local_registry::dump::Dumper) {
        self.red.dump_as_parameter(dumper, "red");
        self.green.dump_as_parameter(dumper, "green");
        self.blue.dump_as_parameter(dumper, "blue");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn local_registry::dump::Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl InnerColor for RGBColor {
    fn to_rgb(self: &Self) -> Option<RGB> {
        Some(RGB {
            red: self.red,
            green: self.green,
            blue: self.blue,
        })
    }

    fn to_hsv(self: &Self) -> Option<HSV> {
        let max = self.reduce(|a, b| a > b) as f64;
        let min = self.reduce(|a, b| a < b) as f64;
        let diff = max - min;
        let diff6 = 6.0 * diff;
        let r = self.red as f64;
        let g = self.green as f64;
        let b = self.blue as f64;

        let hue = if diff.abs() < EPS {
            0.0
        } else if (max - r).abs() < EPS {
            if g >= b {
                (g - b) / diff6
            } else {
                1.0 - (b - g) / diff6
            }
        } else if (max - g).abs() < EPS {
            1.0 / 3.0 + (b - r) / diff6
        } else {
            2.0 / 3.0 + (r - g) / diff6
        };

        let saturation = if max.abs() < EPS {
            0.0
        } else {
            diff / max
        };

        Some(HSV {
            hue,
            saturation,
            value: max / u8::max_value() as f64,
        })
    }

    // Is it wrong?
    fn to_temperature(self: &Self) -> Option<Temperature> {
        // sRGB to linear sRGB
        fn linear(v: u8) -> f64 {
            let v = v as f64 / u8::max_value() as f64;

            if 0.04045 >= v {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        }

        let (r, g, b) = (linear(self.red), linear(self.green), linear(self.blue));

        // linear sRGB to XYZ
        let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
        let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
        let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;

        // XYZ to xyz
        let s = x + y + z;
        let xc = x / s;
        let yc = y / s;

        // xyz to cct
        let n = (xc - 0.3320) / (0.1858 - yc);
        Some(449.0 * n.powi(3) + 3525.0 * n.powi(2) + 6823.3 * n + 5520.33)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn float_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    mod to_hsv {
        use super::*;

        fn construct(r: u8, g: u8, b: u8) -> HSV {
            let rgb = RGBColor::new(r, g, b);
            rgb.to_hsv().expect("HSV value should form")
        }

        fn check(hsv: HSV, h: f64, s: f64, v: f64) {
            assert!(float_eq(hsv.hue, h),        "hue:        {} != {}", hsv.hue, h);
            assert!(float_eq(hsv.saturation, s), "saturation: {} != {}", hsv.saturation, s);
            assert!(float_eq(hsv.value, v),      "value:      {} != {}", hsv.value, v);
        }

        #[test]
        fn test_red() {
            check(construct(255, 0, 0),
                  0.0,
                  1.0,
                  1.0);
        }

        #[test]
        fn test_green() {
            check(construct(0, 255, 0),
                  1.0 / 3.0,
                  1.0,
                  1.0);
        }

        #[test]
        fn test_blue() {
            check(construct(0, 0, 255),
                  2.0 / 3.0,
                  1.0,
                  1.0);
        }

        #[test]
        fn test_random1() {
            check(construct(240, 240, 208),
                  1.0 / 6.0,
                  4.0 / 30.0,
                  240.0 / 255.0);
        }

        #[test]
        fn test_random2() {
            check(construct(51, 92, 33),
                  (-18.0 / 59.0 + 2.0) / 6.0,
                  59.0 / 92.0,
                  92.0 / 255.0);
        }
    }

    mod to_temperature {
        use super::*;

        fn construct(r: u8, g: u8, b: u8) -> f64 {
            let rgb = RGBColor::new(r, g, b);
            rgb.to_temperature().expect("CCT value should form")
        }

        // Pupupupupupupupuupupu..........
        #[test]
        fn test_1000() {
            let temp = construct(255, 56, 0);
            assert_eq!(temp, 1000.0);
        }

        #[test]
        fn test_random1() {
            let temp = construct(255, 235, 12);
            assert_eq!(temp, 3557.10272422);
        }

        #[test]
        fn test_1000_2() {
            let temp = construct(255, 14, 3);
            assert_eq!(temp, 1000.0);
        }
    }
}

