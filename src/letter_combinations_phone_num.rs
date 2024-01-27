struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits_len = digits.len();
        let res: Vec<String> = Vec::new();
        let keypad_matches: Vec<String> = digits
            .chars()
            .map(|x| Self::match_digit_to_phone_keypad(x))
            .collect();
        for i in 0..keypad_matches.len() - 2 {}
        println!("matches: {:?}", keypad_matches);
        res
    }

    pub fn match_digit_to_phone_keypad(digit: char) -> String {
        let res = match digit {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jki",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "abc",
            _ => unreachable!(),
        };
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success1() {
        let input = "23".to_string();
        let output: Vec<&str> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        let res = Solution::letter_combinations(input);
        assert_eq!(res, output);
    }
}
