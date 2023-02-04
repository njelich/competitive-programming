function search(nums: number[], target: number): number {
    let l = 0, r = nums.length - 1;
    console.log("start", target, l, r)
    while (l <= r) {
        const i = (l + r) / 2 >> 0;
        console.log(nums[i], i, l, r)
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