use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Because iterators are lazy, and we're only getting the first
    // three args (`next` calls inside `Config::build`), we should be
    // safe even if the user passes many more.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}