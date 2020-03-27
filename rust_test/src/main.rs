use std::thread;
use std::time::Duration;


trait TypeInfo {
    fn type_name() -> String;
    fn type_of(&self) -> String;
}

macro_rules! impl_type_info {
    ($($name:ident$(<$($T:ident),+>)*),*) => {
        $(impl_type_info_single!($name$(<$($T),*>)*);)*
    };
}

macro_rules! mut_if {
    ($name:ident = $value:expr, $($any:expr)+) => (let mut $name = $value;);
    ($name:ident = $value:expr,) => (let $name = $value;);
}

macro_rules! impl_type_info_single {
    ($name:ident$(<$($T:ident),+>)*) => {
        impl$(<$($T: TypeInfo),*>)* TypeInfo for $name$(<$($T),*>)* {
            fn type_name() -> String {
                mut_if!(res = String::from(stringify!($name)), $($($T)*)*);
                $(
                    res.push('<');
                    $(
                        res.push_str(&$T::type_name());
                        res.push(',');
                    )*
                    res.pop();
                    res.push('>');
                )*
                res
            }
            fn type_of(&self) -> String {
                $name$(::<$($T),*>)*::type_name()
            }
        }
    }
}

impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a T {
    fn type_name() -> String {
        let mut res = String::from("&");
        res.push_str(&T::type_name());
        res
    }
    fn type_of(&self) -> String {
        <&T>::type_name()
    }
}

impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a mut T {
    fn type_name() -> String {
        let mut res = String::from("&mut ");
        res.push_str(&T::type_name());
        res
    }
    fn type_of(&self) -> String {
        <&mut T>::type_name()
    }
}

macro_rules! type_of {
    ($x:expr) => { (&$x).type_of() };
}

impl_type_info!(u32, i32, i64, f32, f64, str, String, Vec<T>, Result<T,S>);

struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // generate_workout(
    //     simulated_user_specified_value,
    //     simulated_random_number
    // );

    let x = vec![1, 2, 3];

    let equal_to_x = & |z| {
        for i in &x {
            println!("{}", i);
        }
        z == x
    };

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));


    let v1 = &[1, 2, 3];

    v1.iter()
        .map(|x| x + 1)
        .for_each(|x| println!("{}", x));
}

#[derive(PartialEq, Debug)]
struct Shoe<'a> {
    size: u32,
    style: &'a str,
}

// fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     println!("{:?}", &shoes as *const _);
//     shoes.into_iter()
//         .filter(|s| s.size == shoe_size)
//         .collect()
// }

fn shoes_in_my_size<'a>(shoes: &'a Vec<Shoe>, shoe_size: u32) -> Vec<&'a Shoe<'a>> {
    println!("{:?}", shoes as *const _);
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: "sneaker" },
        Shoe { size: 13, style: "sandal" },
        Shoe { size: 10, style: "boot" },
    ];

    println!("{:?}", &shoes as *const _);

    let in_my_size = shoes_in_my_size(&shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            &Shoe { size: 10, style: "sneaker" },
            &Shoe { size: 10, style: "boot" },
        ]
    );

    println!("{:?}", shoes);
}

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_clousure = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_clousure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_clousure.value(intensity)
        );
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_clousure.value(intensity)
        );
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}