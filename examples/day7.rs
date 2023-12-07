use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

use crate::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

type Input = Vec<(Hand, Bid)>;
type Input2 = Input;
type Output = u32;

const DAY: u8 = 7;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseCardError;

impl TryFrom<char> for Card {
    type Error = ParseCardError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let card = match value {
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => return Err(ParseCardError),
        };
        Ok(card)
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card = match s {
            "2" => Self::_2,
            "3" => Self::_3,
            "4" => Self::_4,
            "5" => Self::_5,
            "6" => Self::_6,
            "7" => Self::_7,
            "8" => Self::_8,
            "9" => Self::_9,
            "T" => Self::T,
            "J" => Self::J,
            "Q" => Self::Q,
            "K" => Self::K,
            "A" => Self::A,
            _ => return Err(ParseCardError),
        };
        Ok(card)
    }
}


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {
        let mut all_cards = HashMap::with_capacity(5);
        for card in value {
            if let Some(v) = all_cards.get_mut(card) {
                *v += 1;
            } else {
                all_cards.insert(*card, 1u8);
            }
        }
        if all_cards.len() > 1 {
            let jokers = all_cards.remove(&Card::J).unwrap_or(0);
            let max = all_cards.iter().max_by(|(k1, v1), (k2, v2)| {
                let count = v1.cmp(v2);
                if count == Ordering::Equal {
                    k1.cmp(k2)
                } else {
                    count
                }
            }).unwrap();
            let max = *max.0;
            let card_count = all_cards.get_mut(&max).unwrap();
            *card_count += jokers;
        }
        match all_cards.len() {
            1 => FiveOfKind,
            2 => {
                let max_cards = *all_cards.values().max().unwrap();
                if max_cards == 4 {
                    FourOfKind
                } else {
                    FullHouse
                }
            }
            3 => {
                let pairs = all_cards.values().filter(|&&v| v == 2).count();
                if pairs > 0 {
                    TwoPair
                } else {
                    ThreeOfKind
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn new(data: &[Card]) -> Self {
        let cards: [Card; 5] = data.try_into().expect("Hand should contain 5 cards");
        let hand_type = (&cards).into();
        Self { hand_type, cards }
    }
}

type Bid = u32;

fn main() {
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task2, parser::parse, |values| solve1(values));
}

fn solve1(input: &Input) -> Output {
    let mut data: Vec<_> = input.iter().cloned().collect();
    data.sort_unstable_by(|(a, ..), (b, ..)| a.cmp(b));
    data.iter().enumerate()
        .map(|(index, (hand, bid))| ((index as Bid) + 1) * bid)
        .sum()
}

mod parser {
    use std::str::FromStr;

    use nom::{IResult, Parser};
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, one_of};
    use nom::combinator::{map_res, opt};
    use nom::multi::{count, many1};
    use nom::sequence::{separated_pair, terminated};

    use advent_of_code_2023::integer;

    use crate::{Bid, Card, Hand, Input, Input2, ParseCardError};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        many1(parse_line)(input)
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        parse(input)
    }

    fn parse_line(input: &str) -> IResult<&str, (Hand, Bid)> {
        terminated(separated_pair(parse_hand, tag(" "), integer::<Bid>), opt(line_ending))(input)
    }

    fn parse_hand(input: &str) -> IResult<&str, Hand> {
        map_res(
            count(one_of("23456789TJQKA"), 5),
            |s| {
                let cards = s.into_iter().map(|c: char| Card::try_from(c))
                    .collect::<Result<Vec<Card>, _>>()?;
                Ok::<Hand, ParseCardError>(Hand::new(&cards))
            })(input)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn check_line() {}

        #[test]
        fn check_all() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {}
}



