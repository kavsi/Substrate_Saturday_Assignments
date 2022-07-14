//Destructuring assignments problem solution
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    // we use '..' to Ignoring Remaining Parts of a Value. so x becomes 3 and y becomes 2.
    assert_eq!([x,y], [3,2]);       

    println!("Success!");
} 
