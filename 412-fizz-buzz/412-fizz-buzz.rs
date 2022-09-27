impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).map(|i| {
            let mut result = "".to_string();
            if i % 3  == 0 { result.push_str("Fizz") }
            if i % 5  == 0 { result.push_str("Buzz") }
            if result == "" { i.to_string() } else { result }
        }).collect()
    }
}