#[derive(Debug)]
enum Message {
    Quit,
    Color(u8, u8, u8),
    Move { x: i32, y: i32 },
    Write(String),
    Num(i32),
}

impl Message {
    fn call(&mut self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Color(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
            Message::Move { x: a, y: b } => println!("X is {}, y is {}", a, b),
            Message::Write(s) => {
                println!("Write: {}", s);
                *s = String::from("World");
            }
            Message::Num(n) => {
                println!("Num: {}", n);
                *n = *n + 1;
            }
        }
    }
}

pub fn get_enum() {
    let mut quit = Message::Quit;
    let mut color = Message::Color(0, 0, 0);
    let mut move_msg = Message::Move { x: 1, y: 32 };
    let mut write = Message::Write(String::from("Hello"));

    quit.call();
    color.call();
    move_msg.call();
    write.call();

    println!("Get {:?}", write);

    let mut msg_number: Message = Message::Num(12);
    msg_number.call();

    match msg_number {
        Message::Num(n) => println!("Number is: {}", n),
        _ => println!("Not Num"), //This is a default case
    }

    match write {
        Message::Write(s) => println!("Write: {}", s), //Ownership is moved
        _ => println!("Not Write"),
    }

    println!("Get {:?}", msg_number);
    //println!("Get {:?}", write); //Error because write is moved on line 50

    //Option
    let option_x: Option<i32> = Some(5);
    if let Some(x) = option_x {
        println!("x is {}", x);
    } else {
        println!("x is None");
    }

    println!("Get {:?}", option_x);
}
