
use serde_json;

use domain::light::Light;
use domain::brightness::Brightness;
use domain::capabilities::Capability;
use domain::mode::Mode;
use domain::color::rgb::RGB;

fn main() {
    let mut light = Light::new("test".to_string(), "1".to_string(),
                               vec![Capability::Color, Capability::Brightness]);

    light.set_color(RGB::new(255, 0, 0).into()).expect("aboba");
    light.set_brightness(Brightness::new(-0.2)).expect("baoab");
    light.set_mode(Mode::new_empty("test".to_string(), "123".to_string()))
        .inspect_err(|err| {eprintln!("Ну да...\n{err}")})
        .expect_err("Так быть не должно");

    // println!("{:?}", light);
    println!("{}", serde_json::to_string(&light).unwrap());
}


