use std::mem;

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
        }
    }
    fn push(&mut self, elem: i32) {
        let new_node = Node{
            elem,
            next: mem::replace(&mut self.head, None),
        };
        self.head = Some(Box::new(new_node));
    }
    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(n) => {
                self.head = n.next;
                Some(n.elem)
            }
        }
    }
}
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ll2_test() {

    }
}