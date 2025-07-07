/// A simple implementation of Rc<T>
pub struct MyRc<T> {
    /// Stores the value of the pointer in a Box 
    value: Box<T>,

    /// Keeps track of the number of references made to the pointer
    strong_count: i32,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        MyRc {
            value: Box::new(value),
            strong_count: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_rc() {
        let my_rc = MyRc::new(15);
        assert_eq!(*my_rc.value, 15);
        assert_eq!(my_rc.strong_count, 1);
    }
}
