function isSubsequence(s: string, t: string): boolean {
    if (s.length == 0) {
        return true
    }
    let i = s.length - 1, j = t.length - 1;
    while (i >= 0 && j >= 0) {
        if(s[i] == t[j]) {
            i -= 1;
        }
        j -= 1;
    }
    return i == -1
    
};