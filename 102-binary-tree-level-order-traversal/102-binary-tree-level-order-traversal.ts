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

function levelOrder(root: TreeNode | null): number[][] {
    // Doesn't require queue/dequeue
    if(!root) return [];
    let traversal: number[][]  = [[root.val]];
    let curr = [root?.left, root?.right].filter((n) => n);
    while (curr.length) {
        let next = curr.flatMap((node) => [node?.left, node?.right]);
        traversal.push(curr.map((node) => node.val));
        curr = next.filter((n) => n);
    }
    return traversal;
};