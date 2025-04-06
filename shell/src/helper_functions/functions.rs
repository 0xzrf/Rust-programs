use regex::Regex;

pub fn parse_shell_like_args(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            '\\' if in_double_quote => {
                match chars.peek() {
                    Some('"') | Some('\\') | Some('$') | Some('\n') => {
                        // Consume the next char and push it literally
                        current.push(chars.next().unwrap());
                    }
                    Some(other) => {
                        // Leave the backslash and push as-is
                        current.push('\\');
                        current.push(*other);
                        chars.next(); // consume the peeked char
                    }
                    None => {
                        current.push('\\');
                    }
                }
            }
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}


/// This function replaces multiple whitespaces between words in a string to only one whitespace between them
pub fn normalize_whitespace(input: &str) -> String {
    let re = Regex::new(r"[ \t]+").unwrap();
    let trimmed = input.trim();
    re.replace_all(trimmed, " ").to_string()
}
