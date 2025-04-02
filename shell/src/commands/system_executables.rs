use regex::Regex;
use std::{env, fs};

use pathsearch::find_executable_in_path;

type Res<T> = Result<T, &'static str>; // Creating a generic type to remove repetitive return value

pub struct SystemExecutables;

impl SystemExecutables {
    pub fn echo(exp: &str) -> Res<()> {
        let mut args = SystemExecutables::extract_regex_val(r"^echo\s+(.*)", exp)?;

        if args.starts_with("'") && args.ends_with("'") {
            args = &args[1..args.len() - 1];
        }

        println!("{args}");

        Ok(())
    }

    pub fn handle_type(exp: &str) -> Res<()> {
        let built_in = ["type", "exit", "echo", "pwd", "cd"];
        let valid_cmds = ["valid_command"];

        let re = match Regex::new(r"^type\s+(.*)") {
            Ok(val) => val,
            Err(_) => {
                println!("Couldn't extract val");
                return Err("Unable extract val");
            }
        };

        if let Some(mat) = re.captures(exp) {
            let val = mat.get(1).map_or("", |m| m.as_str());
            if built_in.contains(&val) {
                println!("{val} is a shell builtin");
            } else if let Some(path) = find_executable_in_path(val) {
                println!("{val} is {}", path.to_str().unwrap())
            } else if valid_cmds.contains(&val) {
                let path = env::var("PATH").unwrap(); // We can unwrap since PATH env. variable is always set

                let mut dir = "";
                if path.contains(":") {
                    let mut directories = path.split(":");

                    dir = directories.next().unwrap();
                } else {
                    dir = &path[..]
                }

                println!("{val} is {dir}");
            } else {
                println!("{val}: not found");
            }
        }

        Ok(())
    }

    pub fn handle_pwd() -> Res<()> {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(_) => return Err("Unable to find path"),
        }

        Ok(())
    }

    pub fn handle_cd(exp: &str) -> Res<()> {
        let re = Regex::new(r"^cd\s+(.*)").unwrap();

        // Search for a match in the input string slice
        if let Some(mat) = re.captures(exp) {
            let mut val = mat.get(1).map_or("", |m| m.as_str()).to_string();

            if val.starts_with("~") {
                let val_cloned = val.clone();

                let (_, remaining_dir) = val_cloned.split_at(1); // splitting right after ~ to get the remaining path

                let mut home = "".to_string();
                if let Ok(usr) = env::var("HOME") { // Getting the current user's name
                    home = usr;
                }

                val = format!("{home}{remaining_dir}");
            }

            
            if env::set_current_dir(&val).is_err() {
                println!("cd: {val}: No such file or directory");
            }

            return Ok(());
        } else {
            return Err("Unable to get the expression");
        }
    }
    pub fn handle_cat(exp: &str) -> Res<()> {
        let re = Regex::new(r"^cat\s+(.*)").unwrap();

        if let Some(mat) = re.captures(exp) {
            let val = mat.get(1).map_or("", |m| m.as_str()).to_string();

            
            let mut args = val.split(" ");

            while let Some(path) = args.next() {
                let mut new_pattern = path; 

                if path.starts_with("'") && path.ends_with("'") {
                    new_pattern = &path[1..path.len() - 1];
                }

                if let Ok(output) = fs::read_to_string(new_pattern) {
                    println!("{output}");
                } else { 
                    println!("cat: {path}: No such file or directory");
                }
            }  
   
        }

        Ok(()) 
    }


    fn extract_regex_val<'a>(exp: &'a str, from: &'a str) -> Res<&'a str>{
        let re = Regex::new(exp).unwrap();

        if let Some(mat) = re.captures(from) {
            let val = mat.get(1).map_or("", |m| m.as_str());

            return Ok(val);
        } else {
            return Err("Unable to get the expression");
        }
    }

}
