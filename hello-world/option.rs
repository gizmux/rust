fn get_slice_index(slice: &[i32], i: usize) -> Option<&i32> {
    slice.get(i)
}

fn main() {
    let x = [0, 1, 2, 3];
    let s = &x;
    println!("index 7: {:?}", get_slice_index(s, 7));
    println!("index 1: {:?}", get_slice_index(s, 1));
}