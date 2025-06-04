use std::io;
use std::io::Write;
use std::process::exit;

#[derive(Debug, PartialEq)]
enum Shape {
    Square,
    Circle,
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
        "1" => Some(Shape::Square),
        "2" => Some(Shape::Circle),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test] // This attribute is required to tell cargo test this is a test
	fn parse_shape_valid_input() {
		assert_eq!(parse_string("1\n"), Some(Shape::Square));
		assert_eq!(parse_string("2   "), Some(Shape::Circle));
	}
}
