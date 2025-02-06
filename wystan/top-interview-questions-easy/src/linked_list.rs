// #[derive(PartialEq, Eq, Clone, Debug)]
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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let (new_head, _) = _remove_nth_from_end(head, n);
    new_head
}

fn _remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
    match head {
        None => (None, 0),
        Some(mut node) => {
            let (next, pos) = _remove_nth_from_end(node.next.take(), n);
            node.next = next;
            let pos = pos + 1;
            // If the current node is the nth node from the end, skip it.
            if pos == n {
                (node.next, pos)
            } else {
                (Some(node), pos)
            }
        }
    }
}
