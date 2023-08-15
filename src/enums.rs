enum Message {
    Quit,
    Color(u8, u8, u8),
    Move { x: i32, y: i32 },
    Write(String),
}

fn match_msg(m: &Message) {
    match m {
        Message::Quit => println!("Quit"),
        Message::Color(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        Message::Move { x: a, y: b } => println!("X is {}, y is {}", a, b),
        Message::Write(s) => {
            println!("Write: {}", s);
            println!("This is Write")
        }
    }
}

pub fn get_enum() {
    let quit = Message::Quit;
    let color = Message::Color(0, 0, 0);
    let move_msg = Message::Move { x: 1, y: 32 };
    let write = Message::Write(String::from("Hello"));

    match_msg(&quit);
    match_msg(&color);
    match_msg(&move_msg);
    match_msg(&write);

    match_msg(&write);

    //Option
    let option_x: Option<i32> = None;
    if let Some(x) = option_x {
        println!("x is {}", x);
    } else {
        println!("x is None");
    }
}
