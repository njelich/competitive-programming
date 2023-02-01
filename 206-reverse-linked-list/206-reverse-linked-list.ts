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

function reverseList(head: ListNode | null): ListNode | null {
    if (!head) return null;
    let prev = head;
    let next = head.next;
    head.next = null;
    while (next) {
        const buff = next.next;
        next.next = prev;
        prev = next;
        next = buff;   
    }
    
    return prev;
};