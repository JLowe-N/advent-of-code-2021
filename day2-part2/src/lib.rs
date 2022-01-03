use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("In the file {}", &config.filename);

    // let contents = fs::read_to_string(&config.filename)?;
    let file = File::open(&config.filename).unwrap();
    let reader = BufReader::new(file);

    let mut position = PositionTracker {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    // Stream file line by line using lines() iterator
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // ignores errors
        let split = line.split(" ");
        let vec: Vec<&str> = split.collect();
        let instruction = vec[0];
        let value: i32 = vec[1].parse().unwrap();
        match instruction {
            "forward" => {
                position.horizontal += value;
                position.depth += value * position.aim;
            }
            "down" => position.aim += value,
            "up" => position.aim -= value,
            _ => println!(
                "Invalid instruction found: {}, skipping line {}",
                instruction, index
            ),
        }
    }
    println!(
        "Final position\nHorizontal: {}\nDepth: {}",
        position.horizontal, position.depth
    );
    println!(
        "Multiplied result: {}",
        position.horizontal * position.depth
    );

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

struct PositionTracker {
    horizontal: i32,
    depth: i32,
    aim: i32,
}
