// Convert by as problem solution
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;  // the integer type here is 'u8'

    let c1: char = decimal as u8 as char;  // the error is corrected by adding 'as u8'
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8 - 1);
}