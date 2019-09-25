fn adder(the_array: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..the_array.len() {
        sum += the_array[i];
    }
    sum
}


fn main() {
    let some_array = [0, 2, 4, 6, 8];
    println!("summing array {:?}", some_array);

    let res = adder(&some_array);
    println!("result is {}", res);
}