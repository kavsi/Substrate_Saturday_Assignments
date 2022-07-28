// into_iter, iter and iter_mut problem solution
fn main() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();  // the blank is filled with 'iter_mut()'

    if let Some(v) = values_iter.next() {   // the blank is filled with 'next()'
        *v = 0;  // this line is added 
    }

    assert_eq!(values, vec![0, 2, 3]);
}