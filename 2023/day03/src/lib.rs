#![allow(unused)]

use itertools::Itertools;

const SYMBOLS: &[char] = &['*', '%', '#', '@', '=', '#', '+', '-', '&', '/', '$'];

pub fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;
    let once = std::iter::once("");
    let line_chunks = once
        .clone()
        .chain(input.lines())
        .chain(once)
        .tuple_windows::<(_, _, _)>();

    for (prev, cur, next) in line_chunks {
        let mut cur_remainder = cur;
        let mut crchars = cur_remainder.chars();
        let mut consumed = 0;

        while let Some((num_start_idx, first_digit)) = crchars.find_position(|c| c.is_numeric()) {
            let num_start_idx_cur = num_start_idx + consumed;
            let next_start_idx;
            let next_start_idx_cur;

            let num = std::iter::once(first_digit)
                .chain(crchars.by_ref().take_while(|c| c.is_numeric()))
                .collect::<String>();
            next_start_idx = num_start_idx + num.len();
            next_start_idx_cur = next_start_idx + consumed;
            let num = num.parse::<u32>().unwrap();

            // Check the current line for adjacent symbols.
            let cur_bytes = cur.as_bytes();

            // Check on the same line.
            if num_start_idx_cur == 0 {
                if SYMBOLS.contains(&(cur_bytes[next_start_idx_cur] as char)) {
                    sum += num;
                }
            } else if next_start_idx_cur == cur.len() {
                if SYMBOLS.contains(&(cur_bytes[num_start_idx_cur - 1] as char)) {
                    sum += num;
                }
            } else {
                if SYMBOLS.contains(&(cur_bytes[num_start_idx_cur - 1] as char))
                    || SYMBOLS.contains(&(cur_bytes[next_start_idx_cur] as char))
                {
                    sum += num;
                }
            }

            // Check the previous line, if there is one, directly above and diagonally.
            if !prev.is_empty() {
                let prev = prev.as_bytes();

                let start_idx = if num_start_idx_cur == 0 {
                    num_start_idx_cur
                } else {
                    num_start_idx_cur - 1
                };

                let end_idx = if next_start_idx_cur == cur.len() {
                    next_start_idx_cur - 1
                } else {
                    next_start_idx_cur
                };

                for idx in start_idx..=end_idx {
                    if SYMBOLS.contains(&(prev[idx] as char)) {
                        sum += num;
                    }
                }
            }

            if !next.is_empty() {
                let next_bytes = next.as_bytes();

                let start_idx = if num_start_idx_cur == 0 {
                    num_start_idx_cur
                } else {
                    num_start_idx_cur - 1
                };

                let end_idx = if next_start_idx_cur == cur.len() {
                    next_start_idx_cur - 1
                } else {
                    next_start_idx_cur
                };

                for idx in start_idx..=end_idx {
                    if SYMBOLS.contains(&(next_bytes[idx] as char)) {
                        sum += num;
                    }
                }
            }

            consumed += next_start_idx;
            cur_remainder = &cur_remainder[next_start_idx..];
            crchars = cur_remainder.chars();
        }
    }

    sum
}

pub fn solve_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(4361, solve_part1(sample_input));
        assert_eq!(0, solve_part2(sample_input));
    }
}
