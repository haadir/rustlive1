fn main() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;
        println!("Looping count is {}", counter);

        if counter == 5 {
            break counter * 2;
        }
    };
    
    println!("{}", result);

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}