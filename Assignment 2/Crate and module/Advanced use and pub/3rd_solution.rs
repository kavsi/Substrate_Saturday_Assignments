// Re-exporting names with pub use problem solutions
fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
     assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}

// for the solution to work
// Adding this line in lib.rs
pub use crate::front_of_house::hosting;