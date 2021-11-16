use std::{env, process};
use std::error::Error;

use minigrep::Config;

fn main() {
    // Reading the Argument Values
    let args: Vec<String> = env::args().collect(); // 相比use std::env::args，然后使用args().collect()这种方式来说，可读性更好
    println!("{:?}", args);

    // Saving the Argument Values in Variables
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

