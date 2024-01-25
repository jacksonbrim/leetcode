use colored::Colorize;
use std::fmt;

struct VecWrapper(Vec<i32>);

struct Highlight<'a> {
    vec_wrapper: &'a VecWrapper,
    highlight_index: usize,
}

impl<'a> fmt::Debug for Highlight<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for (count, v) in self.vec_wrapper.0.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            if count == self.highlight_index {
                write!(f, "->[ {} ]<-", v)?;
            } else {
                write!(f, "{}", v)?;
            }
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
        let mut start_line: (i32, i32) = (-1, 0);
        let mut final_line: (i32, i32) = (-1, 0);
        let mut area: i32 = 0;
        // iterate over Vec<i32>, compare each item against starting line
        // update starting line if the sum of item's height and length diffs are > 0;
        // update final line if the sum of item's height and length diffs are > 0,
        // then update starting line to the old final if the new area is greater.
        for (length, height) in height.iter().enumerate() {
            let highlight = Highlight {
                vec_wrapper: &a,
                highlight_index: length,
            };
            print!(
                "{}",
                format!(
                    "\n\n
heights:     {:?},
start_line:  {:?},
final_line:  {:?},
area:        {:?},
current line:
line_loc:    {:?},
line_height: {:?}",
                    highlight, start_line, final_line, area, length, height
                )
                .bright_white()
                .bold()
                .on_black()
            );

            let start_length_diff = start_line.0 - (length as i32);
            let final_length_diff = (length as i32) - final_line.0;
            let height_diff = height - std::cmp::min(final_line.1, start_line.1);

            print!(
                "{}",
                format!(
                    "
start_length_diff: {:?},
final_length_diff: {:?},
height_diff:       {:?}\n\n",
                    start_length_diff, final_length_diff, height_diff
                )
                .bright_white()
                .bold()
                .on_black()
            );
            if start_line.0 == -1 {
                // set first starting line
                start_line = (length as i32, *height);
                area =
                    std::cmp::min(start_line.1, final_line.1) * (start_line.0 - final_line.0).abs();
                print!(
                    "{}",
                    format!(
                        "
    initializing start_line:
        start_line: {:?}
        final_line: {:?}
        area: {:?}

    ",
                        start_line, final_line, area
                    )
                    .bright_white()
                    .on_black()
                );
            } else if final_line.0 == -1 {
                // set first starting line
                final_line = (length as i32, *height);
                area =
                    std::cmp::min(start_line.1, final_line.1) * (start_line.0 - final_line.0).abs();
                print!(
                    "{}",
                    format!(
                        "
    initializing final_line:
        start_line: {:?}
        final_line: {:?}
        area: {:?}

    ",
                        start_line, final_line, area
                    )
                    .bright_white()
                    .on_black()
                    .bold()
                );
            } else if (height_diff + final_length_diff) > 0 {
                // update final line and check whether or not to update the starting line
                let new_final_line = (length as i32, *height);
                let new_area = std::cmp::min(new_final_line.1, start_line.1)
                    * (final_line.0 - new_final_line.0).abs();
                print!(
                    "{}",
                    format!(
                        "
    updating final line: 
        start_line: {:?}
        final_line: {:?}
        new_final_line: {:?}
        new_area: {:?}

    ",
                        start_line, final_line, new_final_line, new_area
                    )
                    .bright_white()
                    .on_black()
                    .bold()
                );

                if new_area > area && area != 0 {
                    start_line = final_line;
                    final_line = new_final_line;
                    area = new_area;
                    print!(
                        "{}",
                        format!(
                            "
        start_line updated to equal previous final line:
            start_line: {:?}
            final_line: {:?}
            area: {:?}

        ",
                            start_line, final_line, area
                        )
                        .bright_cyan()
                        .on_black()
                        .bold()
                    );
                } else {
                    print!(
                        "{}",
                        format!(
                            "
        only final line updated:
            start_line: {:?}
            old_final_line: {:?}
            old_area: {:?}",
                            start_line, final_line, area
                        )
                        .bright_cyan()
                        .on_black()
                        .bold()
                    );

                    final_line = new_final_line;
                    area = std::cmp::min(start_line.1, final_line.1)
                        * (start_line.0 - final_line.0).abs();
                }
                print!(
                    "{}",
                    format!(
                        "            
            final_line: {:?}
            area: {:?}

        ",
                        final_line, area
                    )
                    .bright_cyan()
                    .on_black()
                    .bold()
                );
            } else if (start_length_diff + height_diff) > 0 {
                start_line = (length as i32, *height);
                area =
                    std::cmp::min(start_line.1, final_line.1) * (start_line.0 - final_line.0).abs();
                print!(
                    "{}",
                    format!(
                        "
    updating start_line:
        start_line: {:?}
        final_line: {:?}
        area: {:?}",
                        start_line, final_line, area
                    )
                    .bright_green()
                    .bold()
                    .on_black()
                );
            }
            print!(
                "{}",
                format!(
                    "
    iter result:
        heights: {:?}
        start_line: {:?}
        final_line: {:?}
        area: {:?}\n\n",
                    highlight, start_line, final_line, area
                )
                .white()
                .bold()
                .on_black()
            );
        }

        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn middle_to_end_success() {
        let height: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = 49;
        let res = Solution::max_area(height);

        assert_eq!(res, output);
    }

    #[test]
    fn length_1_success() {
        let height: Vec<i32> = vec![1, 1];
        let output = 1;
        let res = Solution::max_area(height);

        assert_eq!(res, output);
    }
    #[test]
    fn middle_height_greater_than_points_success() {
        let height: Vec<i32> = vec![1, 2, 1];
        let output = 2;
        let res = Solution::max_area(height);

        assert_eq!(res, output);
    }
    #[test]
    fn max_area_does_not_contain_max_height_success() {
        // [1,2] -> [3,3]
        // length 2, height 2, area = 4
        let height: Vec<i32> = vec![1, 2, 4, 3];
        let output = 4;
        let res = Solution::max_area(height);

        assert_eq!(res, output);
    }
}
