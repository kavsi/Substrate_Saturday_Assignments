// Associated functions problem solution
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. implement a assotiated function `new`,
    // 2. it will return a TrafficLight contains color "red"
    // 3. must use `Self`, DONT use `TrafficLight`
    pub fn new() -> Self {               
        Self {                             // We can define associated functions that don’t have self as their first parameter.
            color: "red".to_string()       // the self block is implemented and the color is 'red'
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
}