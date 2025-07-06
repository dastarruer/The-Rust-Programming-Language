use std::alloc::{Layout, alloc};
use std::ptr;

pub struct MyBox<T> {
    ptr: *mut T, // Store the memory address of the value
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();

            // Create a memory address
            let ptr = alloc(layout) as *mut T;

            // Write the value to the memory address
            ptr::write(ptr, value);

            MyBox { ptr }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
