#[derive(Debug)]
enum Message {
    Quit,
    Color(u8, u8, u8),
    Move { x: i32, y: i32 },
    Write(String),
    Num(i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Color(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
            Message::Move { x: a, y: b } => println!("X is {}, y is {}", a, b),
            Message::Write(s) => {
                println!("Write: {}", s);
                println!("This is Write")
            }
            Message::Num(n) => println!("Num: {}", n),
        }
    }
}

pub fn get_enum() {
    let quit = Message::Quit;
    let color = Message::Color(0, 0, 0);
    let move_msg = Message::Move { x: 1, y: 32 };
    let write = Message::Write(String::from("Hello"));

    quit.call();
    color.call();
    move_msg.call();
    write.call();

    let msg_number: Message = Message::Num(12);
    match msg_number {
        Message::Num(n) => println!("Number is: {}", n),
        _ => println!("Not Write"),
    }

    match write {
        Message::Write(s) => println!("Write: {}", s),
        _ => println!("Not Write"),
    }

    println!("Get {:?}", msg_number);
    //println!("Get {:?}", write); //Error

    //Option
    let option_x: Option<i32> = Some(5);
    if let Some(x) = option_x {
        println!("x is {}", x);
    } else {
        println!("x is None");
    }

    println!("Get {:?}", option_x);
}
