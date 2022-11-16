//! 15.6 Reference cycles can leak memories
//! Let's build a tree data structure. A tree is conceptually a collection of
//! nodes where each node has zero or more children. Programmatically the tree
//! will be represented using its root node (which might be empty).
use std::rc::{Rc, Weak};
use core::cell::RefCell;

/// Representing a node. A non-empty node should include the value it holds,
/// references to its children, and references to its parent.
///
/// Allowing a node to reference its children and its parent means that there
/// could be multiple references to the same node, hence we will place each node
/// behind an "Rc".
pub struct TreeNode<T: Copy> {
    pub val: T,
    pub parent: RefCell<Weak<TreeNode<T>>>,
    pub children: RefCell<Vec<Rc<TreeNode<T>>>>,
}

impl<T: Copy> TreeNode<T> {
    pub fn new(t: T) -> Self {
        return TreeNode{
            val: t,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]) 
        };
    }

    pub fn get_val(&self) -> T {
        return self.val;
    }
}

pub fn set_child<T: Copy>(parent: &Rc<TreeNode<T>>, child: &Rc<TreeNode<T>>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_child() {
        let parent = Rc::new(TreeNode::new(2));
        let child = Rc::new(TreeNode::new(3));
        set_child(&parent, &child);

        assert_eq!(parent.children.borrow().len(), 1);
        assert_eq!(
            parent.children  // RefCell<Vec<Rc<TreeNode<i32>>>>
            .borrow()  // Ref<Vec<Rc<TreeNode<i32>>>>
            .get(0)  // Option<&Rc<TreeNode<i32>>>
            .unwrap()  // &Rc<TreeNode<i32>>
            .get_val(), // i32, this works because Rust compiler can
                        // repeatedly dereference on multiple levels
            child.get_val()
        );
        assert_eq!(
            child.parent  // RefCell<Weak<TreeNode<i32>>>
            .borrow()  // Ref<Weak<TreeNode<i32>>>
            .upgrade()  // Option<Rc<TreeNode<i32>>>
            .unwrap()  // Rc<TreeNode<i32>>
            .get_val(),  // <i32>
            parent.get_val()
        );
    }

}
