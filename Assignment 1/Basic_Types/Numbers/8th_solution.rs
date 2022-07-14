//Floating-Point problem solution
//1st method:
fn main() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); //adding floating-point types to the numbers. 

    println!("Success!");
}

//2nd method:
fn main() {
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);  //the .abs() is used to get the absolute value.
    
    println!("Success!");
}
