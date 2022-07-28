// Convert by as problem solution
fn main() {
    let mut values [i32; 2] = [1, 2];
    let p1 mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;    the blank is filed with 'as usize'
    let second_address = first_address + 4;  4 == stdmemsize_ofi32()
    let p2 = second_address as mut i32;   p2 points to the 2nd element in values and blank is filled with 'as mut i32'
    unsafe {
        p2 += 1;    this line is added.
    }
    assert_eq!(values[1], 3);

    println!(Success!)
}