function search(nums: number[], target: number): number {
    let l = 0, r = nums.length - 1;
    while (l <= r) {
        const i = (l + r) / 2 >> 0;
        if (nums[i] == target) {
            return i;
        } else if (nums[i] > target) {
            r = i - 1;
        } else {
            l = i + 1;
        }
    }
    return -1;
};