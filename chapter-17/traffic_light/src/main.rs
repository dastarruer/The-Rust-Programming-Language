#[derive(Debug)]
pub struct Light {
    state: Option<Box<dyn State>>,
}

impl Light {
    pub fn new() -> Self {
        Light {
            state: Some(Box::new(Red {})),
        }
    }

    pub fn change_light(&mut self) {
        let current_state = self.state.take().unwrap();
        self.state = Some(current_state.change_light());
    }
}

// this is the only way to implement Debug for all these objects and i dont know why.
trait State: std::fmt::Debug {
    fn change_light(self: Box<Self>) -> Box<dyn State>;
}

#[derive(Debug)]
struct Red {}

#[derive(Debug)]
struct Green {}

#[derive(Debug)]
struct Yellow {}

impl State for Red {
    fn change_light(self: Box<Self>) -> Box<dyn State> {
        println!("Switched from red to green.");
        Box::new(Green {})
    }
}

impl State for Green {
    fn change_light(self: Box<Self>) -> Box<dyn State> {
        println!("Switched from green to yellow.");
        Box::new(Yellow {})
    }
}

impl State for Yellow {
    fn change_light(self: Box<Self>) -> Box<dyn State> {
        println!("Switched from yellow to red.");
        Box::new(Red {})
    }
}

fn main() {
    let mut light = Light::new();
    println!("Created new Light: {:?}\n", light);

    for i in 1..4 {
        light.change_light();
        println!("Changed light: {:?}\n", light);
    }

}
