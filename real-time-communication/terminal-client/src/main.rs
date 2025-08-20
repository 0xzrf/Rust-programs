use terminal_client::run;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
    }
}
