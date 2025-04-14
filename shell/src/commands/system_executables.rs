use crate::helper_functions::*;
use std::{env, fs};

use pathsearch::find_executable_in_path;

type Res<T> = Result<T, &'static str>; // Creating a generic type to remove repetitive return value

pub struct SystemExecutables {
    _cmd: String,
    args: String
}

impl SystemExecutables {
    /// Using a build function from start is always better then manual build since you can always add constraints to it
    /// like in this case, I can validate whether the cmd was a valid command or not and then print an error if it was not
    pub fn build(cmd: &str, args: &str) -> Self {
        SystemExecutables { _cmd:  cmd.to_string(), args: args.to_string() }
    }

    pub fn echo(&self) -> Res<()> {
        let parsed = parse_shell_like_args(&self.args); // Parsing the argument to eventually show in the terminal

        println!("{}", parsed.join(" "));
        Ok(())
    }

    pub fn handle_type(&self) -> Res<()> {
        let built_in = ["type", "exit", "echo", "pwd", "cd"]; 
        let valid_cmds = ["valid_command"];
        let args = &self.args;

        if built_in.contains(&&args[..]) {
            println!("{} is a shell builtin",args);
        } else if let Some(path) = find_executable_in_path(&args) {
            println!("{args} is {}", path.to_str().unwrap())
        } else if valid_cmds.contains(&&args[..]) {
            let path = env::var("PATH").unwrap(); // We can unwrap since PATH env. variable is always set

            let mut dir = "";
            if path.contains(":") {
                let mut directories = path.split(":");

                dir = directories.next().unwrap();
            } else {
                dir = &path[..]
            }

            println!("{args} is {}", dir);
        } else {
            println!("{args}: not found");
        }

        Ok(())
    }

    pub fn handle_pwd(&self) -> Res<()> {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(_) => return Err("Unable to find path"),
        }

        Ok(())
    }

    pub fn handle_cd(&self) -> Res<()> {

        let mut args = self.args.clone();

        if self.args.starts_with("~") {
            let val_cloned = args.clone();

            let (_, remaining_dir) = val_cloned.split_at(1); // splitting right after ~ to get the remaining path

            let mut home = "".to_string();
            if let Ok(usr) = env::var("HOME") {
                // Getting the current user's name
                home = usr;
            }
            args = format!("{home}{remaining_dir}");
        }

        if env::set_current_dir(&args).is_err() {
            println!("cd: {args}: No such file or directory");
        }
        Ok(())
    }

    pub fn handle_cat(&self) -> Res<()> {
        let file_paths : Vec<String> = parse_shell_like_args(&self.args);
        let mut redirect: bool = false;
        let mut content: Vec<String> = vec![];

        for path in file_paths {
            if path == ">".to_string() {
                redirect = true;
                continue
            }

            if redirect == true {
                if let Err(_) = fs::write(&path, content.join("")) {
                    return Err("Unable to write")
                }
            }

            if let Ok(output) = fs::read_to_string(&path) {
                content.push(output);
            } else {
                println!("cat: {path}: No such file or directory");
            }
        }
    
        if !redirect {
            println!("{}", content.join(""));
        }

        Ok(())
    }
}
