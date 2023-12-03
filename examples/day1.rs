use advent_of_code_2023::{execute, Type};

fn main() {
    execute(1, Type::Task2, parser::parse, |values| solve1(values));
}

fn solve1(values: &[u32]) -> u32 {
    values.iter().sum()
}


mod parser {
    use nom::{InputTake, IResult, Parser};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::bytes::complete::take;
    use nom::character::complete::line_ending;
    use nom::combinator::{eof, map, map_opt, opt, peek, verify};
    use nom::multi::{many1, many_till};
    use nom::sequence::{delimited, terminated};

    pub(crate) fn parse(input: &str) -> IResult<&str, Vec<u32>> {
        many1(terminated(parse_line, opt(alt((line_ending, eof))))).parse(input)
    }

    fn named_digit(input: &str) -> IResult<&str, &str> {
        map(
            alt((
                tag("one"),
                tag("two"),
                tag("three"),
                tag("four"),
                tag("five"),
                tag("six"),
                tag("seven"),
                tag("eight"),
                tag("nine"),
            )),
            |str| {
                match str {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => unreachable!(),
                }
            },
        ).parse(input)
    }

    fn num_digit(input: &str) -> IResult<&str, &str> {
        verify(take(1usize), |str: &str| {
            let next = str.chars().next();
            if let Some(c) = next {
                c.is_ascii_digit()
            } else {
                false
            }
        }).parse(input)
    }

    fn not_digit(input: &str) -> IResult<&str, &str> {
        let parser = many_till(take(1usize), alt((peek(line_ending), peek(digit))));
        map(parser, |(_, str)| str).parse(input)
    }

    fn digit(input: &str) -> IResult<&str, &str> {
        alt((named_digit, num_digit)).parse(input)
    }

    fn parse_line(input: &str) -> IResult<&str, u32> {
        let char = delimited(opt(not_digit), digit, opt(not_digit));
        map_opt(
            many1(char),
            |results| {
                println!("{results:?}");
                let c1 = results.first()?.chars().next()?;
                let c2 = results.last()?.chars().last()?;
                let mut str = String::with_capacity(2);
                str.push(c1);
                str.push(c2);
                str.parse().ok()
            },
        ).parse(input)
    }

    pub(crate) fn parse_v2(input: &str) -> IResult<&str, Vec<u32>> {
        let vec = input.lines()
            .map(|mut line| {
                let mut digits = Vec::new();
                while !line.is_empty() {
                    if let Ok((_, output)) = alt((num_digit, named_digit)).parse(line) {
                        digits.push(output);
                    }
                    line = &line[1..];
                }
                let mut s = String::new();
                s.push_str(digits.first().unwrap());
                s.push_str(digits.last().unwrap());
                s.parse().unwrap()
            })
            .collect();
        Ok(("", vec))
    }

    #[cfg(test)]
    mod tests {
        use crate::parser::{digit, not_digit};

        #[test]
        fn check_line() {
            let input = "eightwothree";
            let (input, _) = not_digit(input).unwrap();
            println!("{input}");
            let (input, num) = digit(input).unwrap();
            println!("{input} : {num}");
            let (input, _) = not_digit(input).unwrap();
            println!("{input}");
            let (input, num) = digit(input).unwrap();
            println!("{input} : {num}");
        }
    }
}