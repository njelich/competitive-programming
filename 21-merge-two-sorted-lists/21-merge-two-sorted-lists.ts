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

function mergeTwoLists(list1: ListNode | null, list2: ListNode | null): ListNode | null {
    if (list1 == null && list2 == null) {
        return null;
    } else if (list1 == null) {
        return list2
    } else if (list2 == null) {
        return list1
    } else {
        return list1.val > list2.val
            ? new ListNode(list2.val, mergeTwoLists(list1, list2.next))
            : new ListNode(list1.val, mergeTwoLists(list1.next, list2))
    }
};