use std::{
    io::{self, Write},
    process::exit,
};

use crate::{Area, Perimeter, UserGenerated};

#[derive(Debug, PartialEq)]
pub struct Decagon {
    side_length: f64,
}

impl Area for Decagon {
    fn calculate_area(&self) -> f64 {
        ((5.0 / 2.0) * self.side_length * self.side_length) * ((5.0 + 2.0 * 5.0_f64.sqrt()).sqrt())
    }

    fn print_area(&self) {
        let area = self.calculate_area();
        println!("Area of decagon: {}", area);
    }
}

impl Perimeter for Decagon {
    fn calculate_perimeter(&self) -> f64 {
        10.0 * self.side_length
    }

    fn print_perimeter(&self) {
        let perimeter = self.calculate_perimeter();
        println!("Perimeter of decagon: {}", perimeter)
    }
}

impl Decagon {
    pub fn new(side_length: f64) -> Self {
        Decagon { side_length }
    }
}

impl UserGenerated for Decagon {
    fn get_new_shape() -> Self {
        let mut side_length = String::new();

        // Ask for side_length
        print!("Input decagon side length: ");
        std::io::stdout()
            .flush()
            .expect("Error flushing stdout, please try again.");

        // Take user input
        io::stdin()
            .read_line(&mut side_length) // Read the user input into a variable
            .expect("Failed to read line");

        let side_length: u64 = side_length.trim().parse().unwrap_or_else(|_| {
            println!("Invalid input, please try again.");
            exit(1)
        });
        let side_length = side_length as f64;

        // Return a new circle
        Self::new(side_length)
    }
}
