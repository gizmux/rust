use std::env;

fn main() {

    for arg in env::args() {
        println!("arg = {}", arg);
    }

    let argvec: Vec<String> = env::args().skip(1).collect();

    println!("argvec = {:?}", argvec);
}