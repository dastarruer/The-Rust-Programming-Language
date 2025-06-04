use std::io;
use std::io::Write;
use std::process::exit;
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
    
    match buffer.as_str() {
        "1" => Some(Shape::Square),
        "2" => Some(Shape::Circle),
        _ => None
    }
}

// #[cfg(test)]
// mod tests {
// 	use super::* // Import everything from the file

// 	#[test] // This attribute is required to tell cargo test this is a test
// 	fn it_works() {
// 		assert_eq!(2 + 2, 4);
// 	}
// }
