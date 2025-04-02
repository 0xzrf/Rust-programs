use regex::Regex;
type Res<T> = Result<T, &'static str>; // Creating a generic type to remove repetitive return value

pub fn extract_regex_val<'a>(exp: &'a str, from: &'a str) -> Res<&'a str>{
    let re = Regex::new(exp).unwrap();

    if let Some(mat) = re.captures(from) {
        let val = mat.get(1).map_or("", |m| m.as_str());

        return Ok(val);
    } else {
        return Err("Unable to get the expression");
    }
}