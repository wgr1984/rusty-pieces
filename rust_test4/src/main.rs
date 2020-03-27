use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let test0;
    let test1;
    let test2;
    let test3;

    {
        let test = Rc::new(120);
        test0 = Rc::downgrade(&test);
        test1 = Weak::clone(&test0);
        test2 = Rc::new(120);
        test3 = RefCell::new(121);
        let test4 = test3.borrow_mut();

        println!("test0={:?} test1={:?} test2={:?}", test0.upgrade(), test1.upgrade(), test2);
    }

    let test5 = test3.borrow();

    println!("test0={:?} test1={:?} test2={:?} test3={:?}", test0.upgrade(), test1.upgrade(), test2, test3.borrow());
}