use std::collections::btree_map::Keys;
struct Solution;
impl Solution {
    // 100%, 0ms, 2.17MB, 52.79%
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();
        Self::dfs(
            &mut result,
            &Self::digit_to_chars(),
            &digits,
            0,
            String::new(),
        );
        result
    }

    // depth first search
    fn dfs(
        result: &mut Vec<String>,
        keypad: &Vec<&str>,
        digits: &str,
        index: usize,
        current: String,
    ) {
        if index == digits.len() {
            result.push(current);
            return;
        }

        let digit = digits.chars().nth(index).unwrap();
        let letters = keypad[digit as usize - '0' as usize];

        for letter in letters.chars() {
            let mut new_current = current.clone();
            new_current.push(letter);
            Self::dfs(result, keypad, digits, index + 1, new_current);
        }
    }

    fn digit_to_chars() -> Vec<&'static str> {
        vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success1() {
        let input = "23".to_string();
        let output_strs: Vec<&str> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        let output: Vec<String> = output_strs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let res = Solution::letter_combinations(input);
        assert_eq!(res, output);
    }
    fn three_digits_success() {
        let input = "234".to_string();
        let output_strs = vec![
            "adg", "adg", "adh", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdg", "bdh",
            "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdg", "cdh", "ceg", "ceh", "cei",
            "cfg", "cfh", "cfi",
        ];
        let output: Vec<String> = output_strs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let res = Solution::letter_combinations(input);
        assert_eq!(res, output);
    }
}
