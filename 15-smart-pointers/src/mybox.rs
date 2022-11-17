//! # My (Stupid) Box
//! Implement my own stupid pointer as an illustration of the Deref and DerefMut traits
use std::ops::{Deref, DerefMut};

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(t: T) -> MyBox<T> {
        return MyBox(t);
    }
}

impl<T> Deref for MyBox<T> {
    // Specifying "Target" type allows Rust compiler to know what your
    // "pointer" actually points to
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let MyBox(t) = self;
        return t;
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let MyBox(t) = self;
        return t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dereference() {
        let mut mybox = MyBox::new(10);
        assert_eq!(*mybox, 10);  // check dereference operator works
        
        *mybox = 20; // check mutable deferencing works
        assert_eq!(*mybox, 20);
    }
}
