use regex::Regex;

pub struct SystemExecutables;

impl SystemExecutables {
    pub fn echo(exp: &str) {
        // Compile the regular expression pattern to match "echo " followed by any characters
        let re = Regex::new(r"^echo\s+(.*)").unwrap();

        // Search for a match in the input string slice
        if let Some(mat) = re.captures(exp) {
            // Return the captured part after "echo "
            let val = mat.get(1).map_or("", |m| m.as_str());

            println!("{val}");
        } else {
            eprintln!("Unable to get the expression");
        }
    } 
}