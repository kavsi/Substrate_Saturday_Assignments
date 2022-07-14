//Diverging functions problem solution
fn main() {
    let b = false;  // the value is false.

    let v = match b {
        true => 1,
        // Diverging functions can also be used in match expression
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}