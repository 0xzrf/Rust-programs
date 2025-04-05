use regex::Regex;
type Res<T> = Result<T, &'static str>; // Creating a generic type to remove repetitive return value


/// This function replaces multiple whitespaces between words in a string to only one whitespace between them
pub fn normalize_whitespace(input: &str) -> String {
    let re = Regex::new(r"[ \t]+").unwrap();
    let trimmed = input.trim();
    re.replace_all(trimmed, " ").to_string()
}
