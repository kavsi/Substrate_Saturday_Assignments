//While problem solution
fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;   // continue is used to continue the loop
        }

        break;          // break is usd to break the loop
    }

    assert_eq!(n, 66);
    
    println!("Success!");
}