#![feature(iter_array_chunks)]

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::prelude::*;
use std::ops::Range;

#[derive(Debug)]
struct Map {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn translate(&self, seed: u64) -> u64 {
        let Some((src_range, dest_range)) = self
            .mappings
            .iter()
            .find(|(src_range, _)| src_range.contains(&seed))
        else {
            return seed;
        };

        let offset = seed - src_range.start;

        dest_range.start + offset
    }
}

fn parse_seeds(seeds: &str) -> IResult<&str, Vec<u64>> {
    tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(seeds)
}

fn mapping(mapping: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (mapping, (dest_start, src_start, len)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))
    .parse(mapping)?;

    let src_range = src_start..src_start + len;
    let dest_range = dest_start..dest_start + len;

    Ok((mapping, (src_range, dest_range)))
}

fn map(map: &str) -> IResult<&str, Map> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(mapping)).map(|mappings| Map { mappings }))
        .parse(map)
}

fn parse_maps(maps: &str) -> IResult<&str, Vec<Map>> {
    many1(map).parse(maps)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let (_, seeds) = parse_seeds(input).unwrap();
    let (_, maps) = parse_maps(input).unwrap();

    (seeds, maps)
}

pub fn solve_part1(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);

    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
        .min()
        .unwrap()
}

pub fn solve_part2(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);
    let seeds = seeds
        .iter()
        .array_chunks::<2>()
        .flat_map(|[&seed_range_start, &seed_range_len]| {
            seed_range_start..seed_range_start + seed_range_len
        })
        .collect::<Vec<_>>();

    seeds
        .into_par_iter()
        .map(|seed| maps.iter().fold(seed, |seed, map| map.translate(seed)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(35, solve_part1(sample_input));
        assert_eq!(46, solve_part2(sample_input));
    }
}
