
use domain::light::{
    Light,
    Capability,
};
use domain::color::rgb::RGB;
use serde_json;

fn main() {
    let mut light = Light::new("test".to_string(), "1".to_string(),
                               vec![Capability::Color, Capability::Brightness]);

    light.set_color(RGB::new(255, 0, 0).into()).expect("aboba");

    // println!("{:?}", light);
    println!("{}", serde_json::to_string(&light).unwrap());
}


