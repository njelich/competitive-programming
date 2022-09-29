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
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut fast, mut slow, mut i) = (head.clone(), head.clone(), 0);
        
        while fast.is_some() {
            if(i % 2 == 1) {
                slow = slow.unwrap().next;
            }
            fast = fast.unwrap().next;
            i+=1;
        }
        
        slow
    }
}