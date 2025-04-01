use regex::Regex;
use std::env;

use pathsearch::find_executable_in_path;

pub struct SystemExecutables;

impl SystemExecutables {
    pub fn echo(exp: &str) -> Result<(), &'static str> {
        let re = Regex::new(r"^echo\s+(.*)").unwrap();

        if let Some(mat) = re.captures(exp) {
            let val = mat.get(1).map_or("", |m| m.as_str());

            println!("{val}");
            return Ok(());
        } else {
            return Err("Unable to get the expression");
        }
    }

    pub fn handle_type(exp: &str) -> Result<(), &'static str> {
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

    pub fn handle_pwd() -> Result<(), &'static str> {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(_) => return Err("Unable to find path"),
        }

        Ok(())
    }

    pub fn handle_cd(exp: &str) -> Result<(), &'static str> {
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
}
