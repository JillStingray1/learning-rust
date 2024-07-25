// in this function, x goes out of scope before r is used
// this causes an error
// fn example() {
//     let r;
//     {
//         let x = 5;
//         r = &x
//     }
//     println!("r: {r}")
// }

pub fn example() {
    let string1 = String::from("hi");
    let string2 = "hello";
    println!("the longest string is {}", longest(&string1, string2))
}

// when the x, y types are &str, the compiler does not know what the lifetime
// of the returned value should be, as we don't know if x or y is returned
// if either x or y is freed, and then we use the return value
// the reference would become invalid, which is unsafe.
// The compiler will prevent compilation as it has no way of checking the validity
// of the returned reference elsewhere in the code
// here the 'a is a lifetime annotation, which tells us that the reference
// return by this function should last at least as long as x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
