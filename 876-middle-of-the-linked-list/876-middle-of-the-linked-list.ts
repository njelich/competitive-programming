/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function middleNode(head: ListNode | null): ListNode | null {
    if (!head) return null;
    let fast = head;
    let slow = head;
    while (fast) {
        fast = fast.next;
        if (fast) {
            fast = fast.next;
            slow = slow.next;
        }
    }
    return slow;
};