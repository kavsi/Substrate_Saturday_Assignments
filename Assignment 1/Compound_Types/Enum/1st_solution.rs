//Enum problem solution
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn main() {
    // a enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);    // used 'as u8' to both
    assert_eq!(Number1::One as u8, Number2::One as u8);   // used 'as u8' to both
} 

