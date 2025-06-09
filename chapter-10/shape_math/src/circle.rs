use std::{f64::consts::PI, io::{self, Write}, process::exit};

use crate::{Area, UserGenerated};

#[derive(Debug, PartialEq)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl UserGenerated for Circle {
    fn get_new_shape() -> Self {
        let mut radius = String::new();

        // Ask for radius
        print!("Input circle radius: ");
        std::io::stdout()
            .flush()
            .expect("Error flushing stdout, please try again.");

        // Take user input
        io::stdin()
            .read_line(&mut radius) // Read the user input into a variable
            .expect("Failed to read line");

        let radius: u64 = radius.trim().parse().unwrap_or_else(|_| {
            println!("Invalid input, please try again.");
            exit(1)
        });
        let radius = radius as f64;

        // Return a new circle
        Self::new(radius)
    }
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn print_area(&self) {
        let area = self.calculate_area();
        println!("Area of circle: {}", area);
    }
}
