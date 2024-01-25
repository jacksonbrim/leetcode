struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }

        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        let s_len = s_chars.len();
        let p_len = p_chars.len();
        let mut dp = vec![vec![false; p_len + 1]; s_len + 1];

        dp[0][0] = true;

        for i in 1..=p_len {
            if p_chars[i - 1] == '*' {
                dp[0][i] = dp[0][i - 2];
            }
        }

        for i in 1..=s_len {
            for j in 1..=p_len {
                if p_chars[j - 1] == '.' || p_chars[j - 1] == s_chars[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p_chars[j - 1] == '*' {
                    dp[i][j] = dp[i][j - 2];
                    if p_chars[j - 2] == '.' || p_chars[j - 2] == s_chars[i - 1] {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                }
            }
        }

        dp[s_len][p_len]
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn match_one_letter_fail() {
        let s = "aa";
        let p = "a";
        let output = false;
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(res, output);
    }
    #[test]
    fn match_all_a_true() {
        let s = "aa";
        let p = "a*";
        let output = true;
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(res, output);
    }
    #[test]
    fn match_all_true() {
        let s = "ab";
        let p = ".*";
        let output = true;
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(res, output);
    }
    #[test]
    fn multi_pattern_true() {
        let s = "aab";
        let p = "c*a*b";
        let output = true;
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(res, output);
    }
    #[test]
    fn multi_pattern_false() {
        let s = "aaaaaaaaaaaaaaaaaaa";
        let p = "a*a*a*a*a*a*a*a*a*b";
        let output = false;
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(res, output);
    }
    #[test]
    fn mix_wildcard_match_and_single_char_match_success() {
        let s = "mississippi";
        let p = "mis*is*ip*.";
        let output = true;
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(res, output);
    }
}
