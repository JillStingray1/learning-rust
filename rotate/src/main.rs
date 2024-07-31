mod tests;

use std::process;


fn main() {
    let args = rotate::get_args().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {err}");
        process::exit(1)
    });

    if let Err(e) = rotate::run(args) {
        eprintln!("Problem parsing arguments {e}");
        process::exit(1)
    };
}
