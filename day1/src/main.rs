use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", args);
    println!("In the file {}", &config.filename);

    let contents = fs::read_to_string(&config.filename);
    println!("With text:\n {:?}", contents);
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Requires a filename argument");
        }
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}
