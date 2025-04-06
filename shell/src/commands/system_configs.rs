use std::process;
use std::process::Command;

pub struct SystemConfig;

impl SystemConfig {
    pub fn execute_cmd(cmd: &str) -> Result<(), ()>  {
        let mut v = cmd.split(" ");

        let mut child = match Command::new(v.next().unwrap())
        .args(v)
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
