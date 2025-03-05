use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|  {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments, Stranger!");
        }

        let file_path = args[1].clone();
        let query = args[2].clone();

        Ok(Config { file_path, query })
    }
}
