fn dumpy(input: &str) {
    println!("{}", input)
}

fn clumpy(input: String) {
    println!("{}", input)
}

fn main() {
    let s1 = "hello party".to_string();
    dumpy(&s1);
    println!("{}", s1);
    clumpy(s1);
    // this won't work because we've moved the value
    // println!("{}", s1);
}