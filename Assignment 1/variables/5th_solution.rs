//Shadowing problem solution
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); //value of 5 is changed to 12 because value of x is 12 in inner scope.
    }

    assert_eq!(x, 5);      //value of 12 is changed to 5.

    let x =  42;
    println!("{}", x);     // Prints "42".
}