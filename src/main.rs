use std::env;
use std::fs;
use std::process;

mod config;
mod search;

use config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(&cfg.path).unwrap_or_else(|err| {
        println!("error reading file: {}", err);
        process::exit(1);
    });

    let results = if cfg.case_insensitive {
        search::search_case_insensitive(&cfg.query, &contents)
    } else {
        search::search(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
}