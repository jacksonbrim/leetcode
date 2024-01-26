use std::iter::repeat;

struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut res: String = String::with_capacity(8);
        let thousands = num / 1000;
        num = num % 1000;
        let hundreds = num / 100;
        num = num % 100;
        let tens = num / 10;
        num = num % 10;
        match num {
            1..=3 => res.extend(repeat('I').take(num as usize)),
            4 => res.extend(['V', 'I'].iter()),
            5 => res.push('V'),
            6..=8 => {
                res.extend(repeat('I').take(num as usize - 5));
                res.push('V');
            }
            9 => res.extend(['X', 'I'].iter()),
            0 => {}
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }
        match tens {
            1..=3 => res.extend(repeat('X').take(tens as usize)),
            4 => res.extend(['L', 'X'].iter()),
            5 => res.push('L'),
            6..=8 => {
                res.extend(repeat('X').take(tens as usize - 5));
                res.push('L');
            }
            9 => res.extend(['C', 'X'].iter()),
            0 => {}
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }
        match hundreds {
            0 => {}
            1..=3 => res.extend(repeat('C').take(hundreds as usize)),
            4 => res.extend(['D', 'C'].iter()),
            5 => res.push('D'),
            6..=8 => {
                res.extend(repeat('C').take(hundreds as usize - 5));
                res.push('D');
            }
            9 => res.extend(['M', 'C'].iter()),
            _ => {
                panic!(
                    "Hundreds has impossible range: hundreds {:?} not in 0..=9",
                    hundreds
                )
            }
        }
        res.extend(repeat('M').take(thousands as usize));

        res.chars().rev().collect::<String>()
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
