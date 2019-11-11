

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(2);
    v1.push(5);

    println!("v1 = {:?}", v1);

    let mut v2 = Vec::new();
    v2.push("a");
    v2.push("b");

    println!("v2 = {:?}", v2);


    let mut v3 = vec![1.2, 1.2, 1.5, 1.456, 3.];
    for i in v3.iter() {
        println!("v3 has {}", i);
    }
}