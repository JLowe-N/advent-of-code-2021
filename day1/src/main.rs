use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = parse_config(&args);
    println!("{:?}", args);
    println!("In the file {}", filename);

    let contents =
        fs::read_to_string(filename).expect("Something went wrong trying to read the file");

    println!("With text:\n {}", contents);
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];

    filename
}
