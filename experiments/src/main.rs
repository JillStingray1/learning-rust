mod examples;
use examples::*;

fn main() {
    smart_pointers::example();
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

    threads::thread_example();
}
