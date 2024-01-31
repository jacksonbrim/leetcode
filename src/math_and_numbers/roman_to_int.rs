struct Solution;
impl Solution {
    // Beats 100.00% of users with rust, 0ms runtime, 1.98MB Memor beating 96.79% of users w/ Rust
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        // peekable() allows you to get a reference to the next element without
        // advancing the iterator
        let mut s_chars = s.chars().peekable();
        // iterate over all characters
        while let Some(ch) = s_chars.next() {
            let next_ch = s_chars.peek().unwrap_or(&'_');
            match ch {
                'I' if next_ch == &'V' => {
                    s_chars.next();
                    res += 4;
                }
                'I' if next_ch == &'X' => {
                    s_chars.next();
                    res += 9;
                }
                'I' => res += 1,
                'V' => res += 5,
                'X' if next_ch == &'C' => {
                    s_chars.next();
                    res += 90;
                }
                'X' if next_ch == &'L' => {
                    s_chars.next();
                    res += 40;
                }
                'X' => res += 10,
                'L' => res += 50,
                'C' if next_ch == &'M' => {
                    s_chars.next();
                    res += 900;
                }
                'C' if next_ch == &'D' => {
                    s_chars.next();
                    res += 400;
                }
                'C' => res += 100,
                'D' => res += 500,
                'M' => res += 1000,
                _ => {
                    panic!("{:?} is not a Roman Numeral", ch);
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ones_success() {
        let numeral = "III";
        let output = 3;
        let res = Solution::roman_to_int(numeral.to_string());

        assert_eq!(res, output);
    }
    #[test]
    fn tens_and_ones_success() {
        let numeral = "LVIII";
        let output = 58;
        let res = Solution::roman_to_int(numeral.to_string());

        assert_eq!(res, output);
    }

    #[test]
    fn nine_hundred_success() {
        let numeral = "CM";
        let output = 900;
        let res = Solution::roman_to_int(numeral.to_string());

        assert_eq!(res, output);
    }
    #[test]
    fn thousands_and_hundreds_success() {
        let numeral = "MCD";
        let output = 1400;
        let res = Solution::roman_to_int(numeral.to_string());

        assert_eq!(res, output);
    }

    #[test]
    fn all_digits_nine_hundred_nine_tens_four_success() {
        let numeral = "MCMXCIV";
        let output = 1994;
        let res = Solution::roman_to_int(numeral.to_string());

        assert_eq!(res, output);
    }
    #[test]
    fn six_hundred_twenty_one_success() {
        let numeral = "DCXXI";
        let output = 621;
        let res = Solution::roman_to_int(numeral.to_string());

        assert_eq!(res, output);
    }

    #[test]
    fn very_big() {
        let numeral = "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMCMXCIX";
        let output = 99999;
        let res = Solution::roman_to_int(numeral.to_string());

        assert_eq!(res, output);
    }
}
