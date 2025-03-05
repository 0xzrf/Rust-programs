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

    
    for line in search(&config.query, &file) {
        println!("{line}");
    }


    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "safe";
        let content = "\
Rust:
safe and secure
safety is something very important,
you are safe
but I'm not safe
what is something
you are what?
safe is safe
";

        dbg!(search(query, content));

        assert_eq!(vec!("safe and secure"), search(query, content));

    }
}


pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}