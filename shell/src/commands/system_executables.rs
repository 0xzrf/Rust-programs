use crate::helper_functions::*;
use std::{env, fs};

use pathsearch::find_executable_in_path;

type Res<T> = Result<T, &'static str>; // Creating a generic type to remove repetitive return value

pub struct SystemExecutables;

impl SystemExecutables {
    pub fn echo(exp: &str) -> Res<()> {
        let mut args = extract_regex_val(r"^echo\s+(.*)", exp)?;

        if args.starts_with("'") && args.ends_with("'") {
            args = &args[1..args.len() - 1];
        }

        println!("{args}");
        Ok(())
    }

    pub fn handle_type(exp: &str) -> Res<()> {
        let built_in = ["type", "exit", "echo", "pwd", "cd"];
        let valid_cmds = ["valid_command"];

        let val = extract_regex_val(r"^type\s+(.*)", exp)?;
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
        let mut val = extract_regex_val(r"^cd\s+(.*)", exp)?.to_string();

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
        let args = extract_regex_val(r"^cat\s+(.*)", exp)?;

        let mut args_iter = args.split(" ");

        while let Some(path) = args_iter.next() {
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

        Ok(())
    }
}
