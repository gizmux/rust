fn dump<T>(val: &T)
where T: std::fmt::Debug {
    println!("{:?}", val)
}

fn sqr<T>(x: T) -> T::Output
where T: std::ops::Mul + Copy {
    x * x
}

fn main() {
    let x = 5.6;
    dump(&x);

    let y = 8;
    dump(&sqr(y));
}