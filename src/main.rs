use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            println!("Invalid arguments: {e}");
            process::exit(1);
        }
    };

    println!("Searching for '{}' in {}", config.query, config.filename);

    if let Err(e) = run(&config) {
        println!("Error: {e:?}");
    }
}
