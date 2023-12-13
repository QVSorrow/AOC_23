use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter::Product;
use std::ops::{Div, Mul, Sub};
use std::str::FromStr;

use derive_new::new;
use num::{PrimInt, Unsigned};
use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = Vec<Vec<Cell>>;
type Input2 = Input;
type Output = u64;

const DAY: u8 = 11;

fn main() {
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task2, parser::parse2, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    solve(input, 2)
}

fn solve(input: &Input, multiplier: u64) -> Output {
    let galaxies = expand(input, multiplier);
    let distances = calculate_distances(&galaxies);
    distances.iter()
        .map(|&GalaxyDistance { distance, .. }| distance)
        .sum()
}

fn solve2(input: &Input2) -> Output {
    solve(input, 1_000_000)
}


fn expand(input: &Input, space_multiplier: u64) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();
    let mut expand_y = Vec::new();
    let mut columns_with_galaxies = HashSet::new();
    for (y, line) in input.iter().enumerate() {
        let mut has_galaxy = false;
        for (x, &cell) in line.iter().enumerate() {
            match cell {
                Cell::Galaxy => {
                    has_galaxy = true;
                    columns_with_galaxies.insert(x);
                    galaxies.push(Galaxy::new(galaxies.len() as u32 + 1, Location::new(x as u64, y as u64)));
                }
                Cell::Space => (),
            }
        }
        if !has_galaxy {
            expand_y.push(y as u64);
        }
    }
    let expand_x = (0..input.len()).into_iter()
        .filter(|i| !columns_with_galaxies.contains(i))
        .map(|v| v as u64)
        .collect::<Vec<_>>();

    // -1 as we already have x1 expansion by default
    let space_multiplier = space_multiplier - 1;
    for galaxy in galaxies.iter_mut() {
        let index = galaxy.index;
        let expand_x_times = expand_x.iter().take_while(|v| **v < index.x).count() as u64;
        let expand_y_times = expand_y.iter().take_while(|v| **v < index.y).count() as u64;
        let index = Location::new(
            index.x + expand_x_times * space_multiplier,
            index.y + expand_y_times * space_multiplier,
        );
        galaxy.index = index;
    }
    galaxies
}

fn calculate_distances(galaxies: &[Galaxy]) -> Vec<GalaxyDistance> {
    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxy1 = galaxies[i];
            let galaxy2 = galaxies[j];
            let distance = galaxy1.distance(&galaxy2);
            distances.push(GalaxyDistance::new(galaxy1, galaxy2, distance));
        }
    }
    distances
}

#[derive(Debug, Copy, Clone, new)]
struct GalaxyDistance {
    galaxy1: Galaxy,
    galaxy2: Galaxy,
    distance: u64,
}

impl Display for GalaxyDistance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} = {}", self.galaxy1, self.galaxy2, self.distance)
    }
}

#[derive(Debug, Copy, Clone, new, Eq, PartialEq)]
struct Galaxy {
    id: u32,
    index: Location,
}

impl Display for Galaxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}", self.id, self.index)
    }
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> u64 {
        self.index.distance(&other.index)
    }
}


#[derive(Debug, Copy, Clone, new, Eq, PartialEq)]
struct Location {
    x: u64,
    y: u64,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Location {
    fn distance(&self, other: &Location) -> u64 {
        let dx = (self.x as i64 - other.x as i64).abs() as u64;
        let dy = (self.y as i64 - other.y as i64).abs() as u64;
        dx + dy
    }
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Galaxy,
    Space,
}

mod parser {
    use std::str::FromStr;

    use nom::{IResult, Parser};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::{opt, value};
    use nom::multi::many1;
    use nom::sequence::terminated;

    use crate::{Cell, Input, Input2};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        many1(terminated(many1(cell), opt(line_ending)))(input)
    }

    fn cell(input: &str) -> IResult<&str, Cell> {
        alt((
            value(Cell::Galaxy, tag("#")),
            value(Cell::Space, tag(".")),
        ))(input)
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
    use crate::solve;
    use crate::parser::parse;

    #[test]
    fn check_demo_1() {
        let input = include_str!("day11/demo");
        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve(&output, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn check_demo_2() {
        let input = include_str!("day11/demo");
        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve(&output, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn check_demo_3() {
        let input = include_str!("day11/demo");
        let (input, output) = parse(input).unwrap();
        assert!(input.is_empty());
        let result = solve(&output, 100);
        assert_eq!(result, 8410);
    }
}



