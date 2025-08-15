use std::io::{self, Write};

pub fn print_welcome_message() {
    // ANSI color codes
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const CYAN: &str = "\x1b[36m";
    const YELLOW: &str = "\x1b[33m";
    const GREEN: &str = "\x1b[32m";
    const MAGENTA: &str = "\x1b[35m";
    const DIM: &str = "\x1b[2m";

    // Clear screen and move cursor to top
    print!("\x1b[2J\x1b[1;1H");

    let welcome_art = format!(
        r#"
{BOLD}{CYAN}╔═════════════════════════════════════════════════════════════════════════╗
║                                                                         ║
║  {YELLOW}████████╗███████╗██████╗ ███╗   ███╗ ██████╗██╗  ██╗ █████╗ ████████╗{CYAN}  ║
║  {YELLOW}╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██╔════╝██║  ██║██╔══██╗╚══██╔══╝{CYAN}  ║
║  {YELLOW}   ██║   █████╗  ██████╔╝██╔████╔██║██║     ███████║███████║   ██║   {CYAN}  ║
║  {YELLOW}   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║     ██╔══██║██╔══██║   ██║   {CYAN}  ║
║  {YELLOW}   ██║   ███████╗██║  ██║██║ ╚═╝ ██║╚██████╗██║  ██║██║  ██║   ██║   {CYAN}  ║
║  {YELLOW}   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   {CYAN}  ║
║                                                                         ║
║  {GREEN}Welcome to TermChat - Your Terminal Chat Experience!{CYAN}                   ║
║                                                                         ║
╚═════════════════════════════════════════════════════════════════════════╝{RESET}"#
    );

    println!("{welcome_art}");

    // Feature highlights
    println!("{BOLD}{MAGENTA}🚀 Features:{RESET}");
    println!("{GREEN}   • Join multiple chat rooms simultaneously",);
    println!("   • Real-time messaging with other users");
    println!("   • Create rooms");
    println!("   • Simple command-based interface");
    println!("   • Cross-platform terminal support{RESET}");

    print_help();

    println!();
    println!("{BOLD}{CYAN}💡 Tips:{RESET}");
    println!("{DIM}   • Type your message and press Enter to send",);
    println!(
        "{DIM}   • if you want to write commands while in a room, write a messages starting with /",
    );
    println!("{DIM}   • eg. /leave in the chat pannel will make you leave the room",);
    println!(
        "{DIM}   • If you want to write a message that starts with /(donno why), just write // instead(I'll handle the rest😉)",
    );

    println!();
    println!(
        "{BOLD}{CYAN}═══════════════════════════════════════════════════════════════════{RESET}"
    );
    println!(
        "{GREEN}Ready to chat? Start by typing {YELLOW} /join <room_number> {GREEN} or {YELLOW} /create{RESET}"
    );
    println!("{CYAN}═══════════════════════════════════════════════════════════════════{RESET}");
    println!();

    // Flush stdout to ensure everything is displayed
    io::stdout().flush().unwrap();
}

pub fn print_help() {
    const BOLD: &str = "\x1b[1m";
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";
    const BLUE: &str = "\x1b[34m";

    println!("{BOLD}{BLUE}📋 Quick Commands:{RESET}");
    println!("{YELLOW}   /join <room>     - Join a chat room");
    println!("   /create <room>   - Create a new room");
    println!("   /set_user        - Set the username(default to terminal user){RESET}");
    // println!("   /list            - List available rooms");
    println!("   /leave           - Leave current room");
    println!("   /help            - Show all commands");
    println!("   /quit            - Exit the application{RESET}");
}
