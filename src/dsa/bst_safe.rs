struct Tree<T> {
    root: Link<T>,
    len: usize,
}

struct Node<T> {
    elem: T,
    left_child: Link<T>,
    right_child: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Tree<T>
where
    T: Ord + std::fmt::Debug,
{
    fn new() -> Self {
        Tree { root: None, len: 0 }
    }

    fn insert(&mut self, val: T) {
        let mut current = &mut self.root;
        while let Some(n) = current {
            match val.cmp(&n.elem) {
                std::cmp::Ordering::Less => {
                    current = &mut current.as_mut().unwrap().left_child;
                }
                std::cmp::Ordering::Greater => {
                    current = &mut current.as_mut().unwrap().right_child;
                }
                std::cmp::Ordering::Equal => return,
            }
        }
        *current = Self::new_node(val);
        self.len += 1;
    }

    fn insert_recursive(&mut self, val: T) {
        if Self::insert_rec_helper(&mut self.root, val) {
            self.len += 1;
        };
    }

    fn insert_rec_helper(node: &mut Link<T>, val: T) -> bool {
        if let Some(n) = node {
            match val.cmp(&n.elem) {
                std::cmp::Ordering::Equal => false,
                std::cmp::Ordering::Less => Self::insert_rec_helper(&mut n.left_child, val),
                std::cmp::Ordering::Greater => Self::insert_rec_helper(&mut n.right_child, val),
            }
        } else {
            *node = Self::new_node(val);
            true
        }
    }

    fn search(&self, val: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if node.elem == *val {
                return true;
            }

            if *val > node.elem {
                current = &node.right_child
            } else {
                current = &node.left_child
            }
        }
        false
    }

    fn search_recursive(&self, val: &T) -> bool {
        Self::search_rec_helper(&self.root, val)
    }

    fn search_rec_helper(node: &Link<T>, val: &T) -> bool {
        if let Some(n) = node {
            match val.cmp(&n.elem) {
                std::cmp::Ordering::Less => Self::search_rec_helper(&n.left_child, val),
                std::cmp::Ordering::Greater => Self::search_rec_helper(&n.right_child, val),
                std::cmp::Ordering::Equal => true,
            }
        } else {
            false
        }
    }

    fn delete(&mut self, elem: &T) -> Option<T> {
        let mut current = &mut self.root;

        while let Some(n) = current {
            match elem.cmp(&n.elem) {
                std::cmp::Ordering::Less => {
                    current = &mut current.as_mut().unwrap().left_child;
                }
                std::cmp::Ordering::Greater => {
                    current = &mut current.as_mut().unwrap().right_child;
                }
                std::cmp::Ordering::Equal => break,
            }
        }

        let mut del_node = current.take()?;
        match (del_node.left_child.take(), del_node.right_child.take()) {
            (None, None) => {}
            (Some(n), None) | (None, Some(n)) => *current = Some(n),
            (Some(l), Some(r)) => {
                let mut right = Some(r);
                let suc = Self::find_successor(&mut right).map(|mut node| {
                    node.left_child = Some(l);
                    node.right_child = right;
                    node
                });
                *current = suc;
            }
        }

        self.len -= 1;
        Some(del_node.elem)
    }

    fn find_successor(p_node: &mut Link<T>) -> Link<T> {
        let mut curr = p_node;
        while let Some(n) = curr
            && n.left_child.is_some()
        {
            curr = &mut curr.as_mut().unwrap().left_child;
        }
        let mut suc = curr.take().unwrap();
        *curr = suc.right_child.take();

        Some(suc)
    }

    fn new_node(elem: T) -> Link<T> {
        Some(Box::new(Node {
            elem,
            left_child: None,
            right_child: None,
        }))
    }
}

impl<T> Drop for Tree<T> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // Recursively collects elements in sorted order — doubles as a BST-invariant check.
    fn inorder_vec<T: Clone>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            inorder_vec(&node.left_child, out);
            out.push(node.elem.clone());
            inorder_vec(&node.right_child, out);
        }
    }

    fn tree_inorder<T: Clone + Ord>(tree: &Tree<T>) -> Vec<T> {
        let mut out = Vec::new();
        inorder_vec(&tree.root, &mut out);
        out
    }

    #[test]
    fn new_tree_is_empty() {
        let t: Tree<i32> = Tree::new();
        assert!(t.root.is_none());
        assert_eq!(t.len, 0);
    }

    #[test]
    fn insert_increases_len() {
        let mut t = Tree::new();
        t.insert(5);
        t.insert(3);
        t.insert(8);
        assert_eq!(t.len, 3);
    }

    #[test]
    fn insert_duplicate_does_not_increase_len() {
        let mut t = Tree::new();
        t.insert(5);
        t.insert(5);
        t.insert(5);
        assert_eq!(t.len, 1);
    }

    #[test]
    fn insert_maintains_sorted_order() {
        let mut t = Tree::new();
        for v in [5, 3, 8, 1, 4, 7, 9, 2, 6] {
            t.insert(v);
        }
        assert_eq!(tree_inorder(&t), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn insert_recursive_matches_insert_behavior() {
        let mut t = Tree::new();
        for v in [5, 3, 8, 1, 4, 7, 9] {
            t.insert_recursive(v);
        }
        assert_eq!(t.len, 7);
        assert_eq!(tree_inorder(&t), vec![1, 3, 4, 5, 7, 8, 9]);
    }

    #[test]
    fn insert_recursive_duplicate_ignored() {
        let mut t = Tree::new();
        t.insert_recursive(10);
        t.insert_recursive(10);
        assert_eq!(t.len, 1);
    }

    #[test]
    fn search_on_empty_tree() {
        let t: Tree<i32> = Tree::new();
        assert!(!t.search(&42));
    }

    #[test]
    fn search_finds_existing_elements() {
        let mut t = Tree::new();
        for v in [5, 3, 8, 1, 4, 7, 9] {
            t.insert(v);
        }
        for v in [5, 3, 8, 1, 4, 7, 9] {
            assert!(t.search(&v), "expected to find {v}");
        }
    }

    #[test]
    fn search_does_not_find_missing_elements() {
        let mut t = Tree::new();
        for v in [5, 3, 8] {
            t.insert(v);
        }
        assert!(!t.search(&100));
        assert!(!t.search(&-1));
        assert!(!t.search(&6));
    }

    #[test]
    fn search_recursive_matches_search() {
        let mut t = Tree::new();
        for v in [5, 3, 8, 1, 4, 7, 9] {
            t.insert(v);
        }
        for v in [5, 3, 8, 1, 4, 7, 9] {
            assert!(t.search_recursive(&v));
        }
        assert!(!t.search_recursive(&100));
    }

    #[test]
    fn delete_from_empty_tree_returns_none() {
        let mut t: Tree<i32> = Tree::new();
        assert_eq!(t.delete(&5), None);
    }

    #[test]
    fn delete_nonexistent_element_returns_none() {
        let mut t = Tree::new();
        t.insert(5);
        t.insert(3);
        assert_eq!(t.delete(&100), None);
        assert_eq!(t.len, 2);
    }

    #[test]
    fn delete_leaf_node() {
        let mut t = Tree::new();
        for v in [5, 3, 8] {
            t.insert(v);
        }
        assert_eq!(t.delete(&3), Some(3));
        assert_eq!(t.len, 2);
        assert!(!t.search(&3));
        assert_eq!(tree_inorder(&t), vec![5, 8]);
    }

    #[test]
    fn delete_node_with_one_child_left() {
        let mut t = Tree::new();
        for v in [5, 3, 2] {
            t.insert(v);
        }
        assert_eq!(t.delete(&3), Some(3));
        assert_eq!(t.len, 2);
        assert_eq!(tree_inorder(&t), vec![2, 5]);
    }

    #[test]
    fn delete_node_with_one_child_right() {
        let mut t = Tree::new();
        for v in [5, 3, 4] {
            t.insert(v);
        }
        assert_eq!(t.delete(&3), Some(3));
        assert_eq!(t.len, 2);
        assert_eq!(tree_inorder(&t), vec![4, 5]);
    }

    #[test]
    fn delete_node_with_two_children() {
        let mut t = Tree::new();
        for v in [5, 3, 8, 2, 4, 7, 9] {
            t.insert(v);
        }
        assert_eq!(t.delete(&3), Some(3));
        assert_eq!(t.len, 6);
        assert_eq!(tree_inorder(&t), vec![2, 4, 5, 7, 8, 9]);
        assert!(!t.search(&3));
    }

    #[test]
    fn delete_root_with_two_children() {
        let mut t = Tree::new();
        for v in [5, 3, 8, 2, 4, 7, 9] {
            t.insert(v);
        }
        assert_eq!(t.delete(&5), Some(5));
        assert_eq!(t.len, 6);
        // successor should be the smallest node in the right subtree (7)
        assert_eq!(t.root.as_ref().unwrap().elem, 7);
        assert_eq!(tree_inorder(&t), vec![2, 3, 4, 7, 8, 9]);
    }

    #[test]
    fn delete_root_with_one_child() {
        let mut t = Tree::new();
        t.insert(5);
        t.insert(3);
        assert_eq!(t.delete(&5), Some(5));
        assert_eq!(t.len, 1);
        assert_eq!(t.root.as_ref().unwrap().elem, 3);
    }

    #[test]
    fn delete_only_node() {
        let mut t = Tree::new();
        t.insert(42);
        assert_eq!(t.delete(&42), Some(42));
        assert_eq!(t.len, 0);
        assert!(t.root.is_none());
    }

    #[test]
    fn delete_all_elements_sequentially() {
        let mut t = Tree::new();
        let vals = [5, 3, 8, 1, 4, 7, 9, 2, 6];
        for v in vals {
            t.insert(v);
        }
        for v in vals {
            assert_eq!(t.delete(&v), Some(v));
        }
        assert_eq!(t.len, 0);
        assert!(t.root.is_none());
    }

    #[test]
    fn insert_search_delete_with_strings() {
        let mut t: Tree<String> = Tree::new();
        for s in ["banana", "apple", "cherry"] {
            t.insert(s.to_string());
        }
        assert!(t.search(&"apple".to_string()));
        assert_eq!(t.delete(&"banana".to_string()), Some("banana".to_string()));
        assert!(!t.search(&"banana".to_string()));
        assert_eq!(t.len, 2);
    }

    #[test]
    fn successor_with_its_own_right_child_is_reattached_correctly() {
        let mut t = Tree::new();
        for v in [10, 5, 15, 3, 7, 12, 20, 6, 8] {
            t.insert(v);
        }
        // deletes 5, whose successor (6) has a right child... exercises the
        // find_successor promotion path, not just the trivial no-child case.
        assert_eq!(t.delete(&5), Some(5));
        assert_eq!(tree_inorder(&t), vec![3, 6, 7, 8, 10, 12, 15, 20]);
        assert_eq!(t.len, 8);
    }

    #[test]
    fn large_sequence_maintains_sorted_order() {
        let mut t = Tree::new();
        let vals = [50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45, 55, 65, 75, 90];
        for v in vals {
            t.insert(v);
        }
        let mut expected = vals.to_vec();
        expected.sort();
        assert_eq!(tree_inorder(&t), expected);
        assert_eq!(t.len, vals.len());
    }
}
