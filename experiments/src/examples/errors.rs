#[allow(dead_code)]
fn panic() {
    // this causes a panic, which is an unrecoverable error
    panic!("i died");
}
// errors that are meant to be handled should be using the Error enum type
// unwrap can get value inside of the generic
