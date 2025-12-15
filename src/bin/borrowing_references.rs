fn calculate_length(s: &String) -> usize {
    let len: usize = s.len();
    println!("Consumed string length: {}", len);
    len
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn main() {
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);

    println!("The lenght is {}", len);
    println!("s1 is still usuable: {}", s1);

    // Mutable borrow
    let mut s2: String = String::from("hello");
    println!("\nBefore change: {}", s2);
    change(&mut s2);
    println!("After change: {}", s2);
}