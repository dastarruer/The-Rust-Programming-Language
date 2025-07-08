use std::alloc::{Layout, alloc, dealloc};
use std::ops::Deref;
use std::ptr;

/// The heap-allocated structure that a MyRc will point to
struct InnerRc<T> {
    /// Stores the value of the pointer in a Box
    value: T,

    /// Keeps track of the number of references made to the pointer
    strong_count: usize,
}

/// A simple implementation of Rc<T>
pub struct MyRc<T> {
    ptr: *mut InnerRc<T>,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        let ptr = Box::new(InnerRc {
            value,
            strong_count: 1,
        });
        let ptr = Box::into_raw(ptr);

        MyRc { ptr }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            // Remove one from reference count
            (*self.ptr).strong_count -= 1;

            // If there are no more references to the data, drop it
            if (*self.ptr).strong_count == 0 {
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(*self.ptr).value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_rc() {
        let my_rc = MyRc::new(15);
        assert_eq!(*my_rc, 15);
    }
}
