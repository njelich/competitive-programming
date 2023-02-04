/**
 * The knows API is defined in the parent class Relation.
 * isBadVersion(version: number): boolean {
 *     ...
 * };
 */

var solution = function(isBadVersion: any) {

    return function(n: number): number {
        let l = 1, r = n;
        while (l < r) {
            const i = (l + r) / 2 >> 0;
            if (isBadVersion(i)) {
                r = i;
            } else {
                l = i + 1;
            }
        }
        return l;
    };
};