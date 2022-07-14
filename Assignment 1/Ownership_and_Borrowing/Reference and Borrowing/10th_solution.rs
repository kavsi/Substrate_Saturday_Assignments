//NLL problem solution
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
   // println!("{}",r1);
   // the above line is turned into comment to make code work
}
