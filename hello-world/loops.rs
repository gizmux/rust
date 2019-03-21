fn main() {
    for i in 1..5 {
        println!("i is {}", i);

        if i < 2 {
            println!("tiny num");
        } else {
            println!("big num");
        }

        let condvar = if i % 2 == 0 { "even" } else { "odd" };
        println!("{}", condvar);
    }
}