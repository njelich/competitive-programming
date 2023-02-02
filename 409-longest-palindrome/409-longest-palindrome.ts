function longestPalindrome(s: string): number {
    const letters = new Map();
    for (let c of s) {
        if(letters.has(c)) {
            letters.set(c, letters.get(c) + 1);
        } else {
            letters.set(c, 1);
        }
    }
    let total = 0;
    let odd = 0;
    function mapElements(value, key, map) {
        if (value % 2) {
            total += value - 1;
            odd = 1;
        } else {
            total += value;
        }
        
    }
    letters.forEach(mapElements);
    return total + odd;
};