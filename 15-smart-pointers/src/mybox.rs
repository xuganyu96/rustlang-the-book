use std::fmt;
use std::ops::{Deref, DerefMut};

/// # Singly Linked List
/// A singly linked list is either empty `Nil` or non-empty 
/// `Elem(i32, <pointer to another list>)`. However, you cannot define
/// recursive type because the compiler won't know how much space to allocate
/// on the stack, so instead you use a `Box` so that data of unknown size can
/// be stored on the heap while data of known size (a pointer) can be stored
/// on the stack
#[derive(Debug)]
pub enum List<T: fmt::Display> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T: fmt::Display> List<T> {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        match self {
            Self::Nil => s.push_str("Nil"),
            Self::Cons(t, p) => {
                let elem_str = format!("{}, ", t);
                s.push_str(&elem_str);
                // Because "self" is a reference to a List, "p" is a reference
                // to the "Box". Therefore "*p" is the "Box" itself, or in
                // other words, "*p" is a pointer to another List object.
                s.push_str(&(*p).to_string());
            },
        }

        return s;
    }
}

/// # MyBox
/// Illustrate how automatic dereference works by implementing our own "smart
/// pointer" type
pub struct MyBox<T>(pub T);

impl<T> MyBox<T> {
    pub fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    /// With "Deref" trait implemented, you can use the deference operator on
    /// a box object:
    ///
    /// Example:
    /// ```
    /// use smart_pointers::mybox::MyBox;
    /// let mybox = MyBox("Hello, world");
    /// assert_eq!(*mybox, "Hello, world");
    /// ```
    ///
    /// Calling `*mybox` is the same as `*(mybox.deref())`
    fn deref(&self) -> &Self::Target {
        let MyBox(t) = self;
        return t;
    }
}

/// Given name, return a String that uses the input name to generate a
/// greeting phrase
pub fn generate_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

/// # Mutable dereference
/// Recall that by implementing the Deref trait for some type T with target
/// type U, Rust compiler can coerce type &T into type &U.
/// Similarly, by implementing the MutDeref trait for some type T with target
/// type U, Rust compiler will be able to coerce type &mut T into type &mut U.
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T> {
    /// The "drop" method will be called when a value of this type goes out of
    /// scope. This is equivalent to a "destructor" method in C++, or the
    /// "free()" function in C, but it cannot be explicitly called (I guess to
    /// prevent double freeing?)
    fn drop(&mut self) {
        println!("Going out of scope");
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_cons_list() {
        let list = List::Cons(0, Box::new(List::Cons(1, Box::new(List::Nil))));
        assert_eq!(&list.to_string(), "0, 1, Nil");
    }

    #[test]
    fn dereference_my_box() {
        let mybox = MyBox::new("Hello, box");
        assert_eq!(*mybox, "Hello, box");
    }

    /// Rust compiler can automatically coerce a reference to a "Deref" type
    /// into a reference to the target type (recall that the Deref trait
    /// requires that an associated type "Target" be declared)
    #[test]
    fn implicit_dereference_coercion() {
        let name = MyBox::new("Steve Jobs".to_string());
        // name has type "MyBox<Target = String>", so &name is a reference to
        // "MyBox<Target = String>". When &name is passed into the function,
        // Rust compiler coerced "reference to MyBox" into "reference to 
        // String"
        //
        // However, "reference to String" (&String) is still not the correct
        // type since the function takes a string slice &str. This code works
        // because the "String" type also implements "Deref" with 
        // String::deref() -> &str, and the Rust compiler will coerce deref
        // for as many times as needed to get the right type
        let greeting = generate_greeting(&name);

        assert_eq!(&greeting, "Hello, Steve Jobs!");
    }

    #[test]
    fn mutable_dereference() {
        let mut name = MyBox::new("Steve Jobs".to_string());
        let nameref = &mut name;

        // Given "T<Target = U>: Deref", Rust compiler can coerce &mut T
        // into &U (mutable reference to pointer into immutable reference to
        // data).
        let greeting = generate_greeting(nameref);
        assert_eq!(&greeting, "Hello, Steve Jobs!");

        // Rust compiler can also coerce &mut T into &mut U (mutable reference
        // to pointer into mutable reference to data).
        // In the example below, "&mut name" is a mutable reference to the type
        // MyBox<Target=String>, so the Rust compiler can coerce it into 
        // &mut String, which is the type for String::insert_str 
        (*nameref).insert_str(0, "Hello, ");  // explicitly dereference 
        nameref.push_str("!"); // implicitly coerce
        assert_eq!(*name, "Hello, Steve Jobs!");
    }

    #[test]
    fn force_early_drop() {
        use std::mem;
        let name = MyBox::new("Steve Jobs".to_string());
        mem::drop(name);
    }
}
