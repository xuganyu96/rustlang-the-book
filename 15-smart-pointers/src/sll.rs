//! # Singly Linked List (SLL)
//! A recursive implementation of singly linked list where each element holds
//! a reference to the next element, and the list is represented by its head.

pub enum Node<T: Copy> {
    Nil,
    Cons(T, Box<Node<T>>),
}

impl<T: Copy> Node<T> {
    /// Return a new node with the input value. There will be no "new" for
    /// outputing Node::Nil because we can directly call it.
    ///
    /// The ownership of the "remainder" will be lost
    pub fn new(t: T, remainder: Box<Node<T>>) -> Node<T> {
        return Node::Cons(t, remainder);
    }

    /// A O(N) implementation of length that recursively walk from the head
    /// and return the number of non-empty nodes
    pub fn len(&self) -> usize {
        match self {
            Node::Nil => 0,
            Node::Cons(_, remainder) => 1 + &remainder.len(),
        }
    }

    /// Returns a copy of the data at index "i". If index is out of bound,
    /// return an Err
    pub fn get(&self, i: usize) -> Result<T, &'static str> {
        match self {
            Node::Nil => Err("Index out of bounds"),
            Node::Cons(t, remainder) => {
                match i {
                    0 => Ok(*t),
                    _ => remainder.get(i-1),
                }
            }
        }
    }

    /// Set the element at index i to the input val, or panic if out of bound
    /// Note that i == head.len() is considered out of bound; there is no
    /// "append" in this implementation
    pub fn set(&mut self, i: usize, val: T) {
        match self {
            Node::Nil => panic!("Index out of bounds"),
            Node::Cons(
                t,  // &mut T, hence can be dereferenced and reassigned
                remainder,  // &mut Box<Node<T>>, hence can coerce to &mut Node<T>
                ) => {
                match i {
                    0 => *t = val,
                    _ => remainder.set(i-1, val),
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Instantiate an immutable singly linked list 
    #[test]
    fn test_singly_linked_list() {
        let head: Node<i32> = Node::Nil;
        let head = Node::new(2, Box::new(head));
        let head = Node::new(1, Box::new(head));
        let mut head = Node::new(0, Box::new(head));

        assert_eq!(head.len(), 3);
        assert_eq!(head.get(0).unwrap(), 0);
        assert_eq!(head.get(1).unwrap(), 1);
        assert_eq!(head.get(2).unwrap(), 2);
        assert_eq!(head.get(3).unwrap_or_else(|_| 3), 3);

        head.set(0, 10);
        head.set(1, 10);
        head.set(2, 10);
        assert_eq!(head.get(0).unwrap(), 10);
        assert_eq!(head.get(1).unwrap(), 10);
        assert_eq!(head.get(2).unwrap(), 10);
    }
}

