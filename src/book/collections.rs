#[derive(Debug)]
enum CheatSheet {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}

pub fn enum_vector() {
    let mut v = vec![
        CheatSheet::Int(1),
        CheatSheet::Float(1.0),
        CheatSheet::Text(String::from("Hello")),
    ];

    for item in &mut v {
        if let CheatSheet::Int(n) = item {
            *n += 1;
        } else {
            println!("Not Int");
        }
    }

    println!("{:?}", v[0]);
}

pub fn collection_string() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
}
