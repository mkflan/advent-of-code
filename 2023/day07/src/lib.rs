#![allow(unused)]

use itertools::Itertools;
use nom::{
    bytes::complete::is_a,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(ch: char) -> Self {
        use Card::*;

        match ch {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("Invalid card."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        use HandType::*;

        let counts = self.cards.iter().counts();
        let mut counts = counts.values();

        if counts.len() == 1 {
            FiveOfAKind
        } else if counts.len() == 2 {
            if counts.any(|c| *c == 4) {
                FourOfAKind
            } else {
                FullHouse
            }
        } else if counts.len() == 3 {
            if counts.any(|c| *c == 3) {
                ThreeOfAKind
            } else {
                TwoPair
            }
        } else if counts.len() == 4 {
            OnePair
        } else {
            HighCard
        }
    }
}

fn cards(cards: &str) -> IResult<&str, Vec<Card>> {
    is_a("23456789TJQKA")
        .map(|s: &str| s.chars().map(Card::from_char).collect())
        .parse(cards)
}

fn hand(hand: &str) -> IResult<&str, Hand> {
    separated_pair(cards, space1, complete::u32)
        .map(|(cards, bid)| Hand { cards, bid })
        .parse(hand)
}

fn parse_hands(hands: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, hand).parse(hands)
}

fn parse_input(input: &str) -> Vec<Hand> {
    parse_hands(input).unwrap().1
}

pub fn solve_part1(input: &str) -> u32 {
    let hands = parse_input(input);

    hands
        .into_iter()
        .sorted_by_key(|h| (h.hand_type(), h.cards.clone()))
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1) as u32)
        .sum()
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
