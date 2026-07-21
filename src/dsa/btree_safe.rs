use std::cell::RefCell;
use std::ops::DerefMut;
use std::{
    ptr,
    rc::{Rc, Weak},
};
struct Tree<T> {
    root: Option<Box<Node<T>>>,
}
struct Node<T> {
    elem: T,
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
}

impl<T> Tree<T>
where
    T: Ord,
{
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, val: T) {
        let mut current = &mut self.root;
        if let Some(node) = current {
            if val > node.elem {
                current = &mut node.right_child
            } else if val < node.elem {
                current = &mut node.left_child
            } else {
                // values are equal
                return;
            }
        }
        let new = Box::new(Node {
            elem: val,
            left_child: None,
            right_child: None,
        });
        *current = Some(new);
    }

    fn search(&self, val: T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if node.elem == val {
                return true;
            } else if val > node.elem {
                current = &node.right_child
            } else {
                current = &node.left_child
            }
        }
        false
    }
}

impl<T> Drop for Tree<T> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btree_test() {}
}
