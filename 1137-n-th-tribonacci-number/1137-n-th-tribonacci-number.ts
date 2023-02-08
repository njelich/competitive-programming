function tribonacci(n: number): number {
    if(n == 0) return 0;
    if(n == 1 || n == 2) return 1;
    let [curr, prev, pprev] = [2, 1, 1];
    for (let i = 3; i < n; i++) {
        // Cool ES6 feature
        [curr, prev, pprev] = [prev + pprev + curr, curr, prev];
    }
    return curr;
};