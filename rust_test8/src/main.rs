use std::ops::Add;
use std::fmt;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for String {}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

trait VectorTools<T> {
    fn for_each<F>(self, f: F) where
        F: Fn(T);
}

impl <T> VectorTools<T> for Vec<T> {
    fn for_each<F>(self, f: F) where
        F: Fn(T) {
            for x in self {
                f(x)
            }
    }
}

fn for_each<T>(v: Vec<T>, f: fn(T) -> ()) {
    for x in v {
        f(x)
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });

    (&Human{} as &dyn Wizard).fly();

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    String::from("Hello World!").outline_print();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    list_of_strings.iter().for_each(|x| println!("{}", x));

    for_each(list_of_strings, |item| item.outline_print());

    let test = returns_closure()(10);
    println!("added +1 {}", test);

    struct TestOption {
        field: Option<String>
    }

    let mut test = TestOption {
        field: None
    };

    test.field.replace(String::from("Hallo"));

    let h = test.field.take();

    test.field.replace(String::from("Hallo2"));
}