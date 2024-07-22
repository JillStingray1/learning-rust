mod collections;
mod enums;
mod errors;
mod generics;
mod second_word;
mod structs;

fn main() {
    collections::example();
}

#[allow(dead_code)]
fn unused() {
    second_word::example();
    structs::example();
    enums::example();
    collections::example();
    generics::example();
}
