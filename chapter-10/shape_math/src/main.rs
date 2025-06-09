use std::f64::consts::PI;
use std::io;
use std::io::Write;
use std::process::exit;

pub trait Area {
    fn calculate_area(&self) -> f64;
}

#[derive(Debug, PartialEq)]
enum Shape {
    Square(Square),
    Circle,
}

#[derive(Debug, PartialEq)]
struct Square {
    height: f64,
    width: f64,
}

impl Square {
    fn new(width: f64, height: f64) -> Self {
        Square { height, width }
    }
}

impl Area for Square {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn main() {
    let shape = get_shape().unwrap_or_else(|| {
        println!("Invalid input, please try again.");
        exit(1)
    });
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
            const DEFAULT_WIDTH: f64 = 2.0;
            const DEFAULT_HEIGHT: f64 = 2.0;
            Some(Shape::Square(Square::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)))
        }
        "2" => Some(Shape::Circle),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shape_valid_input() {
        assert_eq!(
            parse_string("1\n"),
            Some(Shape::Square(Square {
                height: 2.0,
                width: 2.0
            }))
        );
        assert_eq!(parse_string("2   "), Some(Shape::Circle));
    }

    #[test]
    fn shape_area_valid_output() {
        let square = Square::new(5.0, 3.0);
        assert_eq!(square.calculate_area(), 15.0);

        let circle = Circle::new(5.0);
        assert_eq!(circle.calculate_area(), 78.53981634)
    }
}
