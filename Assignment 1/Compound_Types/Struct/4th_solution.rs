//Operating on structs problem solution
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {                 // mut is added to p
        name: String::from("sunface"),
        age,
    };

    p.age = 30;
   
    p.name = String::from("sunfei");     // the blak is filled with p.name

    println!("Success!");
}
