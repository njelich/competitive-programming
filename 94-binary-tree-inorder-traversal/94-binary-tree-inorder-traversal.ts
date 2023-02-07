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

function inorderTraversal(root: TreeNode | null): number[] {
    // Doesn't require queue/dequeue
    if(!root) return [];
    let traversal: number[]  = [];
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
            traversal.push(root.val);
            if(!root.right) {
                break;
            }
            root = root.right;
        }
    }
    return traversal;
};