
use dump::{Dumpable, Dumper, DumperResult};
use dump::dumpers::json::JSONDumper;

fn dump(object: &dyn Dumpable) -> String {
    let mut dumper = JSONDumper::new();
    object.dump(&mut dumper);
    dumper.finish();

    let val = dumper.result().expect("Unable to parse");
    val.to_string()
}

struct Other {
    v: Vec<String>,
}

struct Test {
    a: f64,
    b: i64,
    v: Other,
}

impl Other {
    fn new(v: Vec<String>) -> Self {
        Self { v }
    }
}

impl Test {
    fn new(a: f64, b: i64, v: Vec<String>) -> Self {
        Self { a, b, v: Other::new(v) }
    }
}

impl Dumpable for Other {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.v.dump_as_parameter(dumper, "v");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

impl Dumpable for Test {
    fn dump(self: &Self, dumper: &mut dyn Dumper) {
        self.a.dump_as_parameter(dumper, "a");
        self.b.dump_as_parameter(dumper, "b");
        self.v.dump_as_parameter(dumper, "v");
    }

    fn dump_as_parameter(self: &Self, dumper: &mut dyn Dumper, name: &str) {
        dumper.dump_fold_as_parameter(name, self);
    }
}

fn main() {
    // let val = 5;
    // let val: Vec<u8> = vec![1, 2, 3, 4];
    // let val = Test::new(1.47, -2);
    let val: Vec<Test> = vec![Test::new(1.47, -2, vec![String::from("aboba")]),
                              Test::new(-5.215, 215, vec![String::from("basbasd")])];

    println!("{}", dump(&val));
}


