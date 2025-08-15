use std::io::{self, Write};

/// Prints a colorful welcome message for the terminal chat application
pub fn print_welcome_message() {
    // ANSI color codes
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const CYAN: &str = "\x1b[36m";
    const YELLOW: &str = "\x1b[33m";
    const GREEN: &str = "\x1b[32m";
    const BLUE: &str = "\x1b[34m";
    const MAGENTA: &str = "\x1b[35m";
    const DIM: &str = "\x1b[2m";

    // Clear screen and move cursor to top
    print!("\x1b[2J\x1b[1;1H");

    let welcome_art = format!(
        r#"
{}{}╔═════════════════════════════════════════════════════════════════════════╗
║                                                                         ║
║  {}████████╗███████╗██████╗ ███╗   ███╗ ██████╗██╗  ██╗ █████╗ ████████╗{}  ║
║  {}╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██╔════╝██║  ██║██╔══██╗╚══██╔══╝{}  ║
║  {}   ██║   █████╗  ██████╔╝██╔████╔██║██║     ███████║███████║   ██║   {}  ║
║  {}   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║     ██╔══██║██╔══██║   ██║   {}  ║
║  {}   ██║   ███████╗██║  ██║██║ ╚═╝ ██║╚██████╗██║  ██║██║  ██║   ██║   {}  ║
║  {}   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   {}  ║
║                                                                         ║
║  {}Welcome to TermChat - Your Terminal Chat Experience!{}                   ║
║                                                                         ║
╚═════════════════════════════════════════════════════════════════════════╝{}"#,
        BOLD,
        CYAN,
        YELLOW,
        CYAN,
        YELLOW,
        CYAN,
        YELLOW,
        CYAN,
        YELLOW,
        CYAN,
        YELLOW,
        CYAN,
        YELLOW,
        CYAN,
        GREEN,
        CYAN,
        RESET
    );

    println!("{}", welcome_art);

    // Feature highlights
    println!("{}{}🚀 Features:{}", BOLD, MAGENTA, RESET);
    println!("{}   • Join multiple chat rooms simultaneously", GREEN);
    println!("   • Real-time messaging with other users");
    println!("   • Create rooms");
    println!("   • Simple command-based interface");
    println!("   • Cross-platform terminal support{}", RESET);

    println!();
    println!("{}{}📋 Quick Commands:{}", BOLD, BLUE, RESET);
    println!("{}   /join <room>     - Join a chat room", YELLOW);
    println!("   /create <room>   - Create a new room");
    println!(
        "   /set_user        - Set the username(default to terminal user){}",
        RESET
    );
    println!("   /list            - List available rooms");
    println!("   /users           - Show users in current room");
    println!("   /leave           - Leave current room");
    println!("   /help            - Show all commands");
    println!("   /quit            - Exit the application{}", RESET);

    println!();
    println!("{}{}💡 Tips:{}", BOLD, CYAN, RESET);
    println!("{}   • Type your message and press Enter to send", DIM);
    println!(
        "{}   • if you want to write commands while in a room, write a messages starting with /",
        DIM
    );
    println!(
        "{}   • eg. /leave in the chat pannel will make you leave the room",
        DIM
    );
    println!(
        "{}   • If you want to write a message that starts with /(donno why), just write // instead(I'll handle the rest😉)",
        DIM
    );

    println!();
    println!(
        "{}{}═══════════════════════════════════════════════════════════════════{}",
        BOLD, CYAN, RESET
    );
    println!(
        "{}Ready to chat? Start by typing {} /join <room_name> {} or {} /create <room_name>{}",
        GREEN, YELLOW, GREEN, YELLOW, RESET
    );
    println!(
        "{}═══════════════════════════════════════════════════════════════════{}",
        CYAN, RESET
    );
    println!();

    // Flush stdout to ensure everything is displayed
    io::stdout().flush().unwrap();
}
