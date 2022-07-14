//For problem solution
fn main() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {       // '&' is added to names
        // do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // the elements in numbers are Copy，so there is no move here
    for n in numbers {
        // do something with name...
    }

    println!("{:?}", numbers);
}