use crate::helper_functions::*;
use std::{env, fs};

use pathsearch::find_executable_in_path;

type Res<T> = Result<T, &'static str>; // Creating a generic type to remove repetitive return value

pub struct SystemExecutables {
    pub cmd: String,
    pub args: String
}

impl SystemExecutables {

    /// @exp is the intire input of the command(eg. echo hello world)
    /// @cmd_exp is how the build function will extracts the args from the input variable
    pub fn build(cmd_exp: String, exp: &str) -> Self {
        let args = extract_args_from_cmd(&cmd_exp, exp).unwrap().to_string();

        let cmd = exp.split(" ").next().unwrap().to_string();

        SystemExecutables { cmd, args }
    }

    pub fn echo(exp: &str) -> Res<()> {
        let args = extract_args_from_cmd(r"^echo\s+(.*)", exp)?.to_string();

        let parsed = SystemExecutables::parse_shell_like_args(&args);

        println!("{}", parsed.join(" "));
        Ok(())
    }

    pub fn handle_type(exp: &str) -> Res<()> {
        let built_in = ["type", "exit", "echo", "pwd", "cd"];
        let valid_cmds = ["valid_command"];

        let val = extract_args_from_cmd(r"^type\s+(.*)", exp)?;
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
        let mut val = extract_args_from_cmd(r"^cd\s+(.*)", exp)?.to_string();

        if val.starts_with("~") {
            let val_cloned = val.clone();

            let (_, remaining_dir) = val_cloned.split_at(1); // splitting right after ~ to get the remaining path

            let mut home = "".to_string();
            if let Ok(usr) = env::var("HOME") {
                // Getting the current user's name
                home = usr;
            }
            val = format!("{home}{remaining_dir}");
        }

        if env::set_current_dir(&val).is_err() {
            println!("cd: {val}: No such file or directory");
        }
        Ok(())
    }

    pub fn handle_cat(exp: &str) -> Res<()> {
        let args = extract_args_from_cmd(r"^cat\s+(.*)", exp)?;
        let file_paths = SystemExecutables::parse_shell_like_args(args);
    
        for path in file_paths {
            if let Ok(output) = fs::read_to_string(&path) {
                println!("{output}");
            } else {
                println!("cat: {path}: No such file or directory");
            }
        }
    
        Ok(())
    }
    

    fn parse_shell_like_args(input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut chars = input.chars().peekable();
        let mut in_single_quote = false;
    
        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    // Toggle single quote mode
                    if in_single_quote {
                        in_single_quote = false;
                    } else {
                        in_single_quote = true;
                    }
                }
                ' ' | '\t' if !in_single_quote => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    // skip the space
                }
                _ => {
                    current.push(c);
                }
            }
        }
    
        if !current.is_empty() {
            tokens.push(current);
        }
    
        tokens
    }
}
