use std::thread;

pub fn immutable_only_borrow() {
    let list = vec![1, 2, 3];
    println!("Before defining a scope lend, the list = {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure, the list = {:?}", list);
    only_borrows();
    println!("After calling closure, the list = {:?}", list);
}

pub fn mutable_borrow() {
    let mut list = vec![1, 2, 3];
    println!("Before defining a scope lend, the list = {:?}", list);

    let mut borrows_mutable = || list.push(7);

    //Don't use println! between definition and execute

    borrows_mutable();
    println!("After calling closure, the list = {:?}", list);

    //borrows_mutable(); //ERROR because list is immutable on previous code
}

pub fn move_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining a scope lend, the list = {:?}", list);

    // `move` keyword usually used to thread process

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

//fn make_a_cloner(s_ref: str) -> impl Fn() -> String + '_ //lifetime elision rules

fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

pub fn main_make_cloner() {
    let s_own = String::from("Hello");
    let s_ref = &s_own;
    let cloner = make_a_cloner(s_ref);

    //drop(s_own); //Error, because s_own lifetime is bound with cloner
}
