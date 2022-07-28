// Named argumentss problem solution
fn main() {
    println!("{argument}", argument = "test"); // => "test"

    /* Fill in the blanks */
    assert_eq!(format!("{name}{}", 1, name = 2), "21");  // the blank is filled with 'name = 2'
    assert_eq!(format!("{a} {c} {b}",a = "a", b = 'b', c = 3 ), "a 3 b");   // the blank is filled with "{a} {c} {b}"
    
    // named argument must be placed after other arguments
    println!("{abc} {0}", 2, abc = "def");  // replaced value '1' with '0'

    println!("Success!")
}