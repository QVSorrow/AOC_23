use std::str::FromStr;

use rayon::prelude::*;

use advent_of_code_2023::{execute, Type};

type Input = u32;
type Input2 = Input;
type Output = u32;

const DAY: u8 = 0;

fn main() {
    execute(DAY, Type::Demo, parser::parse, |values| solve1(values));
    execute(DAY, Type::Task1, parser::parse, |values| solve1(values));
    execute(DAY, Type::Demo, parser::parse2, |values| solve2(values));
    execute(DAY, Type::Task2, parser::parse2, |values| solve2(values));
}

fn solve1(input: &Input) -> Output {
    todo!("⚠️ Solution 1 🤦‍")
}

fn solve2(input: &Input2) -> Output {
    todo!("⚠️ Solution 2 🤦‍")
}

mod parser {
    use std::str::FromStr;

    use nom::{IResult, Parser};

    use crate::{Input, Input2};

    pub(crate) fn parse(input: &str) -> IResult<&str, Input> {
        todo!("⚠️ Parser is not implemented 🤦‍")
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
    #[test]
    fn check() {}
}



