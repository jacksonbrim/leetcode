use std::iter::repeat;

pub struct Solution;
impl Solution {
    // first attempt second highest perf, beats 100%, 0ms runtime, 2.10MB Memory.
    // Initially, I processed it in reverse and then reversed the string aftwerwards. It had the
    // exact same performance strangely.
    // My first try, too. I'm told that extend and repeat should have more overhead, but
    // not enough to affect leetcode apparently
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut res: String = String::with_capacity(8);
        let thousands = num / 1000;
        num = num % 1000;
        let hundreds = num / 100;
        num = num % 100;
        let tens = num / 10;
        num = num % 10;
        // thousands
        res.extend(repeat('M').take(thousands as usize));
        // hundreds
        match hundreds {
            0 => {}
            1..=3 => res.extend(repeat('C').take(hundreds as usize)),
            4 => res.extend(['C', 'D'].iter()),
            5 => res.push('D'),
            6..=8 => {
                res.push('D');
                res.extend(repeat('C').take(hundreds as usize - 5));
            }
            9 => res.extend(['C', 'M'].iter()),
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }

        // tens
        match tens {
            1..=3 => res.extend(repeat('X').take(tens as usize)),
            4 => res.extend(['X', 'L'].iter()),
            5 => res.push('L'),
            6..=8 => {
                res.push('L');
                res.extend(repeat('X').take(tens as usize - 5));
            }
            9 => res.extend(['X', 'C'].iter()),
            0 => {}
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }
        // ones
        match num {
            1..=3 => res.extend(repeat('I').take(num as usize)),
            4 => res.extend(['I', 'V'].iter()),
            5 => res.push('V'),
            6..=8 => {
                res.push('V');
                res.extend(repeat('I').take(num as usize - 5));
            }
            9 => res.extend(['I', 'X'].iter()),
            0 => {}
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }

        res
    }
    // Beats 100.00% with 0ms runtime, 2.00MB Memory beating 97.33%
    pub fn int_to_roman_perf1(num: i32) -> String {
        let mut num = num;
        let mut res: String = String::with_capacity(8);
        let thousands = num / 1000;
        num = num % 1000;
        let hundreds = num / 100;
        num = num % 100;
        let tens = num / 10;
        res.push_str(&"M".repeat(thousands as usize));
        num = num % 10;
        match hundreds {
            0 => {}
            1..=3 => res.push_str(&"C".repeat(hundreds as usize)),
            4 => res.push_str("CD"),
            5 => res.push('D'),
            6..=8 => {
                res.push('D');
                res.push_str(&"C".repeat(hundreds as usize - 5));
            }
            9 => res.push_str("CM"),
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }
        match tens {
            1..=3 => res.push_str(&"X".repeat(tens as usize)),
            4 => res.push_str("XL"),
            5 => res.push('L'),
            6..=8 => {
                res.push('L');
                res.push_str(&"X".repeat(tens as usize - 5));
            }
            9 => res.push_str("XC"),
            0 => {}
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }

        match num {
            1..=3 => res.push_str(&"I".repeat(num as usize)),
            4 => res.push_str("IV"),
            5 => res.push('V'),
            6..=8 => {
                res.push('V');
                res.push_str(&"I".repeat(num as usize - 5));
            }
            9 => res.push_str("IX"),
            0 => {}
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }

        res
    }

    // 3ms runtime && 2.16MB Memory
    // worst perf
    pub fn int_to_roman_perf2(num: i32) -> String {
        let mut num = num;
        let mut res: String = String::with_capacity(8);
        // Precomputed components
        let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];

        for _ in 0..(num / 1000) {
            res.push('M');
        }
        num %= 1000;
        res.push_str(hundreds[(num / 100) as usize]);
        num %= 100;
        res.push_str(tens[(num / 10) as usize]);
        num %= 10;
        res.push_str(ones[num as usize]);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hundred_success() {
        let num = 100;
        let output = "C";
        let res = Solution::int_to_roman(num);

        assert_eq!(res, output);
    }
    #[test]
    fn four_hundred_success() {
        let num = 400;
        let output = "CD";
        let res = Solution::int_to_roman(num);

        assert_eq!(res, output);
    }

    #[test]
    fn nine_hundred_success() {
        let num = 900;
        let output = "CM";
        let res = Solution::int_to_roman(num);

        assert_eq!(res, output);
    }
    #[test]
    fn thousands_and_hundreds_success() {
        let num = 1400;
        let output = "MCD";
        let res = Solution::int_to_roman(num);

        assert_eq!(res, output);
    }

    #[test]
    fn tens_and_ones_success() {
        let num = 58;
        let output = "LVIII";
        let res = Solution::int_to_roman(num);

        assert_eq!(res, output);
    }
    #[test]
    fn all_digits_nine_hundred_nine_tens_four_success() {
        let num = 1994;
        let output = "MCMXCIV";
        let res = Solution::int_to_roman(num);

        assert_eq!(res, output);
    }
    #[test]
    fn very_big() {
        let num = 99999;
        let output = "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMCMXCIX";
        let res = Solution::int_to_roman(num);

        assert_eq!(res, output);
    }
}
