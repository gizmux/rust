use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_name_arg = env::args().nth(1).expect("Need a filename!");
    let file_name: String = file_name_arg.parse().expect("filename not parsable!");

    println!("Opening '{}'...", file_name);
    let mut file = File::open(file_name).expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("couldn't read!");

    println!("dump: {}", contents);
}