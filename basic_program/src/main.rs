fn main() {
    println!("Hello, world!");
    let joe1 = Joe {
        name: String::from("joe"),
        age: 42,
    };
}

struct Joe {
    name: String,
    age: i32,
}
impl Joe {}
