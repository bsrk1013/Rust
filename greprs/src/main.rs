extern crate greprs;

use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = greprs::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    greprs::run(config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
