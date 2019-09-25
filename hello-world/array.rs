
fn print_array(the_array: &[i32]) -> String {
    let mut res = String::new();
    res += "[";
    for e in the_array {
        res += &e.to_string();
        res += ", ";
    }
    //res.pop();
    let len = res.len();
    res.drain((len - 2)..);

    res.push(']');
    res
}

fn main() {
    let test = [5,6,7,8];

    println!("test = {}", print_array(&test));
}