// Have fixed length
// Data is allocated on the stack

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 integers
    let b: [i32; 5] = [3; 5]; // same as let b = [3, 3, 3, 3, 3]
    // access index with []
    println!("First element of a: {}", a[0]);
    println!("Second element of b: {}", b[1]);
}