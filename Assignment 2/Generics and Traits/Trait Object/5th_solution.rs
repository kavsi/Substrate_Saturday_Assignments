// Object safe problem solution
// 1st method:

trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> u32 { 42 }
}

impl MyTrait for String {
    fn f(&self) -> String { self.clone() }
}

fn my_function(x: impl MyTrait) -> impl MyTrait  {  // used 'impl MyTrait' instead of 'Box<dyn MyTrait>' and return type 'impl MyTrait'
    x.f()
}

fn main() {
    my_function(13_u32);
    my_function(String::from("abc"));
    
    println!("Success!");
}

// 2nd method:
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;  // used 'Box<dyn MyTrait>' instead of 'self'
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }  // used 'Box::new(42)' instead of '42' and 'Box<dyn MyTrait>' instead of 'self'
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }  // used 'Box::new(self.clone()' instead of 'self.clone()' and 'Box<dyn MyTrait>' instead of 'self'
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {  // return type is 'Box<dyn MyTrait>'
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));
    
    println!("Success!");
}