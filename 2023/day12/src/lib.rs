#![allow(unused)]

use nom::{
    bytes::complete::is_a,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::tag::complete::tag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringCondition {
    Unknown,
    Damaged,
    Operational,
}

impl SpringCondition {
    fn from_char(ch: char) -> Self {
        use SpringCondition::*;

        match ch {
            '?' => Unknown,
            '#' => Damaged,
            '.' => Operational,
            _ => panic!("Invalid spring condition"),
        }
    }
}

fn springs(spring: &str) -> IResult<&str, Vec<SpringCondition>> {
    is_a("?#.")
        .map(|s: &str| s.chars().map(SpringCondition::from_char).collect())
        .parse(spring)
}

fn record(record: &str) -> IResult<&str, (Vec<SpringCondition>, Vec<u32>)> {
    separated_pair(springs, space1, separated_list1(tag(","), complete::u32)).parse(record)
}

fn parse_records(records: &str) -> IResult<&str, Vec<(Vec<SpringCondition>, Vec<u32>)>> {
    separated_list1(line_ending, record).parse(records)
}

fn parse_input(input: &str) -> Vec<(Vec<SpringCondition>, Vec<u32>)> {
    parse_records(input).unwrap().1
}

pub fn solve_part1(input: &str) -> usize {
    let records = parse_input(input);

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
        let sample_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(21, solve_part1(sample_input));
        assert_eq!(0, solve_part2(sample_input));
    }
}
