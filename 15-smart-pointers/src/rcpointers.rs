//! reference-counted pointers can be cloned to allow multiple references to
//! the same piece of data
use std::fmt::Display;
use std::rc::Rc;

pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T: Display> List<T> {
    /// Start from self and traversing until reaching Nil. Print all element
    /// along the way
    pub fn straverse(&self) -> String {
        let mut s = String::new();
        match self {
            List::Cons(t, l) => {
                s.push_str(&format!("{t}, "));
                s.push_str(&l.straverse());
            },
            List::Nil => (),
        }

        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_straverse() {
        let list = Rc::new(List::Nil);
        let list = Rc::new(List::Cons(2, list));
        let list = Rc::new(List::Cons(1, list));
        let list = Rc::new(List::Cons(0, list));

        // Can't pass "list" directly because of ownership move; use Rc::clone
        // to copy the address. Rc::clone also increases the reference count.
        //
        // list.clone() would have worked, too. Using Rc::clone is by convention
        let first_head = Rc::new(List::Cons(-1, Rc::clone(&list)));
        let second_head = Rc::new(List::Cons(-2, Rc::clone(&list)));

        assert_eq!(&list.straverse(), "0, 1, 2, ");
        assert_eq!(&first_head.straverse(), "-1, 0, 1, 2, ");
        assert_eq!(&second_head.straverse(), "-2, 0, 1, 2, ");
    }
}
