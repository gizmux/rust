#[derive(Debug)]
struct FloatRange {
    start: f64,
    end: f64,
    step: f64
}

impl Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let current = self.start;
            self.start += self.step;
            Some(current)
        }
    }
}

fn frange(s: f64, e: f64, st: f64) -> FloatRange {
    FloatRange { start: s, end: e, step: st }
}

fn main() {
    let x = frange(0.5, 12.0, 0.3);
    println!("{:?}", x);

    for i in x {
        println!("{:.2}", i);
    }
}