use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

enum Rule {
    Basic,
    Advanced,
}

impl Rule {
    fn card(&self, card: char) -> Card {
        match card {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => matches!(self, Self::Basic).then_some(Card::Jack).unwrap_or(Card::Joker),
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }

    fn card_combinaison(&self, cards: &[Card]) -> CardCombinaison {
        let (jokers, cards) = match self {
            Self::Basic => (Vec::default(), cards.to_vec()),
            Self::Advanced => cards.iter().partition(|card| matches!(card, Card::Joker)),
        };

        let cards = {
            let mut grouped_cards = HashMap::<Card, usize>::default();

            for card in cards.iter() {
                grouped_cards.entry(*card).and_modify(|v| *v += 1).or_insert(1);
            }

            grouped_cards
                .into_values()
                .sorted()
                .rev()
                .collect::<Vec<_>>()
        };

        let card_combinaison = match cards.as_slice() {
            [n, ..] if *n == 5 => CardCombinaison::FiveCards,
            [n, ..] if *n == 4 => CardCombinaison::FourCards,
            [n, m, ..] if *n == 3 && *m == 2 => CardCombinaison::FullHouse,
            [n, ..] if *n == 3 => CardCombinaison::ThreeCards,
            [n, m, ..] if *n == 2 && *m == 2 => CardCombinaison::TwoPairs,
            [n, ..] if *n == 2 => CardCombinaison::OnePair,
            _ => CardCombinaison::HighCard,
        };

        card_combinaison.upgrade_by(jokers.len())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Card {
    Joker = 0,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum CardCombinaison {
    HighCard = 0,
    OnePair,
    TwoPairs,
    ThreeCards,
    FullHouse,
    FourCards,
    FiveCards,
}

impl CardCombinaison {
    fn upgrade_by(&self, n: usize) -> Self {
        match (self, n) {
            (Self::HighCard, 1) => Self::OnePair,
            (Self::HighCard, 2) => Self::ThreeCards,
            (Self::HighCard, 3) => Self::FourCards,
            (Self::HighCard, n) if n > 3 => Self::FiveCards,
            (Self::OnePair, 1) => Self::ThreeCards,
            (Self::OnePair, 2) => Self::FourCards,
            (Self::OnePair, n) if n > 2 => Self::FiveCards,
            (Self::TwoPairs, 1) => Self::FullHouse,
            (Self::ThreeCards, 1) => Self::FourCards,
            (Self::ThreeCards, n) if n > 1 => Self::FiveCards,
            (Self::FourCards, n) if n > 0 => Self::FiveCards,
            _ => *self,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    card_combinaison: CardCombinaison,
    bid: usize,
}

fn get_input() -> &'static str {
    include_str!("../../data/day07.txt")
}

fn parse_input(input: &str, rule: Rule) -> Vec<Hand> {
    input
        .lines()
        .map(|s| {
            let s = s.split_whitespace().take(2).collect::<Vec<_>>();

            let cards = s[0].chars().map(|c| rule.card(c)).collect::<Vec<_>>();
            let card_combinaison = rule.card_combinaison(&cards);

            Hand {
                cards,
                card_combinaison,
                bid: s[1].parse().unwrap(),
            }
        })
        .collect()
}

fn get_solution(hands: Vec<Hand>) -> usize {
    hands
        .iter()
        .sorted_by(|a, b| match Ord::cmp(&a.card_combinaison, &b.card_combinaison) {
            Ordering::Equal => Ord::cmp(&a.cards, &b.cards),
            ordering => ordering,
        })
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

pub fn solve_part1() {
    let input = get_input();
    let hands = parse_input(input, Rule::Basic);
    let solution = get_solution(hands);

    println!("{solution}");
}

pub fn solve_part2() {
    let input = get_input();
    let hands = parse_input(input, Rule::Advanced);
    let solution = get_solution(hands);

    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn it_solves_example_part1() {
        let hands = parse_input(INPUT, Rule::Basic);
        let solution = get_solution(hands);

        assert_eq!(6440, solution);
    }

    #[test]
    fn it_solves_example_part2() {
        let hands = parse_input(INPUT, Rule::Advanced);
        let solution = get_solution(hands);

        assert_eq!(5905, solution);
    }
}
