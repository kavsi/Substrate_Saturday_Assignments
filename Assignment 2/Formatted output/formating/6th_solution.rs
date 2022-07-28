// precision problem solution
fn main() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416 

    assert_eq!(format!("{:.2}", v), "3.14");    // the blank is filled with '{:.2}'
    assert_eq!(format!("{:+.2}", v), "+3.14");  // the blank is filled with '{:+.2}'
    assert_eq!(format!("{:.0}", v), "3");       // the blank is filled with '{:.0}'

    println!("Success!")
}