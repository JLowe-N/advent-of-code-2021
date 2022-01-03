use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("In the file {}", &config.filename);

    // let contents = fs::read_to_string(&config.filename)?;
    let file = File::open(&config.filename).unwrap();
    let reader = BufReader::new(file);

    // Stream file line by line using lines() iterator
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();
    let mut num_increases: u32 = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // ignores errors
        let number = line.parse().unwrap();

        if first.len() < 3 {
            first.push(number);
        }
        if index > 0 && second.len() < 3 {
            second.push(number);
        }
        if first.len() == 3 && second.len() == 3 {
            let first_sum: i32 = first.iter().sum();
            let second_sum: i32 = second.iter().sum();
            if second_sum > first_sum {
                num_increases += 1;
            }
            first.rotate_left(1);
            first.pop();
            let pass_to_first = second[2];
            second.rotate_left(1);
            second.pop().unwrap();
            first.push(pass_to_first);
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
