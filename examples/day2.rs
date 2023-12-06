use advent_of_code_2023::{execute, Type};

fn main() {
    execute(2, Type::Demo, parser::parse, |values| solve1(values));
    execute(2, Type::Task1, parser::parse, |values| solve1(values));
    execute(2, Type::Task2, parser::parse, |values| solve2(values));
}

fn solve1(values: &[Game]) -> u32 {
    let expected_set = CubeSet::new(12, 13, 14);

    values.iter()
        .filter(|game| game.is_possible_with(&expected_set))
        .map(|game| game.number)
        .sum()
}

fn solve2(value: &[Game]) -> u32 {
    value.iter()
        .map(|game| game.min_set().power())
        .sum()
}


#[derive(Debug, Clone)]
struct Game {
    number: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn from_sets(number: u32, sets: Vec<CubeSet>) -> Self {
        Self { number, sets }
    }

    fn is_possible_with(&self, set: &CubeSet) -> bool {
        self.sets.iter().all(|s| set.contains(s))
    }

    fn min_set(&self) -> CubeSet {
        let red = self.sets.iter().map(|s| s.red).max().unwrap_or(0);
        let green = self.sets.iter().map(|s| s.green).max().unwrap_or(0);
        let blue = self.sets.iter().map(|s| s.blue).max().unwrap_or(0);
        CubeSet::new(red, green, blue)
    }
}

#[derive(Debug, Copy, Clone)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn contains(&self, other: &CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

mod parser {
    use nom::{IResult, Parser};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::{map, opt};
    use nom::error::context;
    use nom::multi::{many1, separated_list0, separated_list1};
    use nom::sequence::{delimited, separated_pair, terminated};

    use advent_of_code_2023::decimal_legacy;

    use crate::{CubeSet, Game};

    pub(crate) fn parse(input: &str) -> IResult<&str, Vec<Game>> {
        let parser = many1(terminated(parse_game, opt(line_ending)));
        context("parse", parser)(input)
    }

    fn parse_game(input: &str) -> IResult<&str, Game> {
        let (input, number) = delimited(tag("Game "), decimal_legacy, tag(": "))(input)?;
        let (input, list) = context("parse_game", separated_list0(tag("; "), parse_set))(input)?;
        let game = Game::from_sets(number, list);
        Ok((input, game))
    }

    fn parse_set(input: &str) -> IResult<&str, CubeSet> {
        let red = map(parse_cubes("red"), Color::Red);
        let green = map(parse_cubes("green"), Color::Green);
        let blue = map(parse_cubes("blue"), Color::Blue);
        let (input, list) = context("parse_set", separated_list1(tag(", "), alt((red, green, blue)))).parse(input)?;
        let mut red = 0u32;
        let mut green = 0u32;
        let mut blue = 0u32;
        for color in list {
            match color {
                Color::Red(n) => red += n,
                Color::Green(n) => green += n,
                Color::Blue(n) => blue += n,
            }
        }
        let set = CubeSet::new(red, green, blue);
        Ok((input, set))
    }

    fn parse_cubes(name: &str) -> impl for<'a> Fn(&'a str) -> IResult<&'a str, u32> + '_ {
        move |input| map(separated_pair(decimal_legacy, tag(" "), tag(name)), |(n, _)| n).parse(input)
    }


    enum Color {
        Red(u32),
        Green(u32),
        Blue(u32),
    }

    #[cfg(test)]
    mod tests {
        use crate::parser::{parse, parse_game};

        #[test]
        fn check_line() {
            let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
            let (input, game) = parse_game(input).expect("game is parsed");
            println!("{game:?}");
            println!("{input}");
        }

        #[test]
        fn check_all() {
            let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
            let (input, games) = parse(input).expect("games is parsed");
            println!("{games:?}");
            println!("{input}");
        }
    }
}