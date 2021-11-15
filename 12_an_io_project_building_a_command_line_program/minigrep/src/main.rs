use std::env;
use std::fs;

fn main() {
    // Reading the Argument Values
    let args: Vec<String> = env::args().collect(); // 相比use std::env::args，然后使用args().collect()这种方式来说，可读性更好
    println!("{:?}", args);

    // Saving the Argument Values in Variables
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // Reading a File
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
