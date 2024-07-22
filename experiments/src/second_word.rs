// Demonstration of ownership

// &str(a pointer to str) borrows the str passed to this function, instead of transfering the ownership,
// this passes just the value of the String
pub fn find_second_word(s: &str) -> &str {
    let spaces: Vec<usize> = s
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == ' ')
        .map(|(i, _)| i)
        .collect();
    if spaces.len() > 1 {
        &s[spaces[0] + 1..spaces[1]]
    } else if spaces.len() == 1 {
        &s[spaces[0] + 1..]
    } else {
        &s[..]
    }
}

pub fn example() {
    let x = String::from("sex");
    println!("{}", find_second_word(&x));
}
