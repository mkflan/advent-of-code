#![allow(unused)]

use nom::{
    character::complete::{alpha1, alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::tag::complete::tag;
use std::{iter::Cycle, str::Chars};

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    label: &'a str,
    to: (&'a str, &'a str),
}

fn parse_directions(input: &str) -> IResult<&str, Cycle<Chars<'_>>> {
    alpha1.map(|s: &str| s.chars().cycle()).parse(input)
}

fn parse_dest_nodes(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(tag("("), tuple((alpha1, tag(", "), alpha1)), tag(")"))
        .map(|(left, _, right)| (left, right))
        .parse(input)
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    separated_list1(
        line_ending,
        tuple((alphanumeric1, tag(" = "), parse_dest_nodes))
            .map(|(label, _, to)| Node { label, to }),
    )
    .parse(input)
}

fn parse_input(input: &str) -> (Cycle<Chars<'_>>, Vec<Node>) {
    separated_pair(parse_directions, tag("\n\n"), parse_nodes)
        .parse(input)
        .unwrap()
        .1
}

pub fn solve_part1(input: &str) -> u32 {
    let (mut directions, nodes) = parse_input(input);

    let mut cur_node_label = "AAA";
    let mut steps = 0;

    while cur_node_label != "ZZZ" {
        let Some(next_direction) = directions.next() else {
            break;
        };

        let to_nodes = nodes
            .iter()
            .find(|node| node.label == cur_node_label)
            .map(|node| node.to)
            .unwrap();

        if next_direction == 'L' {
            cur_node_label = to_nodes.0;
        } else if next_direction == 'R' {
            cur_node_label = to_nodes.1;
        }

        steps += 1;
    }

    steps
}

pub fn solve_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, solve_part1(sample_input));
        assert_eq!(0, solve_part2(sample_input));
    }
}
