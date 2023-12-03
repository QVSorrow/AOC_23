use advent_of_code_2023::{execute, Type};

fn main() {
    execute(4, Type::Demo, parser::parse, |values| solve1(values));
    // execute(4, Type::Task1, parser::parse, |values| solve1(values));
    // execute(4, Type::Task2, parser::parse, |values| solve2(values));
}

fn solve1(input: &str) -> u32 {
    todo!()
}

fn solve2(input: &str) -> u32 {
    todo!()
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

    pub(crate) fn parse(input: &str) -> IResult<&str, &str> {
        todo!()
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
    fn check() {
    }
}

