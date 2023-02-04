/**
 * Definition for node.
 * class Node {
 *     val: number
 *     children: Node[]
 *     constructor(val?: number) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.children = []
 *     }
 * }
 */

function preorder(root: Node | null): number[] {
    // Pretty recursion, no queue/dequeue
    if(!root) return [];
    let traversal = [root.val];
    let curr = root.children;
    for (const n of curr) {
        traversal.push(...preorder(n));
    }
    return traversal;
};  