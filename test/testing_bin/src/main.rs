fn main() {
    let mut x = 5;
    let y = &mut x;

    y += 1;
    
    println!("y: {}", y); 
    println!("x: {}", x);
}
