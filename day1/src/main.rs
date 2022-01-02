use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    println!("{:?}", args);
    println!("In the file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong trying to read the file");

    println!("With text:\n {}", contents);
}

struct Config {
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let filename = args[1].clone();

    Config { filename }
}
