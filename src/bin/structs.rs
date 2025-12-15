struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let email = String::from("user@example.com");
    let username = String::from("someuser123");

    let mut user1 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    println!("Username is: {}", user1.username);
}