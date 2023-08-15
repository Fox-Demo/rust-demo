#[derive(Debug)]

struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn is_adult(&self) -> bool {
        if self.age > 18 {
            return true;
        } else {
            return false;
        }
    }

    pub fn adult(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

pub fn structs_kata() {
    let jack: Person = Person {
        name: String::from("Not jack"),
        age: 1,
    };

    println!("{}", jack.is_adult());
    borrow_struct(&jack);
    println!("jack is {:?}", jack);

    let a: Person = Person::adult(String::from("No"), 13);
    println!("a is {:?}", a);
    println!("a is a adult? -> {}", a.is_adult())
}

fn borrow_struct(p: &Person) {
    println!("{:?}", p);
}
