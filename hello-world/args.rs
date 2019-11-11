use std::env;

fn main() {

    for arg in env::args() {
        println!("arg = {}", arg);
    }

    let argvec: Vec<String> = env::args().skip(1).collect();

    println!("argvec = {:?}", argvec);

    let file_name_arg = env::args().nth(1).expect("no filename given !");
    let file_name: String = file_name_arg.parse().expect("not a string");

    println!("Opening '{}'...", file_name);
}