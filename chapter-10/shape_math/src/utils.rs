use std::io::{self, Write};

use crate::{circle::Circle, decagon::Decagon, square::Square, Shape, UserGenerated};

pub fn get_shape() -> Option<Shape> {
    print!("Pick a shape:\n1) Square\n2) Circle\n3) Decagon\n-> ");
    std::io::stdout()
        .flush()
        .expect("Error flushing stdout, please try again.");

    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer) // Read the user input into a variable
        .expect("Failed to read line");

    parse_string(&buffer)
}

fn parse_string(input: &str) -> Option<Shape> {
    match input.trim() {
        "1" => {
            let square = Square::get_new_shape();
            Some(Shape::Square(square))
        }
        "2" => {
            let circle = Circle::get_new_shape();
            Some(Shape::Circle(circle))
        }
        "3" => {
            let decagon = Decagon::get_new_shape();
            Some(Shape::Decagon(decagon))
        }
        _ => None,
    }
}
