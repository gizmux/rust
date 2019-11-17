
struct Car {
    number_of_wheels: i32,
    number_of_doors: i32,
    model_name: String
}

impl Car {

    fn new(wheels: i32, doors: i32, name: &str) -> Car {
        Car {
            number_of_wheels: wheels,
            number_of_doors: doors,
            model_name: name.to_string()
        }
    }
}

fn main() {
    let c = Car::new(4,5,"KIA");
}