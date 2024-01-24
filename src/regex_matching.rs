struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        let s = s.as_str();
        let p = p.as_str();

        let first_match = !s.is_empty()
            && (p.chars().next().unwrap() == s.chars().next().unwrap()
                || p.chars().next().unwrap() == '.');

        if p.len() >= 2 && p.chars().nth(1) == Some('*') {
            Self::is_match(s.to_string(), p[2..].to_string())
                || (first_match && Self::is_match(s[1..].to_string(), p.to_string()))
        } else {
            first_match && Self::is_match(s[1..].to_string(), p[1..].to_string())
        }
    }
}

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
}
