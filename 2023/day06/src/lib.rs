use itertools::Itertools;
use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug)]
struct Race {
    duration: u64,
    record_distance: u64,
}

impl Race {
    fn new(duration: u64, record_distance: u64) -> Self {
        Self {
            duration,
            record_distance,
        }
    }

    fn ways_to_beat_record(&self) -> u64 {
        (1..self.duration)
            .filter_map(|hold_time| {
                let distance = hold_time * (self.duration - hold_time);
                (distance > self.record_distance).then_some(distance)
            })
            .count() as u64
    }
}

fn nums(input: &str) -> IResult<&str, Vec<u64>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)
}

fn races(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    races(input).unwrap().1
}

pub fn solve_part1(input: &str) -> u64 {
    let (durations, record_distances) = parse_input(input);

    std::iter::zip(durations, record_distances)
        .map(|(duration, record_distance)| {
            Race::new(duration, record_distance).ways_to_beat_record()
        })
        .product()
}

pub fn solve_part2(input: &str) -> u64 {
    let (duration, record_distance) = parse_input(input);
    let duration = duration.iter().join("").parse::<u64>().unwrap();
    let record_distance = record_distance.iter().join("").parse::<u64>().unwrap();

    let race = Race {
        duration,
        record_distance,
    };

    race.ways_to_beat_record()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(288, solve_part1(sample_input));
        assert_eq!(71503, solve_part2(sample_input));
    }
}
