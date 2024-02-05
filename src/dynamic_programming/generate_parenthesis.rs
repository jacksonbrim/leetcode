pub struct Solution;
impl Solution {
    // loop over list of strings starting at one open bracket and append open and closed bracket
    // combos until we've hit the correct number of pairs,
    // clone the string to append new closed brackets, and push the new string to the Vec<String>
    // append open brackets in place,
    // keep track of the number of open brackets at each location in the Vec<String>
    // only push valid brackets (i.e. cannot push a closed bracket if it will make an invalid
    // combo)
    // How?
    // ->  only push open brackets if the number of open brackets in that string is less than the
    // number of pairs
    // ->  only push closed brackets if the number of closed brackets in that string is less than the
    // number of open brackets in the string

    // initial intuition
    // beats 28ms, 5.38%, 2.35MB beating 36.15%
    pub fn generate_parenthesis_1(n: i32) -> Vec<String> {
        // track open parenthesis at each string
        let mut combos: Vec<String> = vec![String::new()];
        let mut opens: Vec<i32> = vec![0];

        for _i in 0..n * 2 {
            for j in 0..opens.len() {
                // check if we can add open brackets
                let left_possible = opens[j] < n;
                let num_closed_brackets = combos[j].len() as i32 - opens[j];
                let right_possible = num_closed_brackets < opens[j];
                if left_possible && right_possible {
                    // do right side
                    let mut right_combo = combos[j].clone();
                    right_combo.push(')');
                    opens.push(opens[j]);
                    combos.push(right_combo);
                    // do left side
                    combos[j].push('(');
                    opens[j] += 1;
                } else if left_possible {
                    combos[j].push('(');
                    opens[j] += 1;
                } else {
                    combos[j].push(')');
                }
            }
        }
        combos
    }
    // with backtracking
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Self::backtrack(&mut result, String::new(), 0, 0, n as usize);
        result
    }

    fn backtrack(result: &mut Vec<String>, current: String, open: usize, close: usize, max: usize) {
        if current.len() == max * 2 {
            result.push(current);
            return;
        }

        if open < max {
            Self::backtrack(result, current.clone() + "(", open + 1, close, max);
        }
        if close < open {
            Self::backtrack(result, current + ")", open, close + 1, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn three_parenthesis_success() {
        let n = 3;
        let output = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        let mut output: Vec<String> = output
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut res = Solution::generate_parenthesis(n);

        output.sort();
        res.sort();

        assert_eq!(res, output);
    }
    #[test]
    fn one_parenthesis_success() {
        let n = 1;
        let output = vec!["()"];
        let mut output: Vec<String> = output
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut res = Solution::generate_parenthesis(n);
        output.sort();
        res.sort();

        assert_eq!(res, output);
    }
}
