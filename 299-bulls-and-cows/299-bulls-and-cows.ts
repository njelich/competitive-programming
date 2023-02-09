function getHint(secret: string, guess: string): string {
    let bulls = 0, cows = 0;
    let digits = new Map();
    for (let i = 0; i < 10; i++) {
        digits.set(`s${i}`, 0);
        digits.set(`g${i}`, 0)
    }
    for (let i = 0; i < secret.length; i ++) {
        if(secret[i] == guess[i]) {
            bulls += 1;
        } else {
            digits.set(`s${secret[i]}`, digits.get(`s${secret[i]}`) + 1);
            digits.set(`g${guess[i]}`, digits.get(`g${guess[i]}`) + 1);
        }
        
    }
    for (let i = 0; i < 10; i++) {
        cows += Math.min(digits.get(`s${i}`), digits.get(`g${i}`));
    }
    return `${bulls}A${cows}B`;
};