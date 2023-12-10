#![allow(unused)]

pub fn solve_part1(input: &str) -> usize {
    0
}

pub fn solve_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(6440, solve_part1(sample_input));
        assert_eq!(0, solve_part2(sample_input));
    }
}
