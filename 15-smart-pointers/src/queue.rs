//! # Queue
//! Queue is a first-in-first-out data structure and is best implemented
//! using a doubly linked list because we need to keep track of both the
//! head and the tail
//!
//! In this implementation, the queue will pop from the head and push into
//! the tail
use std::rc::{Rc, Weak};
use std::fmt::Display;
use core::cell::RefCell;

/// Individual nodes in a queue
pub enum Node<T: Copy> {
    Nil,
    Cons{
        val: T,
        prev: Weak<RefCell<Node<T>>>,
        next: Rc<RefCell<Node<T>>>,
    }
}

impl<T: Copy> Node<T> {
    pub fn set_next(&mut self, next: &Rc<RefCell<Node<T>>>) {
        match self {
            Node::Nil => panic!("Setting next on Nil"),
            Node::Cons{ val: _, prev: _, next: selfnext } => {
                *selfnext = Rc::clone(next);
            },
        }
    }

    pub fn set_prev(&mut self, prev: &Rc<RefCell<Node<T>>>) {
        match self {
            Node::Nil => panic!("Setting prev on Nil"),
            Node::Cons{ val: _, prev: selfprev, next: _ } => {
                *selfprev = Rc::downgrade(prev);
            }
        }
    }

}

impl<T: Copy + Display> Node<T> {
    pub fn to_string(&self) -> String {
        match self {
            Node::Nil => "Nil".to_string(),
            Node::Cons{ val, prev: _, next: _ } => format!("{val}"),
        }
    }
}

/// The queue data structure. Unlike a stack which only needs to track its
/// head, the queue needs to track both the head and the tail, which is why
/// a separate struct for capturing both head and tail (and conveniently
/// the constant time "len").
///
/// # Design choice
/// Through out the lifecycle of a queue there is exactly one Node::Nil.
/// When the queue is empty, both head and tail points to this Nil.
/// When the queue has 1 element, both head and tail will point to this elem,
/// and the elem's prev and next both points to this Nil.
/// (in some sense this is a circular linked list, and the Nil is the
/// "sentinel" node)
pub struct Queue<T: Copy> {
    head: Rc<RefCell<Node<T>>>,
    tail: Rc<RefCell<Node<T>>>,
    _len: usize,  // hidden, can only be read using the len() method
    _sentinel: Rc<RefCell<Node<T>>>,  // the one and only sentinel node
}

impl<T: Copy> Queue<T> {
    /// Return an empty queue
    pub fn new() -> Queue<T> {
        let sentinel = Rc::new(RefCell::new(Node::Nil));
        return Queue {
            head: Rc::clone(&sentinel),
            tail: Rc::clone(&sentinel),
            _len: 0,
            _sentinel: sentinel,
        };
    }

    /// Add an element to the tail-end of the queue
    pub fn push(&mut self, val: T) {
        match self._len {
            0 => {  // create new node then set both head and tail to new node
                let new_node = Rc::new(RefCell::new(Node::Cons{
                    val,
                    prev: Rc::downgrade(&self._sentinel),
                    next: Rc::clone(&self._sentinel),
                }));
                self.head = Rc::clone(&new_node);
                self.tail = Rc::clone(&new_node);
                self._len = 1;
            },
            _ => {
                let new_node = Rc::new(RefCell::new(Node::Cons{
                    val,
                    prev: Rc::downgrade(&self.tail),
                    next: Rc::clone(&self._sentinel),
                }));
                self.tail.borrow_mut().set_next(&new_node);
                self.tail = Rc::clone(&new_node);
                self._len += 1;
            }
        }
    }

    /// Pop the head and return (a copy of) the element held
    pub fn pop(&mut self) -> T {
        match self._len {
            0 => panic!("Popping an empty queue!"),
            1 => {
                let output = match &*self.head.borrow() {
                    Node::Cons{ val, prev: _, next: _ } => *val,
                    _ => unreachable!(),
                };
                let sentinel = Rc::new(RefCell::new(Node::Nil));
                self.head = Rc::clone(&sentinel);
                self.tail = Rc::clone(&sentinel);
                self._len = 0;
                return output;
            },
            _ => {
                let (output, second) = match &*self.head.borrow() {
                    Node::Cons{ val, prev: _, next } => {
                        (*val, Rc::clone(next))
                    },
                    _ => unreachable!(),
                };
                self.head = Rc::clone(&second);
                self.head.borrow_mut().set_prev(&self._sentinel);
                self._len -= 1;
                return output;
            }
        }
    }

    /// Return the number of elements
    pub fn len(&self) -> usize {
        return self._len;
    }

    /// Return a copy of the value held by the head
    pub fn peek(&self) -> T {
        let head = self.head.borrow();
        match &*head {
            Node::Nil => panic!(""),
            Node::Cons{ val, prev: _, next: _ } => *val, 
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutate_node_references() {
        let sentinel: Node<i32> = Node::Nil;
        let sentinel = Rc::new(RefCell::new(sentinel));

        let tail: Node<i32> = Node::Cons{
            val: 0,
            prev: Rc::downgrade(&sentinel),
            next: Rc::clone(&sentinel),
        };

        let tail = Rc::new(RefCell::new(tail));

        let new_node: Node<i32> = Node::Cons{
            val: 1,
            prev: Rc::downgrade(&sentinel),
            next: Rc::clone(&sentinel),
        };
        let new_node = Rc::new(RefCell::new(new_node));
        
        // mutate tail's next to point to new_node, and mutate new_node's prev
        // to point to tail
        tail.borrow_mut().set_next(&new_node);
        new_node.borrow_mut().set_prev(&tail);

        let tailref = tail.borrow();
        if let Node::Cons{ val, prev: _, next } = &*tailref {
            assert_eq!(*val, 0);

            if let Node::Cons{ val, prev: _, next: _ } = &*next.borrow() {
                assert_eq!(*val, 1);
            }
        }

        let newref = new_node.borrow();
        if let Node::Cons{ val: _, prev, next: _ } = &*newref {
            if let Node::Cons{ val, prev: _, next: _ } = &*prev.upgrade().unwrap().borrow() {
                assert_eq!(*val, 0);
            }
        }
    }

    #[test]
    fn test_queue_push() {
        let mut queue: Queue<i32> = Queue::new();
        queue.push(0);
        assert_eq!(queue.peek(), 0);
        assert_eq!(queue.len(), 1);
        queue.push(1);
        assert_eq!(queue.peek(), 0);
        assert_eq!(queue.len(), 2);
        queue.push(2);
        assert_eq!(queue.peek(), 0);
        assert_eq!(queue.len(), 3);

        assert_eq!(queue.pop(), 0);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.len(), 0);
    }
}
