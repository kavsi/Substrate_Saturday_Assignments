// for and iterator problem solution
fn main() {
    let mut v = Vec::new();
    for n in 1..101 {  // the blank is filled with '1..101'
       v.push(n);
    }

    assert_eq!(v.len(), 100);
}