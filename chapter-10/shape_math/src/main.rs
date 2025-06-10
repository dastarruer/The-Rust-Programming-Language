use std::io;
use std::io::Write;
use std::process::exit;

mod circle;
mod decagon;
mod square;

use crate::circle::Circle;
use crate::decagon::Decagon;
use crate::square::Square;

pub trait UserGenerated {
    fn get_new_shape() -> Self;
}
pub trait Area {
    fn calculate_area(&self) -> f64;
    fn print_area(&self);
}

pub trait Perimeter {
    fn calculate_perimeter(&self) -> f64;
    fn print_perimeter(&self);
}

#[derive(Debug, PartialEq)]
enum Shape {
    Square(Square),
    Circle(Circle),
    Decagon(Decagon),
}

fn main() {
    let shape = get_shape().unwrap_or_else(|| {
        println!("Invalid input, please try again.");
        exit(1)
    });

    match shape {
        Shape::Square(square) => {
            square.print_perimeter();
            square.print_area();
        }
        Shape::Circle(circle) => {
            circle.print_perimeter();
            circle.print_area();
        }
        Shape::Decagon(decagon) => todo!()
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
    fn shape_area_test() {
        let square = Square::new(5.0, 3.0);
        assert_eq!(square.calculate_area(), 15.0);

        let circle = Circle::new(5.0);
        assert_eq!(circle.calculate_area(), 78.53981633974483);

        let decagon = Decagon::new(15.0);
        assert_eq!(decagon.calculate_area(), 1731.1969896610801);
    }

    #[test]
    fn shape_perimetertest() {
        let square = Square::new(5.0, 3.0);
        assert_eq!(square.calculate_perimeter(), 16.0);

        let circle = Circle::new(5.0);
        assert_eq!(circle.calculate_perimeter(), 31.41592653589793);

        let decagon = Decagon::new(15.0);
        assert_eq!(decagon.calculate_perimeter(), 150.0);
    }
}
