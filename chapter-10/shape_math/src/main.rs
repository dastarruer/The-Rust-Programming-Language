use std::io;
use std::io::Write;
use std::process::exit;

mod square;
mod circle;

use crate::square::Square;
use crate::circle::Circle;

pub trait UserGenerated {
    fn get_new_shape() -> Self;
}
pub trait Area {
    fn calculate_area(&self) -> f64;
    fn print_area(&self);
}

#[derive(Debug, PartialEq)]
enum Shape {
    Square(Square),
    Circle(Circle),
}

fn main() {
    let shape = get_shape().unwrap_or_else(|| {
        println!("Invalid input, please try again.");
        exit(1)
    });

    match shape {
        Shape::Square(square) => square.print_area(),
        Shape::Circle(circle) => circle.print_area(),
    }
}

fn get_shape() -> Option<Shape> {
    print!("Pick a shape:\n1) Square\n2) Circle\n-> ");
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
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_area_valid_output() {
        let square = Square::new(5.0, 3.0);
        assert_eq!(square.calculate_area(), 15.0);

        let circle = Circle::new(5.0);
        assert_eq!(circle.calculate_area(), 78.53981633974483)
    }
}
