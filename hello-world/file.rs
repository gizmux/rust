use std::env;
use std::fs::File;
use std::io::Read;
use std::io;


fn file_to_string(file_name: &str) -> Result<String,io::Error> {
    let mut file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return Ok(contents),
        Err(e) => return Err(e),
    }
} 

fn main() {
    let file_name_arg = env::args().nth(1).expect("Need a filename!");
    let file_name: String = file_name_arg.parse().expect("filename not parsable!");

    println!("Opening '{}'...", file_name);

    match file_to_string(&file_name) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("An error happened: {}", e),
    }
}