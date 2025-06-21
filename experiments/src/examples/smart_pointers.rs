use std::ops::Deref;

fn box_example() {
    // simple example of a box, which stores a value on the heap instead of the stack
    let b = Box::new(5);
    println!("b = {b}");
}

/// This a cons list, a data structure similar to a linked list, usually
/// applied in lisp languages.
///
/// The data structure is recursive, and thus cannot be represented directly
/// in Rust, as the size of Rust types must be computable at runtime
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons_example() {
    use List::*;
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let third = match list {
        Cons(_, x) => match *x {
            Cons(_, x) => match *x {
                Cons(x, _) => x,
                _ => 0,
            },
            _ => 0,
        },
        _ => 0,
    };
    println!("the third element of cons is {third}")
}

/// Using smart pointers like a reference by using the deref trait
fn deref_example() {
    let x = 5;
    let y = Box::new(x);
    // since box implements the deref trait
    assert_eq!(5, *y);
}

/// Demonstration of deref trait
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

fn my_box() {
    let x = 5;
    let y = MyBox::new(x);
    // since box implements the deref trait
    assert_eq!(5, *y);
}

pub fn example() {
    box_example();
    cons_example();
    deref_example();
    my_box();
}
