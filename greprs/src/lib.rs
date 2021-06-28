pub struct Config {
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
