struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();
        if x_str == x_str.chars().rev().collect::<String>() {
            true
        } else {
            false
        }
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_positive_success() {
        let x = 121;
        let output = true;
        let res = Solution::is_palindrome(x);
        assert_eq!(res, output);
    }
    #[test]
    fn basic_negative_failure() {
        let x = -121;
        let output = false;
        let res = Solution::is_palindrome(x);
        assert_eq!(res, output);
    }
    #[test]
    fn trailing_zeroes_success() {
        let x = 10;
        let output = false;
        let res = Solution::is_palindrome(x);
        assert_eq!(res, output);
    }
}
