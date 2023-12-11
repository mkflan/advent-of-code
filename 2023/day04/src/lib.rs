use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    our_numbers: HashSet<u32>,
}

impl Card {
    fn winners(&self) -> u32 {
        self.winning_numbers.intersection(&self.our_numbers).count() as u32
    }

    fn card_score(self) -> u32 {
        self.winners()
            .checked_sub(1)
            .map(|p| 2u32.pow(p))
            .unwrap_or(0)
    }
}

fn card_side(side: &str) -> IResult<&str, HashSet<u32>> {
    space1
        .precedes(fold_many1(
            terminated(complete::u32, space1),
            HashSet::new,
            |mut acc, num| {
                acc.insert(num);
                acc
            },
        ))
        .parse(side)
}

fn card(card: &str) -> IResult<&str, Card> {
    take_until(":")
        .precedes(tag(":"))
        .precedes(separated_pair(card_side, tag("|"), card_side))
        .map(|(winning_numbers, our_numbers)| Card {
            winning_numbers,
            our_numbers,
        })
        .parse(card)
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card).parse(input)
}

fn parse_input(input: &str) -> Vec<Card> {
    cards(input).unwrap().1
}

pub fn solve_part1(input: &str) -> u32 {
    parse_input(input).into_iter().map(Card::card_score).sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let cards = parse_input(input);

    let card_to_count = (0..cards.len())
        .map(|card| (card, 1))
        .collect::<HashMap<usize, u32>>();

    cards
        .into_iter()
        .enumerate()
        .fold(card_to_count, |mut ctc, (card_id, card)| {
            let to_add = *ctc.get(&card_id).unwrap();
            let winners = card.winners();

            for next_card_id in card_id + 1..=card_id + winners as usize {
                ctc.entry(next_card_id).and_modify(|c| *c += to_add);
            }

            ctc
        })
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, solve_part1(sample_input));
        assert_eq!(30, solve_part2(sample_input));
    }
}
