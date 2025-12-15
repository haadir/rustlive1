/*
- Parameters must have their types declared
- Can return a value; return type is delcared after ->
- Use return for early returns, otherwise last expression is returned
*/

fn print_value(x: i32) {
    println!("The value is: {}", x);
}

fn add_five (num: i32) -> i32 {
    num + 5
}

fn main() {
    print_value(42);
    let result = add_five(10);
    println!("10 + 5 = {}", result); 
}