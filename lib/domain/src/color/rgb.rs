
use super::Color;

#[derive(Debug)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}

impl From<Color> for RGB {
    fn from(value: Color) -> Self {
        let x: f64 = *value.x;
        let y: f64 = *value.y;
        let z: f64 = *value.z;

        // XYZ to linear sRGB(D65) (http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html)
        let r =  3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
        let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
        let b =  0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

        // linear sRGB to sRGB
        fn nonlinear(v: f64) -> u8 {
            let out = if v.abs() < 0.0031308 {
                12.92 * v
            } else {
                1.055 * v.powf(1f64 / 2.4) - 0.055
            };

            if 0f64 > out {
                0
            } else if 1f64 < out {
                u8::MAX
            } else {
                (u8::MAX as f64 * out).round() as u8
            }
        }

        Self::new(nonlinear(r), nonlinear(g), nonlinear(b))
    }
}

impl From<RGB> for Color {
    fn from(value: RGB) -> Self {
        // sRGB to linear sRGB
        fn linear(v: u8) -> f64 {
            let v = v as f64 / u8::max_value() as f64;

            if 0.04045 >= v {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        }

        let (r, g, b) = (linear(value.red), linear(value.green),
                         linear(value.blue));

        // linear sRGB to XYZ
        let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
        let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
        let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;

        // let x = 0.412390799265960 * r + 0.357584339383878 * g + 0.180480788401834 * b;
        // let y = 0.212639005871510 * r + 0.715168678767756 * g + 0.072192315360734 * b;
        // let z = 0.019330818715592 * r + 0.119194779794626 * g + 0.950532152249661 * b;

        Self::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-5;

    macro_rules! assert_float_eq {
        ($x:expr, $y:expr, $d:expr) => {
            if (($x - $y).abs() > $d) {
                panic!("Left: {}\nRight: {}", $x, $y);
            }
        };
        ($x:expr, $y:expr) => {
            if (($x - $y).abs() > EPS) {
                panic!("Left: {}\nRight: {}", $x, $y);
            }
        };
    }

    mod to_xyz {
        use super::*;

        #[test]
        fn white() {
            let rgb = RGB::new(255, 255, 255);
            let xyz: Color = rgb.into();

            assert_float_eq!(*xyz.x, 0.950470);
            assert_float_eq!(*xyz.y, 1f64);
            assert_float_eq!(*xyz.z, 1.088830);
        }

        #[test]
        fn red() {
            let rgb = RGB::new(255, 0, 0);
            let xyz: Color = rgb.into();

            assert_float_eq!(*xyz.x, 0.412456);
            assert_float_eq!(*xyz.y, 0.212673);
            assert_float_eq!(*xyz.z, 0.019334);
        }

        #[test]
        fn green() {
            let rgb = RGB::new(0, 255, 0);
            let xyz: Color = rgb.into();

            assert_float_eq!(*xyz.x, 0.357576);
            assert_float_eq!(*xyz.y, 0.715152);
            assert_float_eq!(*xyz.z, 0.119192);
        }

        #[test]
        fn blue() {
            let rgb = RGB::new(0, 0, 255);
            let xyz: Color = rgb.into();

            assert_float_eq!(*xyz.x, 0.180437);
            assert_float_eq!(*xyz.y, 0.072175);
            assert_float_eq!(*xyz.z, 0.950304);
        }

        #[test]
        fn black() {
            let rgb = RGB::new(0, 0, 0);
            let xyz: Color = rgb.into();

            assert_float_eq!(*xyz.x, 0f64);
            assert_float_eq!(*xyz.y, 0f64);
            assert_float_eq!(*xyz.z, 0f64);
        }

        #[test]
        fn random1() {
            let rgb = RGB::new(73, 193, 229);
            let xyz: Color = rgb.into();

            assert_float_eq!(*xyz.x, 0.359547);
            assert_float_eq!(*xyz.y, 0.452095);
            assert_float_eq!(*xyz.z, 0.809450);
        }

        #[test]
        fn random2() {
            let rgb = RGB::new(255, 170, 0);
            let xyz: Color = rgb.into();

            assert_float_eq!(*xyz.x, 0.556194);
            assert_float_eq!(*xyz.y, 0.500148);
            assert_float_eq!(*xyz.z, 0.067246);
        }
    }

    mod to_rgb {
        use super::*;

        #[test]
        fn white() {
            let xyz = Color::new(0.950470, 1f64, 1.088830);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 255);
            assert_eq!(rgb.green, 255);
            assert_eq!(rgb.blue, 255);
        }

        #[test]
        fn red() {
            let xyz = Color::new(0.412456, 0.212673, 0.019334);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 255);
            assert_eq!(rgb.green, 0);
            assert_eq!(rgb.blue, 0);
        }

        #[test]
        fn green() {
            let xyz = Color::new(0.357576, 0.715152, 0.119192);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 0);
            assert_eq!(rgb.green, 255);
            assert_eq!(rgb.blue, 0);
        }

        #[test]
        fn blue() {
            let xyz = Color::new(0.180437, 0.072175, 0.950304);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 0);
            assert_eq!(rgb.green, 0);
            assert_eq!(rgb.blue, 255);
        }

        #[test]
        fn black() {
            let xyz = Color::new(0f64, 0f64, 0f64);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 0);
            assert_eq!(rgb.green, 0);
            assert_eq!(rgb.blue, 0);
        }

        #[test]
        fn random1() {
            let xyz = Color::new(0.359547, 0.452095, 0.809450);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 73);
            assert_eq!(rgb.green, 193);
            assert_eq!(rgb.blue, 229);
        }

        #[test]
        fn random2() {
            let xyz = Color::new(0.556194, 0.500148, 0.067246);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 255);
            assert_eq!(rgb.green, 170);
            assert_eq!(rgb.blue, 0);
        }

        #[test]
        fn out_of_bound() {
            let xyz = Color::new(1f64, 1f64, 1f64);
            let rgb: RGB = xyz.into();

            assert_eq!(rgb.red, 255); // 277
            assert_eq!(rgb.green, 249);
            assert_eq!(rgb.blue, 244);
        }
    }
}

// #[derive(Debug)]
// pub struct RGBColor {
//     pub red: u8,
//     pub green: u8,
//     pub blue: u8,
// }
//  
// const EPS: f64 = 1e-8;
//  
// impl RGBColor {
//     pub fn new(red: u8, green: u8, blue: u8) -> Self {
//         Self {
//             red,
//             green,
//             blue,
//         }
//     }
//  
//     fn reduce<F>(self: &Self, mut func: F) -> u8
//     where F: FnMut(u8, u8) -> bool {
//         let mut wrap = |a: u8, b: u8| if func(a, b) { a } else { b };
//  
//         let tmp = wrap(self.red, self.green);
//         wrap(tmp, self.blue)
//     }
// }
//  
// impl Dumpable for RGBColor {
//     fn dump(self: &Self, dumper: &mut dyn dump::Dumper) {
//         self.red.dump_as_parameter(dumper, "red");
//         self.green.dump_as_parameter(dumper, "green");
//         self.blue.dump_as_parameter(dumper, "blue");
//     }
//  
//     fn dump_as_parameter(self: &Self, dumper: &mut dyn dump::Dumper, name: &str) {
//         dumper.dump_fold_as_parameter(name, self);
//     }
// }
//  
// impl InnerColor for RGBColor {
//     fn to_rgb(self: &Self) -> Option<RGB> {
//         Some(RGB {
//             red: self.red,
//             green: self.green,
//             blue: self.blue,
//         })
//     }
//  
//     fn to_hsv(self: &Self) -> Option<HSV> {
//         let max = self.reduce(|a, b| a > b) as f64;
//         let min = self.reduce(|a, b| a < b) as f64;
//         let diff = max - min;
//         let diff6 = 6.0 * diff;
//         let r = self.red as f64;
//         let g = self.green as f64;
//         let b = self.blue as f64;
//  
//         let hue = if diff.abs() < EPS {
//             0.0
//         } else if (max - r).abs() < EPS {
//             if g >= b {
//                 (g - b) / diff6
//             } else {
//                 1.0 - (b - g) / diff6
//             }
//         } else if (max - g).abs() < EPS {
//             1.0 / 3.0 + (b - r) / diff6
//         } else {
//             2.0 / 3.0 + (r - g) / diff6
//         };
//  
//         let saturation = if max.abs() < EPS {
//             0.0
//         } else {
//             diff / max
//         };
//  
//         Some(HSV {
//             hue,
//             saturation,
//             value: max / u8::max_value() as f64,
//         })
//     }
//  
//     // Is it wrong?
//     fn to_temperature(self: &Self) -> Option<Temperature> {
//         // sRGB to linear sRGB
//         fn linear(v: u8) -> f64 {
//             let v = v as f64 / u8::max_value() as f64;
//  
//             if 0.04045 >= v {
//                 v / 12.92
//             } else {
//                 ((v + 0.055) / 1.055).powf(2.4)
//             }
//         }
//  
//         let (r, g, b) = (linear(self.red), linear(self.green), linear(self.blue));
//  
//         // linear sRGB to XYZ
//         let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
//         let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
//         let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;
//  
//         // XYZ to xyz
//         let s = x + y + z;
//         let xc = x / s;
//         let yc = y / s;
//  
//         // xyz to cct
//         let n = (xc - 0.3320) / (0.1858 - yc);
//         Some(449.0 * n.powi(3) + 3525.0 * n.powi(2) + 6823.3 * n + 5520.33)
//     }
// }
//  
// #[cfg(test)]
// mod tests {
//     use super::*;
//  
//     fn float_eq(a: f64, b: f64) -> bool {
//         (a - b).abs() < EPS
//     }
//  
//     mod to_hsv {
//         use super::*;
//  
//         fn construct(r: u8, g: u8, b: u8) -> HSV {
//             let rgb = RGBColor::new(r, g, b);
//             rgb.to_hsv().expect("HSV value should form")
//         }
//  
//         fn check(hsv: HSV, h: f64, s: f64, v: f64) {
//             assert!(float_eq(hsv.hue, h),        "hue:        {} != {}", hsv.hue, h);
//             assert!(float_eq(hsv.saturation, s), "saturation: {} != {}", hsv.saturation, s);
//             assert!(float_eq(hsv.value, v),      "value:      {} != {}", hsv.value, v);
//         }
//  
//         #[test]
//         fn test_red() {
//             check(construct(255, 0, 0),
//                   0.0,
//                   1.0,
//                   1.0);
//         }
//  
//         #[test]
//         fn test_green() {
//             check(construct(0, 255, 0),
//                   1.0 / 3.0,
//                   1.0,
//                   1.0);
//         }
//  
//         #[test]
//         fn test_blue() {
//             check(construct(0, 0, 255),
//                   2.0 / 3.0,
//                   1.0,
//                   1.0);
//         }
//  
//         #[test]
//         fn test_random1() {
//             check(construct(240, 240, 208),
//                   1.0 / 6.0,
//                   4.0 / 30.0,
//                   240.0 / 255.0);
//         }
//  
//         #[test]
//         fn test_random2() {
//             check(construct(51, 92, 33),
//                   (-18.0 / 59.0 + 2.0) / 6.0,
//                   59.0 / 92.0,
//                   92.0 / 255.0);
//         }
//     }
//  
//     mod to_temperature {
//         use super::*;
//  
//         fn construct(r: u8, g: u8, b: u8) -> f64 {
//             let rgb = RGBColor::new(r, g, b);
//             rgb.to_temperature().expect("CCT value should form")
//         }
//  
//         // Pupupupupupupupuupupu..........
//         #[test]
//         fn test_1000() {
//             let temp = construct(255, 56, 0);
//             assert_eq!(temp, 1000.0);
//         }
//  
//         #[test]
//         fn test_random1() {
//             let temp = construct(255, 235, 12);
//             assert_eq!(temp, 3557.10272422);
//         }
//  
//         #[test]
//         fn test_1000_2() {
//             let temp = construct(255, 14, 3);
//             assert_eq!(temp, 1000.0);
//         }
//     }
// }

