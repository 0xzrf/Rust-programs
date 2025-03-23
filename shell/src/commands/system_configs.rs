use std::process;


pub struct SystemConfig;

impl SystemConfig {
    pub fn invalid_command(cmd: &str) {
        println!("{}: command not found", cmd)
    }

    pub fn exit(code: i32) {
        process::exit(code);
    }
}
