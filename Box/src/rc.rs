use std::rc::Rc;
use List::{Cons, Nil};

//Rc -> reference counting

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn rc_list() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // count 1

    // equal to `a.clone()`, but Rc clone() is deep copy by default, so use `Rc::clone(&a)` in convention
    // 強調這是 Rc 的 clone，跟一般的 clone 不同，一般的 clone 會進行 deep copy
    let b = Cons(3, Rc::clone(&a)); // count 2
    println!("Strong count {}", Rc::strong_count(&a)); // eq to 2

    let c = Cons(4, Rc::clone(&a)); // count 3
    println!("Strong count {}", Rc::strong_count(&a)); // eq to 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_list() {
        rc_list();
    }
}
