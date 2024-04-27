
use std::collections::HashMap;

pub trait Dumpable {
    fn dump(self: &Self, dumper: &mut dyn Dumper);
    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str);
}

pub trait Dumper {
    fn finish(self: &mut Self);
    fn dump_none(self: &mut Self);
    fn dump_u64(self: &mut Self, val: u64);
    fn dump_i64(self: &mut Self, val: i64);
    fn dump_f64(self: &mut Self, val: f64);
    fn dump_str(self: &mut Self, val: &str);
    fn dump_arr(self: &mut Self, val: &mut dyn Iterator<Item=&dyn Dumpable>);
    fn dump_bool(self: &mut Self, val: bool);
    fn dump_u64_as_parameter(self: &mut Self, name: &str, val: u64);
    fn dump_i64_as_parameter(self: &mut Self, name: &str, val: i64);
    fn dump_f64_as_parameter(self: &mut Self, name: &str, val: f64);
    fn dump_str_as_parameter(self: &mut Self, name: &str, val: &str);
    fn dump_arr_as_parameter(self: &mut Self, name: &str, val: &mut dyn Iterator<Item=&dyn Dumpable>);
    fn dump_bool_as_parameter(self: &mut Self, name: &str, val: bool);
    fn dump_fold_as_parameter(self: &mut Self, name: &str, val: &dyn Dumpable);
}

pub trait DumperResult {
    type Item;

    fn ready(self: &Self) -> bool;
    fn result(self: Self) -> Option<Self::Item>;
}

impl Dumpable for bool {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_bool(*self)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_bool_as_parameter(name, *self)
    }
}

impl Dumpable for f32 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_f64(*self as f64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_f64_as_parameter(name, *self as f64)
    }
}

impl Dumpable for f64 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_f64(*self)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_f64_as_parameter(name, *self)
    }
}

impl Dumpable for usize {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_u64(*self as u64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_u64_as_parameter(name, *self as u64)
    }
}

impl Dumpable for u8 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_u64(*self as u64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_u64_as_parameter(name, *self as u64)
    }
}

impl Dumpable for u16 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_u64(*self as u64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_u64_as_parameter(name, *self as u64)
    }
}

impl Dumpable for u32 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_u64(*self as u64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_u64_as_parameter(name, *self as u64)
    }
}

impl Dumpable for u64 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_u64(*self)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_u64_as_parameter(name, *self)
    }
}

impl Dumpable for isize {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_i64(*self as i64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_i64_as_parameter(name, *self as i64)
    }
}

impl Dumpable for i8 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_i64(*self as i64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_i64_as_parameter(name, *self as i64)
    }
}

impl Dumpable for i16 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_i64(*self as i64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_i64_as_parameter(name, *self as i64)
    }
}

impl Dumpable for i32 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_i64(*self as i64)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_i64_as_parameter(name, *self as i64)
    }
}

impl Dumpable for i64 {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_i64(*self)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_i64_as_parameter(name, *self)
    }
}

impl Dumpable for char {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        let mut buffer: [u8; 4] = [0; 4]; // enough according to documentation
        dumper.dump_str(self.encode_utf8(&mut buffer))
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        let mut buffer: [u8; 4] = [0; 4];
        dumper.dump_str_as_parameter(name, self.encode_utf8(&mut buffer))
    }
}

impl Dumpable for str {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_str(self)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_str_as_parameter(name, self)
    }
}

impl Dumpable for String {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_str(&self)
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_str_as_parameter(name, &self)
    }
}

impl<T: Dumpable> Dumpable for &[T] {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_arr(&mut self.iter().map(|item| item as &dyn Dumpable));
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl<T: Dumpable> Dumpable for Vec<T> {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        dumper.dump_arr(&mut self.iter().map(|item| item as &dyn Dumpable));
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl<T: Dumpable> Dumpable for HashMap<String, T> {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.iter().for_each(|(k, v)| v.dump_as_parameter(dumper, k));
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl<T: Dumpable> Dumpable for HashMap<&str, T> {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.iter().for_each(|(k, v)| v.dump_as_parameter(dumper, k));
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl<T: Dumpable> Dumpable for Option<T> {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        match self {
            Some(inner) => inner.dump(dumper),
            None => dumper.dump_none(),
        }
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}


