/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function isValidBST(root: TreeNode | null): boolean {
    // Doesn't require queue/dequeue
    if(!root) return false;
    let last: number = Number.MIN_SAFE_INTEGER;
    while (root) {
        if (root.left) {
            let curr = root.left;
            while (curr.right) {
                curr = curr.right;
            }
            curr.right = root;
            root = root.left;
            curr.right.left = null;
        } else {
            if(last < root.val) {
                last = root.val
            } else {
                return false;
            }
            if(!root.right) {
                break;
            }
            root = root.right;
        }
    }
    return true;
};