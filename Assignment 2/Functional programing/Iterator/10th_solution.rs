// Iterator adaptors problem solution
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  // the blanks are filled with 'map(|x| x + 1)' and 'collect()'

    assert_eq!(v2, vec![2, 3, 4]);
}