//Destructuring problem solution
//1st method:
fn main() {
    let (mut x, y) = (1, 2);   //using "mut" keyword before variable x for mutability.
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//2nd method:
fn main() {
    let (x, y) = (1, 2);
    let x = 3;                  //implementing shadowing and assigning value 3 to x.

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}