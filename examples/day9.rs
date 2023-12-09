use std::str::FromStr;

use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = Vec<Vec<i64>>;
type Input2 = Input;
type Output = i64;

const DAY: u8 = 9;

fn main() {
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Demo, parser::parse2, |values| solve2(values));
    execute(DAY, Type::Task2, parser::parse2, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    let groups = input.iter()
        .map(|seq| SequenceGroup::new(seq.clone()))
        .collect::<Vec<_>>();

    groups.iter().map(|group| group.extrapolate_next()).sum()
}

fn solve2(input: &Input2) -> Output {
    let groups = input.iter()
        .map(|seq| SequenceGroup::new(seq.clone()))
        .collect::<Vec<_>>();

    groups.iter().map(|group| group.extrapolate_prev()).sum()
}


#[derive(Debug)]
struct SequenceGroup {
    values: Vec<Vec<i64>>,
}

impl SequenceGroup {
    fn new(root: Vec<i64>) -> Self {
        let mut values = Vec::new();
        values.push(root);
        while !values.last().unwrap().iter().all(|&v| v == 0) {
            let last = values.last().unwrap();
            let mut next = Vec::with_capacity(last.len() - 1);
            for arr in last.windows(2) {
                let a = *arr.get(0).unwrap();
                let b = *arr.get(1).unwrap();
                next.push(b - a);
            }
            values.push(next);
        }
        Self { values }
    }

    fn extrapolate_next(&self) -> i64 {
        let mut next = 0_i64;
        for seq in self.values.iter().rev().skip(1) {
            next = seq.last().unwrap() + next;
        }
        next
    }

    fn extrapolate_prev(&self) -> i64 {
        let mut prev = 0_i64;
        for seq in self.values.iter().rev().skip(1) {
            prev = seq.first().unwrap() - prev;
        }
        prev
    }
}

mod parser {
    use std::str::FromStr;

    use nom::{IResult, Parser};
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, line_ending};
    use nom::combinator::opt;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{pair, terminated};
    use advent_of_code_2023::integer;

    use crate::{Input, Input2};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        many1(parse_line)(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
        terminated(
            separated_list1(
                tag(" "),
                integer::<i64>,
            ),
            opt(line_ending),
        )(input)
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        parse(input)
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
    use crate::parser::parse;
    use crate::{solve1, solve2};

    #[test]
    fn check() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let (remains, sequences) = parse(input).unwrap();
        assert_eq!(remains, "");
        let result = solve1(&sequences);
        assert_eq!(result, 114);
    }
    #[test]
    fn check_2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let (remains, sequences) = parse(input).unwrap();
        assert_eq!(remains, "");
        let result = solve2(&sequences);
        assert_eq!(result, 2);
    }
}



