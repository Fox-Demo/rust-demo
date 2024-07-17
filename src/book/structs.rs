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
    println!("jack is {:?}", jack);

    let a: Person = Person::adult(String::from("Jack"), 13);
    println!("a is {:?}", a);
    println!("a is a adult? -> {}", a.is_adult());
    a.hello_person();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct ColorTuple(i32, i32, i32);

struct UnitStruct; //Unit-Like structs

fn create_color() {
    let black = ColorTuple(0, 0, 0);
}

fn create_user() {
    let mut user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2@gmail.com"),
        ..user1
    };
}

fn borrow_struct() {
    #[derive(Debug)] //attribute -> use debug {:?}
    struct Point {
        x: i32,
        y: i32,
    }

    fn print_point(p: &Point) {
        println!("{}, {}", p.x, p.y);
    }

    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;
    print_point(&p);
    println!("{:?}", p); // Debug -> :?
    println!("{:#?}", p); // Format Debug -> :#?
}

fn self_rect() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn max(&self, other: Self) -> Self {
            let w = self.width.max(other.width);
            let h = self.height.max(other.height);
            Rectangle {
                width: w,
                height: h,
            }
        }
        fn set_to_max(&mut self, other: Rectangle) {
            *self = self.max(other);
        }
    }
    let mut rect = Rectangle {
        width: 0,
        height: 1,
    };
    let other_rect = Rectangle {
        width: 1,
        height: 0,
    };
    rect.set_to_max(other_rect);
    println! {"{:#?}", rect};
}
