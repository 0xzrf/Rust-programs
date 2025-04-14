use std::process;
use std::process::Command;

pub struct SystemConfig;

impl SystemConfig {
    pub fn execute_cmd(cmd: &str) -> Result<(), ()>  {
        // Split the command into executable and args while preserving quotes
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_quotes = None;
        
        for c in cmd.chars() {
            match c {
                '"' | '\'' => {
                    if in_quotes == Some(c) {
                        in_quotes = None;
                    } else if in_quotes.is_none() {
                        in_quotes = Some(c);
                    } else {
                        current.push(c);
                    }
                }
                ' ' => {
                    if in_quotes.is_none() {
                        if !current.is_empty() {
                            parts.push(current);
                            current = String::new();
                        }
                    } else {
                        current.push(c);
                    }
                }
                _ => current.push(c),
            }
        }
        if !current.is_empty() {
            parts.push(current);
        }

        if parts.is_empty() {
            return Err(());
        }

        let executable = &parts[0];
        let args = &parts[1..];

        let mut child = match Command::new(executable)
            .args(args)
            .spawn() {
                Ok(val) => val,
                Err(_) => {
                    return Err(());
                }
            };

        match child.wait() {
            Ok(_) => return Ok(()),
            Err(_) => return Err(())
        };
    }

    pub fn exit(code: &str) {
        if let Ok(val) = code.parse() {
            process::exit(val); 
        }
    }
}
