use std::{io::Write, process::exit};
use std::io;

use crate::{Area, UserGenerated};

#[derive(Debug, PartialEq)]
pub struct Square {
    height: f64,
    width: f64,
}

impl Square {
    pub fn new(width: f64, height: f64) -> Self {
        Square { height, width }
    }
}

impl Area for Square {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }

    fn print_area(&self) {
        let area = self.calculate_area();
        println!("Area of square: {}", area);
    }
}

impl UserGenerated for Square {
    fn get_new_shape() -> Self {
        let mut height = String::new();
        let mut width = String::new();

        // Ask for width
        print!("Input square width: ");
        std::io::stdout()
            .flush()
            .expect("Error flushing stdout, please try again.");

        // Take user inputted width
        io::stdin()
            .read_line(&mut width) // Read the user input into a variable
            .expect("Failed to read line");

        let width: u64 = width.trim().parse().unwrap_or_else(|_| {
            println!("Invalid input, please try again.");
            exit(1)
        });
        let width = width as f64;

        // Ask for height
        print!("Input square height: ");
        std::io::stdout()
            .flush()
            .expect("Error flushing stdout, please try again.");

        // Take user inputted height
        io::stdin()
            .read_line(&mut height) // Read the user input into a variable
            .expect("Failed to read line");

        let height: u64 = height.trim().parse().unwrap_or_else(|_| {
            println!("Invalid input, please try again.");
            exit(1)
        });
        let height = height as f64;

        // Return a new square
        Self::new(width, height)
    }
}
