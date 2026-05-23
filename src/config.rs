pub struct Config {
    pub query: String,
    pub path: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("usage: cargo run -- <query> <file>");
        }
        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
            case_insensitive: args.contains(&String::from("--ignore-case")),
        })
    }
}