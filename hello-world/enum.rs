#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let d = Direction::Up;
    println!("{:?}", d);
}