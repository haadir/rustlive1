 fn main() {
    // Stack data, using a copy
    let x: i32 = 10;
    let y: i32 = x;
    println!("y = {}", y);
    println!("x = {}", x);

    // Heap data
    let s1: String = String::from("hello");
    // have to add clone to make a deep copy
    let s2: String = s1.clone();
    println!("s2 = {}", s2);
    println!("s1 = {}", s1);

    let s3: String = String::from("Rust");
    // without .clone() borrow checker will complain
    calculate_length(s3.clone());

    println!("s3 = {}", s3);
 }

 fn calculate_length(s: String) -> usize {
    let len: usize = s.len();
    println!("Consumed string length: {}", len);
    len // return len
 }