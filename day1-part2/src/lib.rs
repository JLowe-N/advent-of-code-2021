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
    let mut first = 0;
    let mut second = 0;
    let mut num_increases: u32 = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // ignores errors
        if index == 0 {
            first = line.parse().unwrap();
        } else if index == 1 {
            second = line.parse().unwrap();
            if second > first {
                num_increases += 1
            };
        } else {
            first = second;
            second = line.parse().unwrap();
            if second > first {
                num_increases += 1
            };
        }
    }

    println!("Depth increased {} time(s).", num_increases);

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

struct DepthTracker {
    previous_value: i32,
    next_value: i32,
    num_increases: i32,
}
