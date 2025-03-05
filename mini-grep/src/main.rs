use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();    

    let file_path = &args[1];
    let query = &args[2];

    let file = fs::read_to_string(file_path);

    let content = match file {
        Ok(file) => file,
        Err(_e) => {
            panic!("No such file");
        } 
    };
    
    println!("Content of the file: \n{}", content);

}
