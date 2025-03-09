use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a file path")
        };

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get the query parameter")
        };

        Ok(Config { file_path, query, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.file_path)?;

    let result = if !config.ignore_case {
        search(&config.query, &file)
    } else {
        search_case_insensitive(&config.query, &file)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Rust:";
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

        assert_eq!(vec!("Rust:"), search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
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


        assert_eq!(vec!["Rust:"], search_case_insensitive(query, content));
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


pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
