use std::fmt;

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

    fn has_trunk(&self) -> bool {
        (self.number_of_doors > 4)
    }

    // Moves struct and consumes it.
    fn dismantle(self) -> (i32, i32, String) {
        (self.number_of_wheels, self.number_of_doors, self.model_name)
    }
}

// implementing the fmt::Display trait
impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Car {} with {} wheels and {} doors!",
               self.model_name, self.number_of_wheels, self.number_of_doors)
    }
}

fn main() {
    let _c = Car::new(4,5,"KIA");

    println!("{}", _c);
    println!("has_trunk is {}", _c.has_trunk());

    println!("break it up: {:?}", _c.dismantle());
}