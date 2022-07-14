//The types of structs problem solution
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "coding".to_string()    // line added for code to work
    };
    
    println!("Success!");
}