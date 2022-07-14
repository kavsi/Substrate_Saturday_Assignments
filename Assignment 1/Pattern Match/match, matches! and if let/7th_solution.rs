//If let problem solution
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);      // using let and if let statements    
    
    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
    }
}