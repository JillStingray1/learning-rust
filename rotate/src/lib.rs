use std::{env, error::Error, fs};

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>>{
    let mut rotation_val: u8 = 13;
    for arg in &args {
        if arg.parse::<u8>().is_ok() {
            rotation_val = arg.parse::<u8>().unwrap();
        } else if fs::read_to_string(arg).is_ok() {
            println!("{}", rotate_string(&fs::read_to_string(arg).unwrap(), rotation_val));
        } else {
            println!("{}", rotate_string(arg, rotation_val)); 
        }
    }
    Ok(())
}

pub fn rotate_string(string: &String, rotation_val: u8) -> String {
    let mut coded_string = String::from("");

    for char in string.chars() {
        coded_string.push(
            if char.is_lowercase() {
                (((char as u8) - ('a' as u8) + rotation_val) % 26 + ('a' as u8)) as char
            } else if char.is_uppercase() {
                (((char as u8) - ('A' as u8) + rotation_val) % 26 + ('A' as u8)) as char
            } else {
                char
            }
        )
    }

    coded_string
}


pub fn get_args() -> Result<Vec<String>, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err("Not enough arguments")
    } else {
        Ok(args)
    }
}
