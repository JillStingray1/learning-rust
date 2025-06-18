mod examples;
use examples::*;

fn main() {
    threads::thread_example();
}

#[allow(dead_code)]
fn unused() {
    functional_programming::example();
    second_word::example();
    structs::example();
    enums::example();
    collections::example();
    generics::example();
    collections::example();
    lifetimes::example();
    smart_pointers::example();
}
