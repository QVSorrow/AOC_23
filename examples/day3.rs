use std::collections::HashSet;

use advent_of_code_2023::{execute, Type};

fn main() {
    execute(3, Type::Demo, parser::parse, |values| solve1(values));
    execute(3, Type::Task1, parser::parse, |values| solve1(values));
    execute(3, Type::Task2, parser::parse, |values| solve2(values));
}

fn solve1(engine: &Engine) -> u32 {
    let positions: PositionSet = engine.symbols.iter()
        .flat_map(|symbol| symbol.position.nearest())
        .collect::<HashSet<_>>()
        .into();

    engine.numbers.iter()
        .filter(|number| number.positions.intersects(&positions))
        .map(|number| number.value)
        .sum()
}

fn solve2(engine: &Engine) -> u32 {
    engine.symbols.iter()
        .filter(|s| s.name == '*')
        .map(|s| PositionSet::from(s.position.nearest()))
        .map(|nearest| engine.numbers.iter().filter(|num| num.positions.intersects(&nearest)).map(|n| n.value).collect::<Vec<_>>())
        .filter(|numbers| numbers.len() > 1)
        .map(|numbers| numbers.into_iter().product::<u32>())
        .sum()
}

#[derive(Debug, Clone)]
struct Engine {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    row: u32,
    column: u32,
}

impl Position {
    fn new(row: impl Into<u32>, column: impl Into<u32>) -> Self {
        let row = row.into();
        let column = column.into();
        Self { row, column }
    }

    fn nearest(&self) -> HashSet<Position> {
        let &Position { row, column } = self;
        let mut set = HashSet::new();
        set.insert(Position::new(row - 1, column - 1));
        set.insert(Position::new(row - 1, column));
        set.insert(Position::new(row - 1, column + 1));
        set.insert(Position::new(row, column - 1));
        set.insert(Position::new(row, column + 1));
        set.insert(Position::new(row + 1, column - 1));
        set.insert(Position::new(row + 1, column));
        set.insert(Position::new(row + 1, column + 1));
        set
    }
}

#[derive(Debug, Clone, Default)]
struct PositionSet {
    values: HashSet<Position>,
}

impl From<HashSet<Position>> for PositionSet {
    fn from(value: HashSet<Position>) -> Self {
        Self { values: value }
    }
}

impl PositionSet {
    fn add(&mut self, position: Position) -> bool {
        self.values.insert(position)
    }

    fn contains(&self, position: &Position) -> bool {
        self.values.contains(position)
    }

    fn intersects(&self, other: &PositionSet) -> bool {
        self.values.intersection(&other.values).count() > 0
    }
}


#[derive(Debug, Clone)]
struct Symbol {
    name: char,
    position: Position,
}


#[derive(Debug, Clone)]
struct Number {
    positions: PositionSet,
    value: u32,
}

impl Number {
    fn new(value: impl Into<u32>, row: impl Into<u32>, column: impl Into<u32>, len: impl Into<u32>) -> Self {
        let row = row.into();
        let column = column.into();
        let len = len.into();
        let value = value.into();
        let mut positions = PositionSet::default();
        for col in column..(column + len) {
            positions.add(Position::new(row, col));
        }
        Self { positions, value }
    }
}

mod parser {
    use nom::{IResult, Parser};
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take};
    use nom::character::complete::{digit1, line_ending};
    use nom::combinator::{map, map_res, opt, verify};
    use nom::error::context;
    use nom::multi::many1;
    use nom::sequence::terminated;

    use crate::{Engine, Number, Position, Symbol};

    pub(crate) fn parse(input: &str) -> IResult<&str, Engine> {
        let parser = many1(terminated(parse_line, opt(line_ending)));
        let (input, lines) = context("parse", parser)(input)?;
        let mut symbols = Vec::new();
        let mut numbers = Vec::new();
        for (row_index, row) in lines.into_iter().enumerate() {
            let row_index = row_index as u32;
            let mut column = 0u32;
            for t in row {
                match t {
                    Type::Period => column += 1,
                    Type::Symbol(name) => {
                        symbols.push(Symbol { name, position: Position::new(row_index, column) });
                        column += 1;
                    }
                    Type::Number { value, size } => {
                        numbers.push(Number::new(value, row_index, column, size as u32));
                        column += size as u32;
                    }
                }
            }
        }
        let engine = Engine { symbols, numbers };
        Ok((input, engine))
    }

    fn parse_period(input: &str) -> IResult<&str, Type> {
        map(tag("."), |_| Type::Period)(input)
    }

    fn parse_symbol(input: &str) -> IResult<&str, Type> {
        let verified = verify(take(1usize), |s: &str| {
            let c: char = s.chars().next().unwrap();
            !c.is_alphanumeric() && c != '.' && c != '\n'
        });
        map(verified, |s: &str| Type::Symbol(s.chars().next().unwrap()))(input)
    }

    fn parse_number(input: &str) -> IResult<&str, Type> {
        map_res(digit1, |s: &str| {
            s.parse::<u32>()
                .map(|num| Type::Number { value: num, size: s.len() })
        })(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Vec<Type>> {
        many1(alt((parse_symbol, parse_number, parse_period)))(input)
    }


    #[derive(Copy, Clone, Debug)]
    enum Type {
        Period,
        Symbol(char),
        Number {
            value: u32,
            size: usize,
        },
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
    use crate::solve2;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn check_v2() {
        let (_, engine) = parse(INPUT).unwrap();
        let result = solve2(&engine);
        assert_eq!(result, 467835);
    }
}

