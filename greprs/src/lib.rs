use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // first value pass
        args.next();

        let query = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get query string"),
        };

        let filename = match args.next() {
            Some(value) => {
                if value.contains(".txt") {
                    value
                } else {
                    return Err("illegal file name");
                }
            }
            None => return Err("Didn't get filename string"),
        };

        // let query = args[1].clone();
        // let filename = args[2].clone();
        let is_insensitive = env::var("IS_INSENSITIVE").is_err();

        // if !filename.contains(".txt") {
        //     return Err("illegal filename");
        // }

        Ok(Config {
            query,
            filename,
            is_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), std::io::Error> {
    let mut contents = String::new();
    let mut file = File::open(config.filename)?;
    file.read_to_string(&mut contents)?;

    let result = if config.is_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

///Search for query in contents
/// # Examples
/// ```
/// let contents = "aaa bbb ccc ddd";
/// let query = "aaa";
/// let result = search(query, contents);
/// result => "aaa"
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|c| c.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|c| c.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
