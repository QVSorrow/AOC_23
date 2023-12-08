use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::str::FromStr;
use std::string::ToString;
use std::sync::Arc;
use std::time::{Instant, SystemTime};
use colored::Colorize;
use derive_new::new;

use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = (Moves, Maps);
type Input2 = Input;
type Output = u64;

const DAY: u8 = 8;

fn main() {
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task2, parser::parse, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    let (moves, maps) = input;
    let mut current = &Location::from(Location::START);
    let mut steps = 0_u64;
    let mut moves = moves.iter().cycle();
    while *current != Location::END {
        let m = *moves.next().unwrap();
        current = maps.get_move(current, m);
        steps += 1;
    }
    steps
}

fn solve2(input: &Input2) -> Output {
    let (moves, maps) = input;
    let locations = maps.keys()
        .par_bridge()
        .filter(|k| k.name.ends_with("A"))
        .map(|original_location| {
            let mut location = original_location;
            let mut moves = moves.iter().cycle();
            let mut distance: Option<u64> = None;
            for iteration in 1.. {
                let next_move = unsafe { moves.next().unwrap_unchecked() };
                location = maps.get_move(location, *next_move);
                if location.end_with_z {
                    let stop = distance.is_some();
                    distance = if let Some(d) = distance {
                        Some(iteration - d)
                    } else {
                        Some(iteration)
                    };
                    if stop { break }
                }
            }
            distance.unwrap()
        })
        .collect::<Vec<_>>();

    let mut queue = VecDeque::from(locations);
    while queue.len() > 1 {
        let a = queue.pop_front().unwrap();
        let b = queue.pop_front().unwrap();
        queue.push_back(lcm(a, b))
    }
    queue.pop_front().unwrap()
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

mod parser {
    use std::str::FromStr;

    use nom::{IResult, Parser};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, alphanumeric1, line_ending};
    use nom::combinator::{map, opt, value};
    use nom::multi::many1;
    use nom::sequence::{delimited, separated_pair, terminated};

    use crate::{Input, Input2, Location, Maps, Move};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        let (input, moves) = moves(input)?;
        let (input, _) = empty_line(input)?;
        let (input, maps) = map(many1(map_entry), |entries| {
            let mut maps = Maps::default();
            for (key, (left, right)) in entries {
                maps.put(Location::new(key.to_string()), (Location::new(left.to_string()), Location::new(right.to_string())));
            }
            maps
        })(input)?;
        Ok((input, (moves, maps)))
    }

    pub(crate) fn parse2(input: &str) -> IResult<&str, Input2> {
        parse(input)
    }


    fn moves(input: &str) -> IResult<&str, Vec<Move>> {
        let left = value(Move::Left, tag("L"));
        let right = value(Move::Right, tag("R"));
        terminated(many1(alt((left, right))), line_ending)(input)
    }

    fn map_entry(input: &str) -> IResult<&str, (&str, (&str, &str))> {
        terminated(separated_pair(alphanumeric1, tag(" = "), delimited(tag("("), separated_pair(alphanumeric1, tag(", "), alphanumeric1), tag(")"))), opt(line_ending))
            .parse(input)
    }

    fn empty_line(input: &str) -> IResult<&str, ()> {
        value((), opt(line_ending))(input)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn check_line() {}

        #[test]
        fn check_all() {}
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Location {
    name: String,
    end_with_z: bool,
}

impl Location {
    fn new(name: String) -> Self {
        let end_with_z = name.ends_with("Z");
        Self { name, end_with_z }
    }
}

impl PartialEq<&'_ str> for Location {
    fn eq(&self, other: &&str) -> bool {
        self.name.eq(other)
    }
}

impl From<&'_ str> for Location {
    fn from(value: &'_ str) -> Self {
        Location::new(value.to_string())
    }
}

impl Location {
    const START: &'static str = "AAA";
    const END: &'static str = "ZZZ";
}

type Moves = Vec<Move>;


#[derive(Debug, Default)]
struct Maps {
    inner: HashMap<Location, (Location, Location)>,
}

impl Maps {
    fn put(&mut self, key: Location, values: (Location, Location)) {
        self.inner.insert(key, values);
    }

    fn get(&self, key: &Location) -> &(Location, Location) {
        self.inner.get(key).unwrap()
    }

    fn get_move(&self, key: &Location, m: Move) -> &Location {
        let (left, right) = self.inner.get(key).unwrap();
        match m {
            Move::Left => left,
            Move::Right => right,
        }
    }

    fn keys(&self) -> impl Iterator<Item=&Location> {
        self.inner.keys()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::{solve1, solve2};

    #[test]
    fn demo_1() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve1(&output);
        assert_eq!(result, 2);
    }

    #[test]
    fn demo_2() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve1(&output);
        assert_eq!(result, 6);
    }

    #[test]
    fn demo_3() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve2(&output);
        assert_eq!(result, 6);
    }
}



