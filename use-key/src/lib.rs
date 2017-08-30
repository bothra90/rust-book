#[cfg(test)]

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green; // because we didn’t `use` TrafficLight::Green
}

mod tests {
    #[test]
    fn it_works() {}
}
