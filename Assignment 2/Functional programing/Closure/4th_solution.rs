// Type inferred problem solution
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    /* Make it work, only changeg the following line */
    let n = example_closure(String::from("5"));  // added 'string::from' to the value '5'
}