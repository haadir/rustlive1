enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write (String),
    ChangeColor (i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data");
        }
        Message::Move {x, y} => {
            println!("Move to x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to R: {}, B: {}, G: {}", r, b, g);
        }
    }
}

fn main() {
    process_message(Message::Quit);
    process_message(Message::Move {x: 10, y: 20});
    process_message(Message::Write(String::from("hello")));
    process_message(Message::ChangeColor(255, 0, 128));
}
