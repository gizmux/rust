fn sqr(x: f64) -> f64 {
    x * x
}

fn main() {
    let mut val = 5.0;
    println!("calling sqr for {}", val );
    val = sqr(val);
    println!("got result: {}", val);
}