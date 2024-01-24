struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_neg: bool = x < 0;
        let int_string = if is_neg {
            (x as i64 * -1).to_string()
        } else {
            x.to_string()
        };
        let reversed_int_string = int_string.chars().rev().collect::<String>();
        let parsed_int: i64 = reversed_int_string.parse::<i64>().unwrap();
        if parsed_int > i32::MAX.into() || parsed_int < i32::MIN.into() {
            dbg!(parsed_int);
            return 0;
        }
        let parsed_i32 = parsed_int as i32;
        if is_neg {
            parsed_i32 * -1
        } else {
            parsed_i32
        }
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_positive_success() {
        let x = 123;
        let output = 321;
        let res = Solution::reverse(x);
        assert_eq!(res, output);
    }
    #[test]
    fn basic_negative_success() {
        let x = -123;
        let output = -321;
        let res = Solution::reverse(x);
        assert_eq!(res, output);
    }
    #[test]
    fn trailing_zeroes_success() {
        let x = 120;
        let output = 21;
        let res = Solution::reverse(x);
        assert_eq!(res, output);
    }
    #[test]
    fn oversized_i32_success() {
        let x = 1534236469;
        let output = 0;
        let res = Solution::reverse(x);
        assert_eq!(res, output);
    }
    #[test]
    fn negative_oversized_i32_success() {
        let x = -2147483648;
        let output = 0;
        let res = Solution::reverse(x);
        assert_eq!(res, output);
    }
}
