impl Solution {
    pub fn calc_digits(mut n: i32) -> i32 {
        let mut sum = 0;
        while n >= 10 {
            sum += (n % 10).pow(2);
            n /= 10;
        }
        sum += (n % 10).pow(2);
        sum
    }

    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Solution::calc_digits(slow); 
            fast = Solution::calc_digits(Solution::calc_digits(fast));
            if fast == 1 {
                return true
            } else if slow == fast {
                return false
            }
        }
    }
}