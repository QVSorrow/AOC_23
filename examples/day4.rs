use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2023::{execute, Type};

fn main() {
    execute(4, Type::Demo, parser::parse, |values| solve1(values));
    execute(4, Type::Task1, parser::parse, |values| solve1(values));
    execute(4, Type::Task2, parser::parse, |values| solve2(values));
}

fn solve1(input: &[Card]) -> u32 {
    input.iter()
        .map(|card| card.points())
        .sum()
}

fn solve2(input: &[Card]) -> u32 {
    let mut card_queue = VecDeque::with_capacity(input.len() * 2);
    for card in input.iter() {
        card_queue.push_back(card);
    }
    let mut total = 0_u32;
    while !card_queue.is_empty() {
        let card = card_queue.pop_front().unwrap();
        total += 1;
        let matching_count = card.matching_numbers();
        for id in (card.id + 1)..=(card.id + matching_count) {
            let card: &Card = input.get((id - 1) as usize).unwrap();
            card_queue.push_back(card);
        }
    }
    total
}

struct Card {
    id: u32,
    winning: HashSet<u32>,
    your: HashSet<u32>,
    matching: u32,
}

impl Card {
    fn new(id: u32, winning: HashSet<u32>, your: HashSet<u32>) -> Self {
        let matching = *&winning.intersection(&your).count() as u32;
        Self { id, winning, your, matching }
    }

    fn matching_numbers(&self) -> u32 {
        self.matching
    }

    fn points(&self) -> u32 {
        let power = self.winning.intersection(&self.your).count();
        if power == 0 {
            0
        } else {
            2_u32.pow((power - 1) as u32)
        }
    }
}

mod parser {
    use nom::{IResult, Parser};
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::opt;
    use nom::multi::{many0, many1};
    use nom::sequence::{delimited, separated_pair, terminated};

    use advent_of_code_2023::decimal;

    use crate::Card;

    pub(crate) fn parse(input: &str) -> IResult<&str, Vec<Card>> {
        many1(terminated(parse_card, opt(line_ending)))(input)
    }

    fn parse_card(input: &str) -> IResult<&str, Card> {
        let (input, card_id) = delimited(terminated(tag("Card"), many1(tag(" "))), decimal::<u32>, tag(":"))(input)?;
        let (input, (winning, your)) = separated_pair(parse_numbers, tag("|"), parse_numbers)(input)?;
        let card = Card::new(card_id, winning.into_iter().collect(), your.into_iter().collect());
        Ok((input, card))
    }

    fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
        many1(delimited(many0(tag(" ")), decimal::<u32>, many0(tag(" "))))(input)
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
    use crate::{solve1, solve2};
    use crate::parser::parse;

    #[test]
    fn check_demo() {
        let input = include_str!("day4/demo");
        let (_, data) = parse(input).unwrap();
        let result = solve1(&data);
        assert_eq!(result, 13);
    }

    #[test]
    fn check_demo2() {
        let input = include_str!("day4/demo2");
        let (_, data) = parse(input).unwrap();
        let result = solve2(&data);
        assert_eq!(result, 30);
    }
    #[test]
    fn check_task2() {
        let input = include_str!("day4/input_2");
        let (_, data) = parse(input).unwrap();
        let result = solve2(&data);
        assert_eq!(result, 5132675);
    }
}

