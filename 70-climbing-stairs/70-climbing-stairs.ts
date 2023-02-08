function climbStairs(n: number): number {
    if(n == 1) return 1;
    let [prev, curr] = [1, 2];
    for (let i = 2; i < n; i++) {
        // Cool ES6 feature
        [curr, prev] = [prev + curr, curr];
    }
    return curr;
};