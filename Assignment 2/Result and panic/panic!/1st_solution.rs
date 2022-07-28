// panic! problem solution
use core::panic;

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("drinked, duang.....peng!")
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");   // the blank is filled with '("lemonade")'

    println!("Exercise Failed if printing out this line!");
}