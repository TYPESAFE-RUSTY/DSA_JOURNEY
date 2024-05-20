// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut next: Option<Box<ListNode>> = None;
        let mut current: Option<Box<ListNode>> = head;

        while let Some(mut node) = current {
            current = node.next;
            node.next = next;
            next = Some(node);
        }

        next
    }
}
