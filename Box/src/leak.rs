use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn ref_cycle() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    use List::{Cons, Nil};

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); // a count 1
    println!("a rc count {}", Rc::strong_count(&a)); // 1
    println!("a next item {:?}", a.tail()); // Return -> Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // b count 1 & a count 2s
    println!("a rc count {}", Rc::strong_count(&a)); // 2
    println!("b rc count {}", Rc::strong_count(&b)); // 1
    println!("b next item {:?}", b.tail()); // Return -> Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // b count 2
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //In the end of scope, DROP !
    //a count 2 -> 1
    //b count 2 -> 1
    //Because Rc will drop when count is 0, so a & b will not be dropped

    //Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

//Tree Node
fn simple_node() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("{}", Rc::strong_count(&leaf)); // 2 (owned by self & branch)
    println!("{}", Rc::strong_count(&branch)); // 1 (owned by self)
}

fn quiz() {
    let r1 = Rc::new(1); // strong count 1
    let r4 = {
        let r2 = Rc::clone(&r1); // strong count 2
        Rc::downgrade(&r2) // weak count 1
    }; //drop r2, so strong count (2 - 1)

    let r5 = Rc::clone(&r1); // strong count 2
    let r6 = r4.upgrade(); // strong count 3
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
    println!("{:?}", r4.upgrade());
}

// Weak 不影響 ownership, 不會有 count 就歸 0 的狀況發生
fn weak_node() {
    //Add Weak to get parent node (avoid ref cycle)
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Check Weak point to exist
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // Downgrade Rc to Weak
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Option<Rc<Node>>

    assert_eq!(leaf.value, 3);
    assert_eq!(branch.value, 5);
    assert_eq!(branch.children.borrow()[0].value, 3);
    assert_eq!(branch.children.borrow().len(), 1);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_leak() {
        ref_cycle();
    }

    #[test]
    fn test_simple_node() {
        simple_node();
    }

    #[test]
    fn test_weak_node() {
        weak_node();
    }

    #[test]
    fn test_quiz() {
        quiz();
    }
}
