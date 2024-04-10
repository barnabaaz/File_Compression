use compression::{run, Config};
use std::env::args;
use std::process;
fn main() {
    let compression = Config::new(args()).unwrap_or_else(|err| {
        eprintln!("[!] problem passing arguments {}", err);
        process::exit(1)
    });
    if let Err(e) = run(compression) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
