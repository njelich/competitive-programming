function twoSum(nums: number[], target: number): number[] {
    const pair = new Map();
    for (let i = 0; i < nums.length; i++) {
        if(!pair.has(nums[i])) {
            pair.set(target - nums[i], i);
        } else {
            return [pair.get(nums[i]), i];
        }
    }
};