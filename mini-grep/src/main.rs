use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let file = fs::read_to_string(&config.file_path);

    let content = match file {
        Ok(file) => file,
        Err(_e) => {
            panic!("No such file");
        }
    };

    println!("Content of the file: \n{}", content);
}

struct Config {
    file_path: String,
    query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let file_path = args[1].clone();
        let query = args[2].clone();

        Config { file_path, query }
    }
}

