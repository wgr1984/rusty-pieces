use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    #[derive(Debug)]
    struct TestStruct {
        test0: String,
        test1: String
    }

    impl TestStruct {
        fn new(test0: &str, test1: &str) -> TestStruct {
            TestStruct {
                test0: String::from(test0),
                test1: String::from(test1)
            }
        }
    }

    let mut stack = vec![ 
        TestStruct::new("Hallo1", "Hallo2"),
        TestStruct::new("Hallo1", "Hallo2"),
        TestStruct::new("Hallo1", "Hallo2"),
        TestStruct::new("Hallo1", "Hallo2"),
        TestStruct::new("Hallo1", "Hallo2"),
        TestStruct::new("Hallo1", "Hallo2"),
    ];

    // while let Some(top) = stack.pop() {
    //     println!("{:?}", top);
    // }
    for (index, value) in stack.iter().enumerate()  {
        println!("{} {:?}", index, value);
    }


    let x = Some(12);
    let y = 10;

    match x {
        Some(10..=50) => println!("Got 10..=50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    struct Point {
        x: i32,
        y: i32,
    }
    

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);


    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }

    enum BinTree {
        Leaf(i32),
        Node(RefCell<Rc<BinTree>>, RefCell<Rc<BinTree>>),
        Empty()
    }

    let subtree = RefCell::new(Rc::new(BinTree::Node(
        RefCell::new(Rc::new(BinTree::Leaf(0))), 
        RefCell::new(Rc::new(BinTree::Node(
            RefCell::new(Rc::new(BinTree::Leaf(-1))), 
            RefCell::new(Rc::new(BinTree::Node(
                RefCell::new(Rc::new(BinTree::Leaf(-2))), 
                RefCell::new(Rc::new(BinTree::Empty()))
            )))
        )))
    )));

    let s = subtree.as_ptr();

    let tree = BinTree::Node(
        RefCell::new(Rc::new(BinTree::Leaf(4))),
        RefCell::new(Rc::new(BinTree::Node(
            RefCell::new(Rc::new(BinTree::Leaf(3))), 
            RefCell::new(Rc::new(BinTree::Node(
                RefCell::new(Rc::new(BinTree::Leaf(2))), 
                RefCell::new(Rc::new(BinTree::Node(
                    RefCell::new(Rc::new(BinTree::Leaf(1))), 
                    subtree
                )))
            )))
        )))
    );

    if let BinTree::Node(t, _) = &tree {
        *t.borrow_mut() = Rc::new(BinTree::Leaf(12345))
    }
    
    // if let BinTree::Node(x, y) = subtree.as_ref() {

    // }

    fn visit(t: &BinTree) {
        match t {
            BinTree::Leaf(x) => println!("Value: {}", x),
            BinTree::Node(l, r) => {
                let left = &l.borrow();
                visit(left);

                let right = &r.borrow();
                visit(right);
            },
            BinTree::Empty() => {}
        }
    }

    visit(&tree);
}