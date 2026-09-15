// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if n == 0 {
        return head;
    }
    let mut curr = &mut head;
    let mut len = 0;

    // get len of ll
    while curr.is_some() {
        curr = &mut curr.as_mut().unwrap().next;
        len += 1;
    }

    if n > len {
        return None;
    }

    if len == n {
        return head.unwrap().next
    }

    // walk to the node we have to reconnect
    curr = &mut head;
    for _ in 0..len - n {
        curr = &mut curr.as_mut().unwrap().next;
    }

    let nxt_node = &mut curr.as_mut().unwrap().next;
    curr.as_mut().unwrap().next = nxt_node.as_mut().unwrap().next.take();

    head
}
