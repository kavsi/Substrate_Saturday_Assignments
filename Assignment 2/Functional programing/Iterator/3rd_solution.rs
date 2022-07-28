// next method problem solution
// 1st method:
fn main() {
    let v1 = vec![1, 2];

    // moving ownership
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));  // the blank is filled with 'Some(1)'
    assert_eq!(v1_iter.next(), Some(2));  // the blank is filled with 'Some(2)'
    assert_eq!(v1_iter.next(), None);     // the blank is filled with 'None'
}

// 2nd mathod:
fn main() {
    let v1 = vec![1, 2];

    // borrowing
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));  // the blank is filled with 'Some(&1)'
    assert_eq!(v1_iter.next(), Some(&2));  // the blank is filled with 'Some(&2)'
    assert_eq!(v1_iter.next(), None);      // the blank is filled with 'None'
}