function pivotIndex(nums: number[]): number {
    let sum = nums.reduce((acc, a) => acc + a, 0);
    let acc = 0;
    for (let i = 0; i < nums.length; i++) {
        if (sum - nums[i] == 2*acc) {
            return i;
        }
        acc += nums[i];
    }
    return -1;
};