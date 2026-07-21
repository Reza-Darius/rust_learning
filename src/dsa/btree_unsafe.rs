use std::ptr;

struct BTree<T> {
    root: *mut Node<T>,
}
struct Node<T> {
    elem: T,
    parent: *mut Node<T>,
    left_child: *mut Node<T>,
    right_child: *mut Node<T>,
}

impl<T: PartialOrd> BTree<T> {
    pub fn new() -> BTree<T> {
        BTree {
            root: ptr::null_mut(),
        }
    }
    pub fn insert(&mut self, elem: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                elem,
                parent: ptr::null_mut(),
                left_child: ptr::null_mut(),
                right_child: ptr::null_mut(),
            }));
            if self.root.is_null() {
                self.root = new_node;
            } else {
                let mut ptr = self.root;
                while !ptr.is_null() {
                    (*new_node).parent = ptr;
                    if (*new_node).elem < (*ptr).elem {
                        ptr = (*ptr).left_child;
                    } else {
                        ptr = (*ptr).right_child;
                    }
                }
                if (*new_node).elem < (*(*new_node).parent).elem {
                    (*(*new_node).parent).left_child = new_node
                } else {
                    (*(*new_node).parent).right_child = new_node
                }
            }
        }
    }
}
impl<T> Drop for BTree<T> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btree_test() {}
}
