use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments, Stranger!");
        }

        let file_path = args[1].clone();
        let query = args[2].clone();

        Ok(Config { file_path, query })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.file_path)?;

    println!("Content of the file: \n{}", file);   

    Ok(())
}