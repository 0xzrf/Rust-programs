use std::io::{self, Write};

pub fn print_right(msg: &str) {
    if let Some(size) = termsize::get() {
        let width = size.cols as usize;
        let msg_len = msg.chars().count();
        if msg_len < width {
            print!("{:>width$}\r\n", msg, width = width);
        } else {
            println!("{msg}"); // fallback if msg is too long
        }
    } else {
        println!("{msg}",); // fallback if we can't detect size
    }
    io::stdout().flush().unwrap();
}
