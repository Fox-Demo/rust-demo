//? What is diff between Box and Vec?

mod drop;
mod leak;
mod rc;
mod refcell;

use std::boxed::Box;
use std::ops::Deref;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Clone, Copy)]
pub struct AccessLogger(i32);

impl Deref for AccessLogger {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::List::{Cons, Nil};

    #[test]
    fn test_list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    #[test]
    fn test_my_box() {
        my_box();
    }

    #[test]
    fn test_deref_coercion() {
        let name = MyBox::new(String::from("Rust"));
        hello(&name);
    }

    #[test]
    fn test_access_logger() {
        let n = AccessLogger(-1);
        let num = *n + 1; // Copy occur here
        let n2 = n;

        assert_eq!(*n2, -1);
        assert_eq!(*n, -1);
        assert_eq!(num, 0);
    }
}
