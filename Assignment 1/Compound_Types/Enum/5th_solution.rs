//Enum problem solution

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {     // the blank is filled with 'Some(n)'
        println!("{}", n);
        return                 // return is used to not let the code panic
        
        println!("Success!");
    } 
    
    panic!("NEVER LET THIS RUN！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,              // the blank is filled with 'none'
        Some(i) => Some(i + 1),    // the blank is filled with 'Some(i)'
    }
}