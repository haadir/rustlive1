fn main() {
    let z: i32 = 5;
    println!("The value of z is: {}", z);

    let z: i32 = z + 1;
    println!("The value of z after shadowing is: {}", z);

    {
        let z: i32 = z * 2;
        println!("The value of z inside the inner scope is: {}", z);
    }

    println!("The value of z in the outer scope is: {}", z);
}