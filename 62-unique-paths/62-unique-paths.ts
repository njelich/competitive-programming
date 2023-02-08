function uniquePaths(m: number, n: number): number {
    let grid = [new Array(n).fill(1)];
    for (let i = 1; i < m; i++) {
        grid.push([1]);
        for (let j = 1; j < n; j++) {
            grid[i].push(grid[i][j - 1] + grid[i - 1][j])
        }
    }
    return grid[m-1][n-1];
};