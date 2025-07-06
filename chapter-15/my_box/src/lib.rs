use std::alloc::{Layout, alloc, dealloc};
use std::ptr;

pub struct MyBox<T> {
    layout: Layout,  // Store this in order to deallocate it later
    ptr: *mut T, // Store the memory address of the value
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();

            // Create a memory address
            let ptr = alloc(layout) as *mut T;

            // Write the value to the memory address
            ptr::write(ptr, value);

            MyBox { ptr, layout }
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Firstly destroy the value the pointer points to
            ptr::drop_in_place(self.ptr);

            // Then deallocate the pointer
            dealloc(self.ptr as *mut u8, self.layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_box() {
        let value = 16;
        let my_box = MyBox::new(value);

        assert_eq!(*my_box, value);
    }
}
