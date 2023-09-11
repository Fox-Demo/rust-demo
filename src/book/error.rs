//L9

use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 0 || value > 100 {
            panic!("Invalid value");
        }
        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

pub fn get_panic() -> i32 {
    let v = vec![1, 2, 3];
    v[99] // Cause panic error
}

pub fn open_file() {
    // Learn Result<T, E>
    let result = File::open("hello.txt");

    //Result 已經被自動包含，因此不用前綴 Result::
    let get_file1 = match result {
        Ok(ref file) => file,
        Err(error) => panic!("Problem with {}", error),
    };

    let get_file2 = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //針對不同種類的 error kind 去做處理
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem with {}", error),
            },
            other_kind => panic!("Problem with {}", other_kind),
        },
    };
}

pub fn result_method() {
    //與以下寫法相同

    // let get_file1 = match result {
    //     Ok(ref file) => file,
    //     Err(error) => panic!("Problem with {}", error),
    // };

    //let get_file3 = File::open("hello.txt").unwrap();
    let get_file3 = File::open("hello.txt").expect("hello.txt must be in project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // ? 做的事情就是將底下的 match 做簡化，主要作用是
    // 碰到 Err 就強制跳出的這個 function
    // 如果 Ok 那就繼續執行

    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

pub fn get_guess() {
    let guess = Guess::new(30);
    let invalid_guess = Guess::new(-1);

    println!("{}", guess.value());
}
