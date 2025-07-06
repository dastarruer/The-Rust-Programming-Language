use my_box::MyBox;

fn main() {
    let value = MyBox::new(110923841023489102394805 as i128);

    println!("Value: {}", *value);
}
