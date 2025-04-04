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

pub fn example() {
    box_example();
    cons_example();
}
