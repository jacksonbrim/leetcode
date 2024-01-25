use colored::Colorize;
use std::fmt;

struct VecWrapper(Vec<i32>);

struct Highlight<'a> {
    vec_wrapper: &'a VecWrapper,
    highlight_start: Option<usize>,
    highlight_final: Option<usize>,
    highlight_index: Option<usize>,
}

impl<'a> Highlight<'_> {
    fn new(vec_wrapper: &'a VecWrapper, start: usize, fin: usize, index: usize) -> Highlight<'_> {
        Highlight {
            vec_wrapper,
            highlight_start: Some(start),
            highlight_final: Some(fin),
            highlight_index: Some(index),
        }
    }

    fn start(&mut self, start: usize) {
        self.highlight_start = Some(start);
    }
    fn fin(&mut self, fin: usize) {
        self.highlight_final = Some(fin);
    }
    fn index(&mut self, index: usize) {
        self.highlight_index = Some(index);
    }
}

impl<'a> fmt::Debug for Highlight<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for (count, v) in self.vec_wrapper.0.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            let mut what_to_write = || {
                if let Some(idx) = self.highlight_start {
                    if count == idx as usize {
                        return write!(f, "{}", format!("{}", v).green().bold());
                    }
                }
                if let Some(idx) = self.highlight_final {
                    if count == idx as usize {
                        return write!(f, "{}", format!("{}", v).red().bold());
                    }
                }
                if let Some(idx) = self.highlight_index {
                    if count == idx {
                        return write!(f, "{}", format!("{}", v).yellow().bold());
                    }
                }

                write!(f, "{}", v)
            };
            what_to_write()?;
        }
        write!(f, " ]")
    }
}

struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }
        let a = VecWrapper(height.clone());
        let mut start_line: (usize, i32) = (0, height[0]);
        let mut final_line: (usize, i32) = (1, height[1]);
        let mut area: i64 =
            std::cmp::min(start_line.1, final_line.1) as i64 * (final_line.0 - start_line.0) as i64;
        // iterate over heights, compare each item against starting line
        // update starting line if the new_area is greater than the old area
        for i in 2..height.len() {
            let mut highlight = Highlight::new(&a, start_line.0, final_line.0, i);
            if height.len() < 20 {
                println!("{:?}", highlight);
            }
            let new_line = (i, height[i]);
            let new_area = Self::calculate_area(start_line, new_line);
            if new_area > area || area == 0 {
                if height.len() < 20 {
                    println!("\t{:?}", "updating final_line");
                }

                final_line = new_line;
                highlight.fin(final_line.0);
                area = new_area;
                // check all starting lines against new final line
                // iterate over all heights leading up to the new final line
                // check and see if there is a more optimal area
                for j in 0..i {
                    let current_start_line = (j, height[j]);
                    let current_area = Self::calculate_area(current_start_line, final_line);
                    if current_area > area {
                        if height.len() < 20 {
                            println!("\t{:?}", "updating starting line");
                        }
                        area = current_area;
                        start_line = current_start_line;
                        final_line = new_line;
                        highlight.start(start_line.0);
                        highlight.fin(final_line.0);
                    }
                }
            }
        }

        area as i32
    }
    pub fn calculate_area(start_line: (usize, i32), end_line: (usize, i32)) -> i64 {
        std::cmp::min(start_line.1, end_line.1) as i64 * (end_line.0 - start_line.0) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use std::path::Path;
    #[test]
    fn middle_to_end_success() {
        let heights: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = 49;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }

    #[test]
    fn length_1_success() {
        let heights: Vec<i32> = vec![1, 1];
        let output = 1;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn middle_heights_greater_than_points_success() {
        let heights: Vec<i32> = vec![1, 2, 1];
        let output = 2;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn max_area_does_not_contain_max_heights_success() {
        // [1,2] -> [3,3]
        // length 2, heights 2, area = 4
        let heights: Vec<i32> = vec![1, 2, 4, 3];
        let output = 4;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn zeros_and_interspersed_heights_success() {
        let heights: Vec<i32> = vec![0, 14, 6, 2, 10, 9, 4, 1, 10, 3];
        let output = 70;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn scaled_down_list_success() {
        let heights: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1];
        let output = 12;
        let res = Solution::max_area(heights);
        assert_eq!(res, output);
    }

    #[test]
    fn massive_list_success() {
        let data = fs::read_to_string("./test_input/most_water.txt")
            .expect("Opening file test_input/most_water.txt");
        let heights: Vec<i32> = data
            .trim()
            .split_terminator(",")
            .map(|num_str| {
                num_str
                    .parse::<i32>()
                    .map_err(|e| format!("Failed to parse '{}': {}", num_str, e))
            })
            .collect::<Result<Vec<i32>, _>>()
            .expect("One or more items failed to parse");
        println!("big list length: {:?}", heights.len());
        let output = 50000000;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
}
