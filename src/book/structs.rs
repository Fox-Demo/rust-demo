//If you want print struct or enums, you need to use the Debug lint to print entirely

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

    //Constructor
    pub fn adult(name: String, age: i32) -> Self {
        Self { name, age }
    }

    pub fn hello_person(&self) {
        println!("Person name is {}", self.name);
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

    let a: Person = Person::adult(String::from("Jack"), 13);
    println!("a is {:?}", a);
    println!("a is a adult? -> {}", a.is_adult());
    a.hello_person();
}

fn borrow_struct(p: &Person) {
    println!("{:?}", p);
}
