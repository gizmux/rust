fn by_ref<'a>(x: &'a mut i32, id: &str) -> &'a mut i32 {
    *x = *x + 1;
    println!("{}: x just changed to {}", id, *x);
    x
}

fn main() {
    let mut val = 2;
    println!("calling func with {}", val);

    let foo = *by_ref(&mut val, "first") + 1;

    *by_ref(&mut val, "second ?") = *by_ref(&mut val, "third ?") + 2;

    println!("val is now {}, foo is {}", val, foo);
}