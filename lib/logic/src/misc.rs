
use std::marker::PhantomData;
use core::ops::Deref;

use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[macro_export]
macro_rules! assert_float_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) {
            panic!("Left: {}\nRight: {}", $x, $y);
        }
    };
    ($x:expr, $y:expr) => {
        if (($x - $y).abs() > 1e-5) {
            panic!("Left: {}\nRight: {}", $x, $y);
        }
    };
}

pub trait FloatChecker: {
    fn check(value: f64) -> (Option<f64>, f64);
}

pub type Nf64 = ConstraintedF64<Normalized>;
pub type Uf64 = ConstraintedF64<Unsigned>;

#[derive(Debug)]
pub struct ConstraintedF64<C: FloatChecker> (
    f64,
    PhantomData<C>,
);

impl<C: FloatChecker> ConstraintedF64<C> {
    fn apply_check(value: f64) -> f64 {
        let res = C::check(value);

        if let Some(value) = res.0 {
            value
        } else {
            res.1
        }
    }

    pub fn new(value: f64) -> Self {
        Self(Self::apply_check(value), PhantomData::default())
    }
}

impl<C> Deref for ConstraintedF64<C>
where C: FloatChecker {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C> AsRef<f64> for ConstraintedF64<C>
where C: FloatChecker {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl<C> Serialize for ConstraintedF64<C>
where C: FloatChecker {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        self.0.serialize(serializer)
    }
}

impl<'de, C> Deserialize<'de> for ConstraintedF64<C>
where C: FloatChecker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        f64::deserialize(deserializer).and_then(|value| Ok(Self::new(value)))
    }
}

#[derive(Debug)]
pub struct Normalized();
impl FloatChecker for Normalized {
    fn check(value: f64) -> (Option<f64>, f64) {
        if 1f64 + f64::EPSILON < value {
            (None, 1f64)
        } else if 0f64 - f64::EPSILON > value {
            (None, 0f64)
        } else {
            (Some(value), value)
        }
    }
}

#[derive(Debug)]
pub struct Unsigned();
impl FloatChecker for Unsigned {
    fn check(value: f64) -> (Option<f64>, f64) {
        if 0f64 > value {
            (None, 0f64)
        } else {
            (Some(value), value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{size_of, align_of};

    mod unsigned {
        use super::*;

        #[test]
        fn size() {
            assert_eq!(size_of::<Uf64>(), size_of::<f64>());
        }

        #[test]
        fn align() {
            assert_eq!(align_of::<Uf64>(), align_of::<f64>());
        }

        mod normal {
            use super::*;

            #[test]
            fn basic() {
                let number = Uf64::new(0.5);
                assert_eq!(*number, 0.5);
            }

            #[test]
            fn above_one() {
                let number = Uf64::new(1.5);
                assert_eq!(*number, 1.5);
            }

            #[test]
            fn below_zero() {
                let number = Uf64::new(-0.5);
                assert_eq!(*number, 0f64);
            }
        }

        mod serialization {
            use super::*;

            #[test]
            fn basic_json() {
                let number = Uf64::new(0.5);
                assert_eq!(
                    serde_json::to_string(&number).expect("Should be parsed"),
                    "0.5"
                );
            }

            #[test]
            fn above_one_json() {
                let number = Uf64::new(1.5);
                assert_eq!(
                    serde_json::to_string(&number).expect("Should be parsed"),
                    "1.5"
                );
            }

            #[test]
            fn below_zero_json() {
                let number = Uf64::new(-0.5);
                assert_eq!(
                    serde_json::to_string(&number).expect("Should be parsed"),
                    "0.0"
                );
            }
        }

        mod deserialization {
            use super::*;

            #[test]
            fn basic_json() {
                let value = "0.5";
                let number: Uf64 = serde_json::from_str(value)
                    .expect("Value is correct");

                assert_eq!(*number, 0.5);
            }

            #[test]
            fn above_one_json() {
                let value = "1.5";
                let number: Uf64 = serde_json::from_str(value)
                    .expect("Value is correct");

                assert_eq!(*number, 1.5);
            }

            #[test]
            fn below_zero_json() {
                let value = "-0.5";
                let number: Uf64 = serde_json::from_str(value)
                    .expect("Value is correct");

                assert_eq!(*number, 0f64);
            }
        }
    }

    mod normalized {
        use super::*;

        #[test]
        fn size() {
            assert_eq!(size_of::<Nf64>(), size_of::<f64>());
        }

        #[test]
        fn align() {
            assert_eq!(align_of::<Nf64>(), align_of::<f64>());
        }

        mod normal {
            use super::*;

            #[test]
            fn basic() {
                let number = Nf64::new(0.5);
                assert_eq!(*number, 0.5);
            }

            #[test]
            fn above_one() {
                let number = Nf64::new(1.5);
                assert_eq!(*number, 1f64);
            }

            #[test]
            fn below_zero() {
                let number = Nf64::new(-0.5);
                assert_eq!(*number, 0f64);
            }
        }

        mod serialization {
            use super::*;

            #[test]
            fn basic_json() {
                let number = Nf64::new(0.5);
                assert_eq!(
                    serde_json::to_string(&number).expect("Should be parsed"),
                    "0.5"
                );
            }

            #[test]
            fn above_one_json() {
                let number = Nf64::new(1.5);
                assert_eq!(
                    serde_json::to_string(&number).expect("Should be parsed"),
                    "1.0"
                );
            }

            #[test]
            fn below_zero_json() {
                let number = Nf64::new(-0.5);
                assert_eq!(
                    serde_json::to_string(&number).expect("Should be parsed"),
                    "0.0"
                );
            }
        }

        mod deserialization {
            use super::*;

            #[test]
            fn basic_json() {
                let value = "0.5";
                let number: Nf64 = serde_json::from_str(value)
                    .expect("Value is correct");

                assert_eq!(*number, 0.5);
            }

            #[test]
            fn above_one_json() {
                let value = "1.5";
                let number: Nf64 = serde_json::from_str(value)
                    .expect("Value is correct");

                assert_eq!(*number, 1f64);
            }

            #[test]
            fn below_zero_json() {
                let value = "-0.5";
                let number: Nf64 = serde_json::from_str(value)
                    .expect("Value is correct");

                assert_eq!(*number, 0f64);
            }
        }
    }
}

// Previous version (let it be)
//=============================================================================
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Uf64(
//     #[serde(deserialize_with = "Uf64::from_deserializer")]
//     f64,
// );
//  
// impl Uf64 {
//     fn check_float(value: f64) -> (Option<f64>, f64) {
//         if 0f64 > value {
//             (None, 0f64)
//         } else {
//             (Some(value), value)
//         }
//     }
//  
//     fn apply_check(value: f64) -> f64 {
//         let res = Self::check_float(value);
//  
//         if let Some(value) = res.0 {
//             value
//         } else {
//             res.1
//         }
//     }
//  
//     pub fn from_deserializer<'de, D>(deserializer: D) -> Result<f64, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         f64::deserialize(deserializer)
//             .and_then(|value| Ok(Self::apply_check(value)))
//     }
//  
//     pub fn new(value: f64) -> Self {
//         Self(Self::apply_check(value))
//     }
// }
//  
// impl Deref for Uf64 {
//     type Target = f64;
//  
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//  
// impl AsRef<f64> for Uf64 {
//     fn as_ref(&self) -> &f64 {
//         &self.0
//     }
// }
//=============================================================================

