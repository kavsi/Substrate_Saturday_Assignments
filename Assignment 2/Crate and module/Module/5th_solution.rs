// Module problem solution
// in src/main.rs
mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please"); // the blank is filled with 'front_of_house::hosting::seat_at_table()' 
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");  // the bllank is filled with 'hello_package::eat_at_restaurant()'
}