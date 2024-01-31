use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        let mut last_idx = -1;
        let mut substrings: HashMap<char, i32> = HashMap::with_capacity(128);
        for (idx, ch) in s.char_indices() {
            if let Some(p2) = substrings.insert(ch, idx as i32) {
                last_idx = std::cmp::max(p2, last_idx);
            }
            max_length = std::cmp::max(max_length, (idx as i32) - last_idx);
        }
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn success1() {
        // explan1 "The answer is "abc", with the length of 3.";
        let input_str1 = "abcabcbb";
        let output1 = 3;
        let res1 = Solution::longest_substring(input_str1.to_string());
        assert_eq!(res1, output1);
    }
    #[test]
    fn success2() {
        // explan2  "The answer is "b", with the length of 1.";
        let input_str2 = "bbbbb";
        let output2 = 1;
        let res2 = Solution::longest_substring(input_str2.to_string());
        assert_eq!(res2, output2);
    }
    #[test]
    fn success3() {
        // explan3 = "The answer is "wke", with the length of 1.";
        let input_str3 = "pwwkew";
        let output3 = 3;
        let res3 = Solution::longest_substring(input_str3.to_string());
        assert_eq!(res3, output3);
    }
    #[test]
    fn success4() {
        // explan4 = "The answer is "ab", with the length of 2.";
        let input_str4 = "aab";
        let output4 = 2;
        let res4 = Solution::longest_substring(input_str4.to_string());
        assert_eq!(res4, output4);
    }
    #[test]
    fn success5() {
        // explan5 = "The answer is "ab", with the length of 2.";
        let input_str5 = "dvdf";
        let output5 = 3;
        let res5 = Solution::longest_substring(input_str5.to_string());
        assert_eq!(res5, output5);
    }
}
