use minigrep::Config;
use std::process;

fn main() {
    /* Initializes command line arguments and passes it to the main program 
       logic to be executed, and handles errors
       */
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {err}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
