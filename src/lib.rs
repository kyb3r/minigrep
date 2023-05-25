use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        if args.len() < 3 {
            return Err("Not enough arguments provided".to_owned());
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let search_results = search(&contents, &config.query);

    for result in search_results {
        println!("{result}")
    }

    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    return results;
}
