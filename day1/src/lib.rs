use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("In the file {}", &config.filename);

    // let contents = fs::read_to_string(&config.filename)?;
    let file = File::open(&config.filename).unwrap();
    let reader = BufReader::new(file);

    // Stream file line by line using lines() iterator
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // ignores errors
                                  // do useful stuff
        println!("{}. {}", index + 1, line);
    }

    Ok(())
}

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Requires a filename argument");
        }
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}
