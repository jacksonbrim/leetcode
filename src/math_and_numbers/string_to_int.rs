struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut intermediate = String::new();
        let mut last_char: Option<char> = None;
        let s_str = s.as_str().trim_start();

        // handle numbers outside of i32
        fn check_i32(s: String) -> i32 {
            match s.parse::<i32>() {
                Ok(num) => num,
                Err(e) => {
                    match e.kind() {
                        std::num::IntErrorKind::PosOverflow => i32::MAX,
                        std::num::IntErrorKind::NegOverflow => i32::MIN,
                        _ => 0, // Handle other errors, or you might want to panic or log the error
                    }
                }
            }
        }

        for ch in s_str.chars() {
            // add digit
            if ch.is_digit(10) {
                // add preceding character if it exists
                if last_char.is_some() {
                    intermediate.push(last_char.take().unwrap());
                }
                intermediate.push(ch);
            } else if !intermediate.is_empty() {
                // not a digit and intermediate isn't empty means
                // we've already read the end of the number
                // return result
                return check_i32(intermediate);
            } else if last_char.is_none() && intermediate.is_empty() {
                // Only valid preceding characters are '-' and '+'.
                // Whitespace was removed earlier.
                // Anything other than those two trigger a return 0;
                match ch {
                    '-' => last_char = Some(ch),
                    '+' => last_char = Some(ch),
                    _ => return 0,
                }
            } else {
                // ch is not a digit, imtermediate is empty, and the leading char has been captured
                // by the last_char variable already. This means that this string fails
                // return 0;
                return 0;
            }
        }
        if intermediate.is_empty() {
            return 0;
        }
        return check_i32(intermediate);
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_positive_success() {
        let s = "42".to_string();
        let output = 42;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
    #[test]
    fn negative_leading_whitespace_success() {
        let s = "    -42".to_string();
        let output = -42;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
    #[test]
    fn ending_with_words_success() {
        let s = "4193 with words".to_string();
        let output = 4193;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
    #[test]
    fn starting_with_words_handle_fail() {
        let s = "words and 987".to_string();
        let output = 0;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
    #[test]
    fn leading_chars_handle_fail() {
        let s = "+-12".to_string();
        let output = 0;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
    #[test]
    fn only_non_digit_handle_fail() {
        let s = "+".to_string();
        let output = 0;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
    #[test]
    fn int_too_large() {
        let s = "20000000000000000000".to_string();
        let output = i32::MAX;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
    #[test]
    fn int_too_small() {
        let s = "-20000000000000000000".to_string();
        let output = i32::MIN;
        let res = Solution::my_atoi(s);
        assert_eq!(res, output);
    }
}
