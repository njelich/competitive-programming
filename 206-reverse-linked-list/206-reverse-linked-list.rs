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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        
        // First we unwrap curr, using let Some(), we are storing curr this way
        while let Some(mut node) = curr {
            // We store the next node pointer in curr
            curr = node.next;
            // Then we repoint the node pointer to the previous one
            node.next = prev;
            // And we store the node for further us, completing the inversion
            prev = Some(node)
        }
        
        prev
    }
}