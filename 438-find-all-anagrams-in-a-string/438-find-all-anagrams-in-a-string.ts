function findAnagrams(s: string, p: string): number[] {
    const windowSize = p.length
    const chars = new Array(26);
    chars.fill(0);
    let ret = [];
    for (let i = 0; i < windowSize; i++) {
        chars[s.charCodeAt(i) - 97] += 1;
        chars[p.charCodeAt(i) - 97] -= 1;
    }
    for (let i = windowSize; i < s.length; i++) {
        console.log(chars);
        if (chars.every((val) => val == 0)) {
            ret.push(i - windowSize);
        }
        chars[s.charCodeAt(i) - 97] += 1;
        chars[s.charCodeAt(i - windowSize) - 97] -= 1;
    }
    console.log(chars);
    if (chars.every((val) => val == 0)) {
        ret.push(s.length - windowSize);
    }
    return ret;
};