use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("In the file {}", &config.filename);

    let contents = fs::read_to_string(&config.filename)?;
    println!("With text:\n {:?}", contents);
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
