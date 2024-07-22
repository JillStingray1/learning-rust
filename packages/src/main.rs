use garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    // structs fields inherit mutablility from their parents.
    let mut asparagus = Asparagus { age: 1 };
    println!("The aspagargus is age {}", asparagus.age);
    asparagus.age += 1;
    println!("The aspagargus is age {}", asparagus.age);
}
