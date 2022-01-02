use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("{:?}", args);
    println!("In the file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong trying to read the file");

    println!("With text:\n {}", contents);
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();

        Config { filename }
    }
}
