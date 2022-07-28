// Struct and impl problem solution
// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val {
    val: f64,
}

impl<T> Val<T> {               // added '<T>' after 'impl' and 'Val'
    fn value(&self) -> &T {
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}