use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult, Parser,
};

fn parse_histories(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, separated_list1(space1, complete::i64)).parse(input)
}

fn extrapolate(history: Vec<i64>, front: bool) -> i64 {
    if history.iter().all(|n| *n == 0) {
        return 0;
    }

    let deltas = history
        .iter()
        .tuple_windows()
        .map(|(l, r)| r - l)
        .collect::<Vec<_>>();
    let diff = extrapolate(deltas, front);

    front
        .then(|| history.first().unwrap() - diff)
        .unwrap_or_else(|| history.last().unwrap() + diff)
}

pub fn solve_part1(input: &str) -> i64 {
    let (_, histories) = parse_histories(input).unwrap();

    histories
        .into_iter()
        .map(|history| extrapolate(history, false))
        .sum()
}

pub fn solve_part2(input: &str) -> i64 {
    let (_, histories) = parse_histories(input).unwrap();

    histories
        .into_iter()
        .map(|history| extrapolate(history, true))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(68, solve_part1(sample_input));
        assert_eq!(2, solve_part2(sample_input));
    }
}
