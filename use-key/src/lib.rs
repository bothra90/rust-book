enum TrafficLight {
    Red,
    Yellow,
    _Green,
}

use TrafficLight::{Red, Yellow};
pub fn f() {
    let _red = Red;
    let _yellow = Yellow;
    // let _green = TrafficLight::Green; // because we didn’t `use` TrafficLight::Green
}

#[cfg(test)]
mod tests {
    use f;
    #[test]
    fn it_works() {
        f();
    }
}
