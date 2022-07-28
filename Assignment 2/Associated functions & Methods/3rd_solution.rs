// Method problem solution
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Methods must have a parameter named self of type Self for their first parameter, 
    // Rust lets you abbreviate this with only the name self in the first parameter spot.
    pub fn show_state(self)  {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {     // used '&mut self' to fill in the blank.
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}
