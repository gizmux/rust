#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

fn main() {
    let mut d = Direction::Up;
    println!("{:?}", d);

    for _ in 1..10 {
        println!("direction = {:?}", d);
        d = d.next();
    }
}