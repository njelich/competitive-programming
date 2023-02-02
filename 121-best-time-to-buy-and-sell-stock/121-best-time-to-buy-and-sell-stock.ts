function maxProfit(prices: number[]): number {
    let best = 0;
    let low = prices[prices.length - 1];
    let high = prices[prices.length - 1];
    // Indexing backwards makes it easier
    for (let i = prices.length - 1; i >= 0; i--) {
        if (prices[i] > high) {
            low = prices[i];
            high = prices[i];
        } else if (prices[i] < low) { 
            low = prices[i];
        }
        best = Math.max(best, high - low);
    }
    return best;
};