impl Solution {
    pub fn backspace_compare(mut s: String, mut t: String) -> bool {
        let mut ib = 0;
        let mut jb = 0;

        let mut buff = '0';

        loop {  
            if(buff == '0') {
                match s.pop() {
                    Some('#') => {
                        ib+=1;
                        while (ib > 0) {
                            if (buff != '0') {
                                buff = '0';
                                ib -= 1;
                                continue;
                            }
                            if Some('#') == s.pop() {
                                ib += 1;
                            } else {
                                ib -= 1;
                            }
                        }
                        continue;
                    },
                    Some(c) => {
                        buff = c;
                    },
                    None => {
                        while let Some(c) = t.pop() {
                            if c == '#' {
                                jb += 1;
                            } else {
                                jb -= 1;
                            }
                        }
                        if jb < 0 {
                            return false;
                        }
                        break;
                    }
                }
            }

            match t.pop() {
                Some('#') => {
                    jb+=1;
                },
                Some(c) => {
                    if ( c != buff) {
                        return false;
                    }
                    buff = '0';
                },
                None => {
                    if (buff != '0') {
                        return false;
                    }
                    while let Some(c) = s.pop() {
                        if c == '#' {
                            ib += 1;
                        } else {
                            ib -= 1;
                        }
                    }
                    if ib < 0 {
                        return false;
                    }
                    break;
                }
            }

            while (ib > 0) {
                if (buff != '0') {
                    buff = '0';
                    ib -= 1;
                    continue;
                }
                if Some('#') == s.pop() {
                    ib += 1;
                } else {
                    ib -= 1;
                }
            }

            while (jb > 0) {
                if Some('#') == t.pop() {
                    jb += 1;
                } else {
                    jb -= 1;
                }
            }
        }

        return true;
    }
}