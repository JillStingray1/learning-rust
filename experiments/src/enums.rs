enum Person {
    Male(String),
    Female(String),
}

pub fn example() {
    sex(Person::Male(String::from("Joe Biden")));
    sex(Person::Female(String::from("Jill Stingray")))
}

fn sex(person: Person) {
    match person {
        Person::Male(ref i) => println!("{i} is a male"),
        Person::Female(ref i) => println!("{i} is a female"),
    }
    if let Person::Male(ref i) = person {
        println!("This person's name is {i}")
    }
}
