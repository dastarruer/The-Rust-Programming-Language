pub struct Light {
    state: Option<Box<dyn State>>,
}

impl Light {
    pub fn new() -> Self {
        Light {
            state: Some(Box::new(Red {})),
        }
    }
}

trait State {
    fn change_light(self: Box<Self>) -> Box<dyn State>;
}

struct Red {}

struct Green {}

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
    println!("Hello, world!");
}
