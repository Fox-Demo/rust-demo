use std::collections::HashMap;

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
    let s = String::from("hello ");
    let s1 = s.clone();
    let s2 = String::from("world");

    let s3 = s1 + &s2;
    let s4 = format!("{s}{s2}");

    println!("{}", s3);
    println!("{}", s2);
    println!("{}", s4);
}

pub fn index_string() {
    //UTF-8 可以用來表示 Unicode 1~4 bytes 的方式
    //在 rust 中都是 UTF-8 的形式作儲存，因此每個字的長度都不一樣
    //也因為每個字的長度不一樣，所以這邊不能 indexing，很可能會產生無法預期的結果
    let en = String::from("Hola"); // 每個文字占 1 bytes
    let zh = String::from("大中天"); // 每個文字占 3 bytes
    let hello = String::from("Здравствуйте"); // 每個文字占 2 bytes
    println!("{}", en.len()); // 3
    println!("{}", zh.len()); // 9
    println!("{}", hello.len()); // 12
}

pub fn index_string_char() {
    //如果想要準確的印出每個字元，可以採用 for 遍尋的方式
    let en = String::from("Hola"); // 每個文字占 1 bytes
    let zh = String::from("大中天"); // 每個文字占 3 bytes

    for c in en.chars() {
        println!("{}", c);
    }

    for b in en.bytes() {
        println!("{}", b); //72 111 108 97 -> 這些是 ASCII 碼
    }

    for c in zh.chars() {
        println!("{}", c);
    }

    println!("{:?}", en.chars()); //Chars(['H', 'o', 'l', 'a'])
    println!("{:?}", zh.chars()); //Chars(['大', '中', '天'])
}

pub fn hash_map() {
    let mut scores = HashMap::new();
    scores.insert("Yellow".to_string(), 10);
    scores.insert(String::from("Blue"), 30);

    let team = String::from("Red");
    let score = scores.get(&team);
    let score2 = scores.get(&team).copied().unwrap_or(0);

    println!("{:?}", score); // None
    println!("{:?}", score2); // 0

    for (key, values) in &scores {
        println!("{0}: {1}", key, values);
    }
}

pub fn update_hash_map_value() {
    println!("=== Overwriting ===");
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 30);
        let mut result = scores.get("blue").copied().unwrap_or(0);
        println!("{}", result); // 30

        scores.insert(String::from("blue"), 10);
        result = scores.get("blue").copied().unwrap_or(0);
        println!("{}", result); // 10
    }

    println!("=== Adding a Key and Value Only If a Key isn't exist ===");
    {
        let mut scores = HashMap::new();
        //scores.insert(String::from("blue"), 30);
        scores.insert(String::from("yellow"), 10);

        scores.entry(String::from("blue")).or_insert(50);

        let result = scores.get("blue").copied().unwrap_or(0);
        println!("Blue score is {}", result); // 10
    }

    println!("=== Updating a Value Based on the Old Value ===");
    {
        let mut map = HashMap::new();
        let text = "hello world wonderful world";
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0); // return mut ref
            *count += 1;
        }
    }
}

pub fn collection_exercise() {
    // let numbers = vec![1, 5, 6];
    let numbers = vec![1, 2, 3, 4, 5, 6, 6];
    let size = numbers.len() / 2;
    println!("Median: {}", &numbers[size]); //4

    let mut results = HashMap::new();
    for n in &numbers {
        let count = results.entry(*n).or_insert(0); // 如果沒有 key: n 項的話就插入 0
                                                    // 返回一個 ref mut 型別的 value
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_index = 0;

    for (key, value) in &results {
        if *value > max_count {
            max_count = *value;
            max_index = *key;
        }
        println!("{0}: {1}", key, value);
    }

    println!("Mode: {}", max_index);
}

pub fn collection_exercise2() {
    //! Must input data in the function
    let mut company = HashMap::new();

    company.insert(String::from("Sales"), String::from("Amy"));
    company.insert(String::from("Engineering"), String::from("Li"));

    for (depart, name) in &company {
        println!("{0}: {1}", depart, name);
    }
}
