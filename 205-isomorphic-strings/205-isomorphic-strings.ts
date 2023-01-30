function isIsomorphic(s: string, t: string): boolean {
    const charMap = new Map();
    for (let i = 0; i < s.length; i++) {
        if(charMap.has(s[i]) && charMap.get(s[i]) != t[i]) {
            return false;
        }
        charMap.set(s[i], t[i])
    }
    return new Set(charMap.keys()).size == new Set(charMap.values()).size
};