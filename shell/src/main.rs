
use std::process;

use codecrafters_shell::run;

fn main() {
    match run() {
        Ok(val) => val,
        Err(err) => {
            eprint!("{err}");
            process::exit(1);
            
        }
    }
}
