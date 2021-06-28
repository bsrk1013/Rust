use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let content = get_file_content(&config.filename).expect("read content failed...");
    println!(
        "content
---------------------------------------------
{}
---------------------------------------------",
        content
    );
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        if !filename.contains(".txt") {
            return Err("illegal filename");
        }

        Ok(Config { query, filename })
    }
}

fn get_file_content(filename: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;

    Ok(content.clone())
}
