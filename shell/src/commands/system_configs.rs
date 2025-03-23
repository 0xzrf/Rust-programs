use std::process;
use std::process::Command;

pub struct SystemConfig;

impl SystemConfig {
    pub fn invalid_command(cmd: &str)  {
        let mut v = cmd.split(" ");

        let mut child = Command::new(v.next().unwrap())
        .args(v)
        .spawn()
        .map_err(|err| println!("{}", err.to_string()));

        child.wait().map_err(|err| err.to_string()).unwrap();
    }

    pub fn exit(code: i32) {
        process::exit(code);
    }
}
