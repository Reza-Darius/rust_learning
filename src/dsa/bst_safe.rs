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
    T: Ord,
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
        Self::insert_rec_helper(&mut self.root, val);
    }

    fn insert_rec_helper(node: &mut Link<T>, val: T) {
        if let Some(n) = node {
            if val > n.elem {
                Self::insert_rec_helper(&mut n.right_child, val);
            } else if val < n.elem {
                Self::insert_rec_helper(&mut n.left_child, val);
            }
            // if the elements are equal we just return
        } else {
            *node = Self::new_node(val);
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
            if n.elem == *val {
                return true;
            }
            if *val > n.elem {
                return Self::search_rec_helper(&n.right_child, val);
            } else {
                return Self::search_rec_helper(&n.left_child, val);
            }
        }
        false
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
            (Some(l) , Some(r)) => {
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

    #[test]
    fn btree_test() {}
}
