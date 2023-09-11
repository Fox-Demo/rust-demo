struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn print_msg(&self) {
        println!("This is a f64 point");
    }
}

#[derive(Debug)]
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn get_largest_number(list: &[i32]) -> &i32 {
    let mut max = &list[0];
    for n in list {
        if n > max {
            max = n;
        }
    }
    max
}

fn get_largest_chars(list: &[char]) -> &char {
    let mut max = &list[0];
    for n in list {
        if n > max {
            max = n;
        }
    }
    max
}

fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for n in list {
        if n > max {
            max = n;
        }
    }
    max
}

pub fn largest() {
    let number_list = vec![1, 3, 34, 523, 12, 89];
    let large = get_largest_number(&number_list);
    println!("Largest number is {}", large);

    let char_list = vec!['a', 'v', 'z', 'l'];
    let large_char = get_largest_chars(&char_list);
    println!("Largest number is {}", large_char);

    let large_number_generic = get_largest(&number_list);
    let large_char_generic = get_largest(&char_list);
    println!("number: {}", large_number_generic);
    println!("char: {}", large_char_generic);
}

pub fn generic_struct() {
    let point_i32 = Point { x: 1, y: 2 };
    let point_f64 = Point { x: 4.0, y: 2.0 };
    point_f64.print_msg();
    println!("i32 of x => {}", point_i32.x());
    println!("f64 of x => {}", point_f64.x());
    // let point_error = Point { x: 1, y: 2.0 };s
}

pub fn mix_type() {
    let point_i32 = Point2 { x: 1, y: 2 };
    let point_f64 = Point2 { x: 4.0, y: 2.0 };

    let new_point = point_i32.mixup(point_f64);

    println!("new {:?}", new_point);
}
