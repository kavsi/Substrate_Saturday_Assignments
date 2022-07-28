// Exercises problem solution
struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],  // changed float type numbers to integer type
        },
        Array {
            data: [1, 2, 3]   // value '3' is added, but we can add any number.
        }
    ];

    println!("Success!");
}
