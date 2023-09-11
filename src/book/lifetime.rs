fn max_num<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        &x
    } else {
        &y
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetime1() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// pub fn lifetime2() {
//     let x = 8; // -------------+-- x start
//     let max; // -------------+-- max start
//     {
//         //              |
//         let y = 1; // -------------+-- y start
//         max = max_num(&x, &y); //              |
//     } // -------------+-- y over
//     println!("max: {}", max); //
// }
