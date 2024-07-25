use std::collections::HashMap;

fn vector() {
    // creates an empty vector, can't do type inference on an empty vector
    let _v: Vec<i32> = Vec::new();
    // alternative syntax for predefined vector
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);
    v.push(7);
    println!("{:?}", v);
}

fn string() {
    let mut s: String = String::from("some string");
    println!("{s}");
    s += " lmao";
    println!("{s}");
    // strings cannot be indexed into, as strings are stored as a vector of
    // bytes which are encoded as utf-8
    // this prints the bytes making up String s
    s = String::from("Chenjun");
    println!("{:x?}", s.as_bytes());
    // you can also print individual chars
    println!("{:?}", s.chars());
    // for some languages, which use complex characters (eg. diacritics)
    // chars may not be sufficient to find the individual grapheme clusters
    // which means we must access other libraries
}

fn hash_maps() {
    // creating an empty hashmap
    let mut scores = HashMap::new();
    // adding elements into hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // retrieving a value from a hashmap
    // get returns an Option<&i32>, which means we should first copy the value
    // to get an Option<i32>, and then get and i32 by unwrapping, if the value
    // is None, then then the score gets set to 0
    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("{score}");
    // iterating through a hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

pub fn example() {
    vector();
    string();
    hash_maps();
}
